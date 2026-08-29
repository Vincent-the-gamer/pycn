//! 静态 Python 模式下的标准库引导。
//!
//! 编译期不处理标准库；运行时若在可执行文件周边找不到 `python-stdlib/`，
//! 就自动从 python-build-standalone 下载并解压标准库（与构建时使用的产物
//! 完全一致），让 pycn 开箱即用，无需用户手动配置。

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// 与构建脚本（build.rs）保持一致的版本号
const PYTHON_VERSION: &str = "3.12.13";
const PBS_RELEASE: &str = "20260718";
const PBS_BASE_URL: &str = "https://github.com/astral-sh/python-build-standalone/releases/download";

/// 归档内标准库的路径前缀（unix 布局: python/lib/python3.12/）
#[cfg(not(windows))]
const ARCHIVE_STDLIB_PREFIX: &str = "python/lib/python3.12";
/// Windows 布局: python/Lib/
#[cfg(windows)]
const ARCHIVE_STDLIB_PREFIX: &str = "python/Lib";
/// Windows 扩展模块目录
#[cfg(windows)]
const ARCHIVE_DLLS_PREFIX: &str = "python/DLLs";

const STDLIB_DIR: &str = "python-stdlib";

/// 在 exe_dir 周边查找已有的标准库；找不到则自动下载安装。
///
/// 返回 `python-stdlib/` 目录路径（其中包含 `python3.12/`）。
pub fn ensure_stdlib(exe_dir: &Path) -> Result<PathBuf, String> {
    if let Some(found) = find_stdlib(exe_dir) {
        return Ok(found);
    }

    let dest = install_root_for(exe_dir).join(STDLIB_DIR);
    fs::create_dir_all(&dest).map_err(|e| format!("无法创建目录 {}: {e}", dest.display()))?;

    eprintln!("\n[pycn] 未找到 Python 标准库，首次运行将自动下载（约 25 MB，仅需一次）...");
    match download_and_extract(&dest) {
        Ok(()) => {
            eprintln!("[pycn] Python 标准库已就绪: {}", dest.display());
            Ok(dest)
        }
        Err(err) => {
            let _ = fs::remove_dir_all(&dest);
            Err(format!(
                "{err}\n\
                 请检查网络连接后重试。\n\
                 （标准库应位于 {}/ 目录下）",
                dest.display()
            ))
        }
    }
}

