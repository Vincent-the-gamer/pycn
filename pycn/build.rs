//! pycn 构建脚本。
//!
//! `static-python`（默认）模式下，pyo3 使用 `abi3` + `extension-module`：
//! 编译期不需要任何 Python 解释器配置，libpython 的链接参数完全由本脚本负责。
//!
//! 本脚本按以下优先级提供静态 Python：
//! 1. 设置了 `PYO3_CONFIG_FILE`（CI / 手动指定）→ 解析其中的 `lib_dir` /
//!    `lib_name`，输出对应的链接参数，行为与原先一致（发布流程打包动态库 + rpath）；
//! 2. 否则自动下载 python-build-standalone 的预编译 Python（与原先
//!    `scripts/build-release.sh` 相同的产物），静态链接 `libpython3.12.a`。
//!    标准库不在编译期处理：首次运行时若在二进制附近找不到 `python-stdlib/`，
//!    由运行时（bootstrap.rs）自动下载。
//!
//! 若需改用系统 Python：`cargo build --no-default-features`。

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 与 scripts/build-release.sh 保持一致的版本号
const PYTHON_VERSION: &str = "3.12.13";
const PBS_RELEASE: &str = "20260718";
const PBS_BASE_URL: &str = "https://github.com/astral-sh/python-build-standalone/releases/download";

fn main() {
    if cfg!(feature = "static-python") {
        println!("cargo:rerun-if-env-changed=PYO3_CONFIG_FILE");
        if let Err(err) = setup_static_python() {
            eprintln!(
                "\n\x1b[1;31m错误：static-python 模式自动配置失败。\x1b[0m\n\n{err}\n\n\
                 如果你确实想链接系统 Python（不推荐，pycn 将依赖系统环境）：\n\n  \
                 \x1b[2mcargo build --no-default-features\x1b[0m\n"
            );
            std::process::exit(1);
        }
    } else {
        pyo3_build_config::add_python_framework_link_args();
    }
}

/// 配置静态 Python 的链接参数。
fn setup_static_python() -> Result<(), String> {
    // 1) 外部配置（CI / 手动）→ 按其 lib_dir / lib_name 输出链接参数
    if let Ok(config_file) = env::var("PYO3_CONFIG_FILE") {
        let (lib_dir, lib_name, shared) = parse_pyo3_config(Path::new(&config_file))
            .ok_or_else(|| format!("无法解析 PYO3_CONFIG_FILE: {config_file}"))?;
        emit_link(&lib_dir, &lib_name, shared, &[]);
        add_rpath();
        println!("cargo:warning=Building with static Python (external config: {config_file})");
        return Ok(());
    }

    // 2) 开发环境：自动下载预编译的静态 Python
    let python_home = ensure_cached_python()?;

    if is_windows() {
        let libs_dir = python_home.join("libs");
        let lib_dir = if libs_dir.is_dir() {
            libs_dir
        } else {
            python_home.join("lib")
        };
        // Windows 上 pyo3（abi3）自行链接 python3.dll / python312.lib，
        // 本脚本只补充库搜索路径；运行所需的 DLL 复制到产物目录。
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        copy_windows_dlls(&python_home);
    } else {
        // 标准库保留在 PBS 缓存中，运行时会按需自动下载；这里只查询链接参数
        let extra_libs = extra_system_libs_cached(&python_home)?;
        emit_link(&python_home.join("lib"), "python3.12", false, &extra_libs);
    }

    println!(
        "cargo:warning=Static Python auto-configured at {}",
        python_home.display()
    );
    Ok(())
}

/// 解析 pyo3 配置文件，返回 (lib_dir, lib_name, shared)。
fn parse_pyo3_config(path: &Path) -> Option<(PathBuf, String, bool)> {
    let content = fs::read_to_string(path).ok()?;
    let mut lib_dir = None;
    let mut lib_name = None;
    let mut shared = Some(true);
    for line in content.lines() {
        let mut it = line.splitn(2, '=');
        let (key, value) = (it.next()?.trim(), it.next()?.trim());
        match key {
            "lib_dir" => lib_dir = Some(PathBuf::from(value)),
            "lib_name" => lib_name = Some(value.to_string()),
            "shared" => shared = Some(value == "true"),
            _ => {}
        }
    }
    Some((lib_dir?, lib_name?, shared?))
}

/// 输出 libpython 的链接参数。
fn emit_link(lib_dir: &Path, lib_name: &str, shared: bool, extra_libs: &[String]) {
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    let model = if shared { "" } else { "static=" };
    println!("cargo:rustc-link-lib={model}{lib_name}");
    for lib in extra_libs {
        println!("cargo:rustc-link-lib={lib}");
    }
}

