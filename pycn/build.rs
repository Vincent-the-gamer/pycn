use std::env;

fn main() {
    // ── 静态链接模式（默认） ──
    if cfg!(feature = "static-python") {
        let pycn_static = env::var("PYCN_STATIC_PYTHON").is_ok();
        let config_file = env::var("PYO3_CONFIG_FILE");

        // 设置运行时库搜索路径（相对于可执行文件所在目录）
        add_rpath();

        if pycn_static && config_file.is_ok() {
            // CI / release 构建：PYO3_CONFIG_FILE 指向 python-build-standalone
            println!("cargo:warning=Building with static Python (PYO3_CONFIG_FILE)");
            return;
        }

        if config_file.is_ok() {
            // 用户手动设置了 PYO3_CONFIG_FILE
            println!("cargo:warning=Building with static Python (PYO3_CONFIG_FILE)");
            return;
        }

        // 静态链接模式下，必须设置环境变量
        eprintln!(
            "\n\x1b[1;31m错误：static-python 特性已启用，但未配置 Python 运行环境。\x1b[0m\n\n\
             运行以下命令下载预编译的独立 Python（只需执行一次）：\n\n  \
             \x1b[1;33mbash scripts/setup-dev.sh\x1b[0m\n\n\
             或者，如果你确实想链接系统 Python（不推荐，pycn 将依赖系统环境）：\n\n  \
             \x1b[2mcargo build --no-default-features\x1b[0m\n"
        );
        std::process::exit(1);
    }

    // ── 动态链接模式（--no-default-features） ──
    pyo3_build_config::add_python_framework_link_args();
}

/// 设置运行时动态库搜索路径。
///
/// - 若设置了 `PYCN_RPATH` 环境变量（本地开发场景），直接使用其值作为 rpath。
///   此时通常指向 python-build-standalone 的 lib 目录绝对路径。
/// - 否则使用相对于可执行文件的路径（CI / 发布场景）：
///   - Linux:   `$ORIGIN`            → 运行时在 pycn 同级目录查找 .so
///   - macOS:   `@executable_path`   → 运行时在 pycn 同级目录查找 .dylib
///   - Windows: 无需设置，exe 所在目录默认在 DLL 搜索路径中
fn add_rpath() {
    // 本地开发模式：使用 setup-dev.sh 生成 .cargo/config.toml 中设定的绝对路径
    if let Ok(custom_rpath) = env::var("PYCN_RPATH") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", custom_rpath);
        return;
    }

    // CI / 发布模式：使用相对路径，让 pycn 在自身目录查找 libpython
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        }
        "macos" => {
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        }
        "windows" => {
            // Windows 默认搜索 exe 所在目录的 DLL，无需额外 rpath
        }
        _ => {}
    }
}
