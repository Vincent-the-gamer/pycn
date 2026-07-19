use std::{env, fs};

fn main() {
    // ── 静态链接模式（默认） ──
    if cfg!(feature = "static-python") {
        let pycn_static = env::var("PYCN_STATIC_PYTHON").is_ok();
        let config_file = env::var("PYO3_CONFIG_FILE");

        // 从 pyo3 配置文件中解析 lib_dir，添加 rpath
        // （因为 macOS 的 PBS 只提供 dylib，运行时需要用 rpath 定位）
        if let Ok(ref cfg_path) = config_file {
            if let Ok(content) = fs::read_to_string(cfg_path) {
                for line in content.lines() {
                    if let Some(dir) = line.strip_prefix("lib_dir=") {
                        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", dir);
                        break;
                    }
                }
            }
        }

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