/// 定位或下载 python-build-standalone，返回 Python 安装根目录。
fn ensure_cached_python() -> Result<PathBuf, String> {
    let cache_root = project_root().join("build").join("python-static");
    let legacy = project_root().join("build").join("pbs-python");
    for dir in [&cache_root, &legacy] {
        if let Some(home) = cached_python_home(dir) {
            println!("cargo:warning=Using cached Python at {}", home.display());
            return Ok(home);
        }
    }

    let (os_tag, arch_tag) = platform_tags()?;
    let archive_name =
        format!("cpython-{PYTHON_VERSION}+{PBS_RELEASE}-{arch_tag}-{os_tag}-install_only.tar.gz");
    let url = format!("{PBS_BASE_URL}/{PBS_RELEASE}/{archive_name}");

    println!("cargo:warning=首次构建：自动下载预编译的静态 Python（约 25 MB）...");
    println!("cargo:warning=  {url}");

    fs::create_dir_all(&cache_root)
        .map_err(|e| format!("无法创建缓存目录 {}: {e}", cache_root.display()))?;

    let tmp_archive = cache_root.join(".download.tar.gz");
    let status = Command::new("curl")
        .args(["-fSL", "--retry", "3", "--progress-bar", &url, "-o"])
        .arg(&tmp_archive)
        .status()
        .map_err(|e| format!("无法执行 curl（需要 curl 命令）: {e}"))?;
    if !status.success() {
        let _ = fs::remove_file(&tmp_archive);
        return Err(format!("下载失败: {url}\n请检查网络连接后重试。"));
    }

    let status = Command::new("tar")
        .args(["-xzf"])
        .arg(&tmp_archive)
        .args(["-C", &cache_root.to_string_lossy()])
        .status()
        .map_err(|e| format!("无法执行 tar（需要 tar 命令）: {e}"))?;
    let _ = fs::remove_file(&tmp_archive);
    if !status.success() {
        return Err(format!("解压失败: {archive_name}"));
    }

    let home = cached_python_home(&cache_root)
        .ok_or_else(|| format!("解压后未找到 Python 安装目录: {}", cache_root.display()))?;
    fs::write(cache_root.join(".ok"), b"ok").map_err(|e| format!("写入缓存标记失败: {e}"))?;
    println!("cargo:warning=Python 解压完成: {}", home.display());
    Ok(home)
}

/// 校验缓存目录中是否有可用的 Python 安装（解压后是 python/ 子目录）。
fn cached_python_home(cache_dir: &Path) -> Option<PathBuf> {
    let inner = if cache_dir.join("python").is_dir() {
        cache_dir.join("python")
    } else {
        cache_dir.to_path_buf()
    };
    let has_header = inner.join("include").join("Python.h").exists()
        || inner
            .join("include")
            .join("python3.12")
            .join("Python.h")
            .exists();
    has_header.then_some(inner)
}

/// 从 PBS Python 的 sysconfig 获取静态链接 libpython 所需的额外系统库，
/// 结果缓存在 build/python-static/.extra-libs.txt，避免后续构建重复探测。
fn extra_system_libs_cached(python_home: &Path) -> Result<Vec<String>, String> {
    let cache_file = python_home
        .parent()
        .unwrap_or(python_home)
        .join(".extra-libs.txt");
    if let Ok(content) = fs::read_to_string(&cache_file) {
        let libs: Vec<String> = content
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        return Ok(libs);
    }

    let libs = probe_extra_system_libs(python_home)?;
    if !libs.is_empty() {
        let _ = fs::write(&cache_file, libs.join("\n"));
    }
    Ok(libs)
}

fn probe_extra_system_libs(python_home: &Path) -> Result<Vec<String>, String> {
    let python_exe = python_home.join("bin").join("python3");
    if !python_exe.exists() {
        return Ok(Vec::new());
    }
    let output = Command::new(&python_exe)
        .args([
            "-c",
            "import sysconfig as s; print((s.get_config_var('LIBS') or '') + ' ' + (s.get_config_var('SYSLIBS') or ''))",
        ])
        .output()
        .map_err(|e| format!("运行 {} 失败: {e}", python_exe.display()))?;
    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let tokens: Vec<&str> = stdout.split_whitespace().collect();
    let mut libs = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];
        if let Some(name) = token.strip_prefix("-l") {
            libs.push(name.to_string());
        } else if token == "-framework" && i + 1 < tokens.len() {
            libs.push(format!("framework={}", tokens[i + 1]));
            i += 1;
        }
        i += 1;
    }
    Ok(libs)
}

/// Windows: 把运行所需的 python3.dll / python312.dll 复制到产物目录（尽力而为）。
fn copy_windows_dlls(python_home: &Path) {
    let profile = env::var("PROFILE").unwrap_or_default();
    let target = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| project_root().join("target"));
    let dir = target.join(&profile);
    let _ = fs::create_dir_all(&dir);
    for name in ["python3.dll", "python312.dll"] {
        let src = python_home.join(name);
        if src.exists() {
            let _ = fs::copy(&src, dir.join(name));
        }
    }
}

/// 与 scripts/build-release.sh 相同的平台标识。
fn platform_tags() -> Result<(String, String), String> {
    let os = match env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("linux") => "unknown-linux-gnu",
        Ok("macos") => "apple-darwin",
        Ok("windows") => "pc-windows-msvc",
        other => return Err(format!("不支持的目标操作系统: {other:?}")),
    };
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86_64") | Ok("amd64") => "x86_64",
        Ok("aarch64") | Ok("arm64") => "aarch64",
        other => return Err(format!("不支持的目标架构: {other:?}")),
    };
    Ok((os.to_string(), arch.to_string()))
}

/// 设置运行时动态库搜索路径（外部配置路径使用，行为与原先一致）。
fn add_rpath() {
    if let Ok(custom_rpath) = env::var("PYCN_RPATH") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", custom_rpath);
        return;
    }
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "linux" => println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN"),
        "macos" => println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path"),
        _ => {}
    }
}

fn is_windows() -> bool {
    env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
}

fn project_root() -> PathBuf {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    Path::new(&manifest).join("..").join("..")
}