/// 按优先级查找已有的标准库目录（与旧版 setup_python_home 的搜索顺序一致）
fn find_stdlib(exe_dir: &Path) -> Option<PathBuf> {
    let candidates: Vec<PathBuf> = vec![
        exe_dir.join(STDLIB_DIR),                       // 与二进制同目录
        exe_dir.join("..").join("..").join(STDLIB_DIR), // 项目根目录（开发时）
        exe_dir.join("..").join(STDLIB_DIR),            // 上级目录
        exe_dir.join("lib"),
        exe_dir.join("..").join("lib"),
    ];

    for candidate in candidates {
        if let Ok(entries) = fs::read_dir(&candidate) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if entry.path().is_dir() && (name_str.starts_with("python3.") || name_str == "Lib")
                {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

/// 选择标准库的安装位置：
/// - 开发构建（二进制位于 target/ 下）→ 项目根目录，避免 cargo clean 后丢失
/// - 发布包 → 与二进制同目录
fn install_root_for(exe_dir: &Path) -> PathBuf {
    let in_target = exe_dir
        .parent()
        .and_then(|p| p.file_name())
        .map(|n| n == "target")
        .unwrap_or(false);
    if in_target {
        exe_dir.join("..").join("..")
    } else {
        exe_dir.to_path_buf()
    }
}

/// 与 scripts/build-release.sh 相同的平台标识
fn platform_tags() -> Result<(&'static str, &'static str), String> {
    let os = match std::env::consts::OS {
        "linux" => "unknown-linux-gnu",
        "macos" => "apple-darwin",
        "windows" => "pc-windows-msvc",
        other => return Err(format!("不支持的操作系统: {other}")),
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" | "amd64" => "x86_64",
        "aarch64" | "arm64" => "aarch64",
        other => return Err(format!("不支持的 CPU 架构: {other}")),
    };
    Ok((os, arch))
}

fn download_and_extract(dest: &Path) -> Result<(), String> {
    let (os_tag, arch_tag) = platform_tags()?;
    let archive_name =
        format!("cpython-{PYTHON_VERSION}+{PBS_RELEASE}-{arch_tag}-{os_tag}-install_only.tar.gz");
    let url = format!("{PBS_BASE_URL}/{PBS_RELEASE}/{archive_name}");

    eprintln!("[pycn] 下载: {url}");

    let tmp_archive = dest.join(format!(".{archive_name}.part"));
    if let Err(err) = download(&url, &tmp_archive) {
        let _ = fs::remove_file(&tmp_archive);
        return Err(format!("下载 Python 标准库失败: {err}"));
    }

    let result = extract_stdlib(&tmp_archive, dest);
    let _ = fs::remove_file(&tmp_archive);
    result?;

    cleanup_stdlib(dest);
    Ok(())
}

fn download(url: &str, dest: &Path) -> Result<(), String> {
    let agent = ureq::AgentBuilder::new()
        .try_proxy_from_env(true) // 尊重 HTTP(S)_PROXY 等代理环境变量
        .timeout_connect(Duration::from_secs(30))
        .timeout_read(Duration::from_secs(120))
        .build();

    let resp = agent
        .get(url)
        .call()
        .map_err(|e| format!("HTTP 请求失败: {e}"))?;

    let total = resp
        .header("Content-Length")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let mut reader = resp.into_reader();
    let mut file = fs::File::create(dest).map_err(|e| format!("创建临时文件失败: {e}"))?;

    let mut buf = [0u8; 64 * 1024];
    let mut written: u64 = 0;
    let mut reported_mb: u64 = 0;
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| format!("读取下载数据失败: {e}"))?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n])
            .map_err(|e| format!("写入临时文件失败: {e}"))?;
        written += n as u64;
        let mb = written / (1024 * 1024);
        if mb != reported_mb {
            reported_mb = mb;
            if total > 0 {
                eprintln!("[pycn] 下载进度: {mb} MB / {} MB", total / (1024 * 1024));
            } else {
                eprintln!("[pycn] 下载进度: {mb} MB");
            }
        }
    }
    file.flush().map_err(|e| format!("写入临时文件失败: {e}"))?;
    Ok(())
}

/// 从归档中解压标准库内容。
/// unix:  python/lib/python3.12/* → dest/python3.12/（与 build-release.sh 一致）
/// windows: python/Lib/* 与 python/DLLs/* → dest/
fn extract_stdlib(archive_path: &Path, dest: &Path) -> Result<(), String> {
    let file = fs::File::open(archive_path).map_err(|e| format!("打开下载文件失败: {e}"))?;
    let gz = flate2::read::GzDecoder::new(file);
    let mut ar = tar::Archive::new(gz);

    #[cfg(windows)]
    let target_dir = dest.to_path_buf();
    #[cfg(not(windows))]
    let target_dir = dest.join("python3.12");

    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("创建目录 {} 失败: {e}", target_dir.display()))?;

    let mut entries = ar.entries().map_err(|e| format!("读取归档失败: {e}"))?;
    while let Some(entry) = entries.next() {
        let mut entry = entry.map_err(|e| format!("读取归档项失败: {e}"))?;
        let path = entry
            .path()
            .map_err(|e| format!("解析归档路径失败: {e}"))?
            .into_owned();

        let Some(rel) = strip_stdlib_prefix(&path) else {
            continue; // 归档内其它内容（bin/、include/、lib/ 等）不提取
        };

        let out = target_dir.join(&rel);
        let kind = entry.header().entry_type();

        if kind.is_dir() {
            fs::create_dir_all(&out)
                .map_err(|e| format!("创建目录 {} 失败: {e}", out.display()))?;
        } else if kind.is_file() {
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("创建目录 {} 失败: {e}", parent.display()))?;
            }
            let mut f = fs::File::create(&out)
                .map_err(|e| format!("创建文件 {} 失败: {e}", out.display()))?;
            io::copy(&mut entry, &mut f)
                .map_err(|e| format!("解压文件 {} 失败: {e}", out.display()))?;
        } else if kind.is_symlink() {
            #[cfg(unix)]
            {
                if let Ok(Some(target)) = entry.link_name() {
                    if let Some(parent) = out.parent() {
                        fs::create_dir_all(parent).ok();
                    }
                    let _ = fs::remove_file(&out);
                    std::os::unix::fs::symlink(target, &out)
                        .map_err(|e| format!("创建符号链接 {} 失败: {e}", out.display()))?;
                }
            }
        }
        // 其它条目类型（设备文件等）忽略
    }
    Ok(())
}

/// 返回归档路径中标准库部分的相对路径；非标准库内容返回 None
fn strip_stdlib_prefix(path: &Path) -> Option<PathBuf> {
    let prefix = Path::new(ARCHIVE_STDLIB_PREFIX);
    if let Ok(rel) = path.strip_prefix(prefix) {
        return Some(rel.to_path_buf());
    }
    #[cfg(windows)]
    {
        let dlls = Path::new(ARCHIVE_DLLS_PREFIX);
        if let Ok(rel) = path.strip_prefix(dlls) {
            return Some(rel.to_path_buf());
        }
    }
    None
}

/// 清理不必要的目录与 .pyc/.pyo 文件（与 build-release.sh 一致）
fn cleanup_stdlib(dest: &Path) {
    #[cfg(windows)]
    let stdlib_dir = dest.to_path_buf();
    #[cfg(not(windows))]
    let stdlib_dir = dest.join("python3.12");

    for name in [
        "test",
        "tkinter",
        "idlelib",
        "ensurepip",
        "lib2to3",
        "distutils",
        "site-packages",
        "turtledemo",
        "__pycache__",
    ] {
        let _ = fs::remove_dir_all(stdlib_dir.join(name));
    }
    remove_compiled_files(&stdlib_dir);
}

fn remove_compiled_files(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if entry.file_name() == "__pycache__" {
                let _ = fs::remove_dir_all(&path);
            } else {
                remove_compiled_files(&path);
            }
        } else if let Some(ext) = path.extension() {
            if ext == "pyc" || ext == "pyo" {
                let _ = fs::remove_file(&path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_tags_are_known() {
        let (os, arch) = platform_tags().expect("当前平台应受支持");
        assert!(!os.is_empty() && !arch.is_empty());
    }

    #[test]
    fn strip_prefix_works() {
        let rel = strip_stdlib_prefix(Path::new("python/lib/python3.12/os.py"))
            .expect("应匹配标准库前缀");
        assert_eq!(rel, Path::new("os.py"));
        assert!(strip_stdlib_prefix(Path::new("python/bin/python3")).is_none());
        assert!(strip_stdlib_prefix(Path::new("python/lib/libpython3.12.a")).is_none());
    }

    #[test]
    fn extract_and_cleanup_stdlib() {
        let dir = std::env::temp_dir().join(format!("pycn-bootstrap-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        // 手工构造与 python-build-standalone 同布局的归档
        let archive = dir.join("fake.tar.gz");
        let file = fs::File::create(&archive).unwrap();
        let gz = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut builder = tar::Builder::new(gz);

        let files: &[(&str, &[u8])] = &[
            ("python/lib/python3.12/os.py", b"import os\n"),
            ("python/lib/python3.12/lib-dynload/_ssl.so", b"\x7fELF-fake"),
            ("python/lib/python3.12/test/test_x.py", b"x"),
            (
                "python/lib/python3.12/__pycache__/os.cpython-312.pyc",
                b"pyc",
            ),
            ("python/lib/python3.12/site-packages/foo.py", b"foo"),
            ("python/bin/python3.12", b"bin"),
        ];
        for (path, data) in files {
            let mut header = tar::Header::new_gnu();
            header.set_size(data.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            builder.append_data(&mut header, path, &data[..]).unwrap();
        }
        builder.finish().unwrap();
        builder.into_inner().unwrap().finish().unwrap();

        extract_stdlib(&archive, &dir).unwrap();
        cleanup_stdlib(&dir);

        let stdlib = dir.join("python3.12");
        assert!(stdlib.join("os.py").exists(), "os.py 应被解压");
        assert!(
            stdlib.join("lib-dynload/_ssl.so").exists(),
            "lib-dynload 应被解压"
        );
        assert!(!stdlib.join("test").exists(), "test 应被清理");
        assert!(
            !stdlib.join("site-packages").exists(),
            "site-packages 应被清理"
        );
        assert!(!stdlib.join("__pycache__").exists(), "__pycache__ 应被清理");
        assert!(
            !dir.join("python/bin").exists(),
            "bin/ 不属于标准库，不应被解压"
        );

        let _ = fs::remove_dir_all(&dir);
    }
}
