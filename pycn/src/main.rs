use pycn::cli::use_cli;

fn main() {
    // 手动初始化 Python 解释器（替代 auto-initialize，兼容静态/动态链接）
    pyo3::prepare_freethreaded_python();

    // static-python 模式下，从二进制所在目录查找自带的 Python 标准库
    // 若找不到则直接报错退出，绝不回退到系统 Python
    if cfg!(feature = "static-python") {
        setup_python_home();
    }

    use_cli();
}

/// 设置 PYTHONHOME 环境变量，使静态链接的 Python 能找到自带的 stdlib。
/// 只在 `static-python` feature 启用时调用；找不到 stdlib 会直接退出，
/// 绝不回退到系统 Python。
fn setup_python_home() {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[pycn] 错误: 无法获取当前可执行文件路径: {e}");
            std::process::exit(1);
        }
    };

    let exe_dir = match exe_path.parent() {
        Some(d) => d,
        None => {
            eprintln!("[pycn] 错误: 无法确定可执行文件所在目录");
            std::process::exit(1);
        }
    };

    // 按优先级搜索 python-stdlib/
    let candidates: Vec<std::path::PathBuf> = vec![
        exe_dir.join("python-stdlib"),                       // 与二进制同目录
        exe_dir.join("..").join("..").join("python-stdlib"), // 项目根目录（开发时）
        exe_dir.join("..").join("python-stdlib"),            // 上级目录
        exe_dir.join("lib"),
        exe_dir.join("..").join("lib"),
    ];

    for candidate in &candidates {
        if let Ok(entries) = std::fs::read_dir(candidate) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("python3.") && entry.path().is_dir() {
                    // SAFETY: 在 main 函数开始时调用，此时尚未启动其他线程
                    unsafe { std::env::set_var("PYTHONHOME", candidate) };
                    eprintln!("[pycn] 使用自带 Python 标准库: {}", candidate.display());
                    return;
                }
            }
        }
    }

    eprintln!(
        "\n\
         ================================================================\n\
         [pycn] 错误: 未找到 Python 标准库，且 static-python 模式下禁止回退到系统 Python。\n\
         \n\
         请确保 python-stdlib/ 目录与 pycn 二进制位于同一目录，\n\
         结构应为:\n\
           ├── pycn\n\
           └── python-stdlib/\n\
               └── python3.12/\n\
                   └── ...\n\
         \n\
         开发环境下请运行:  bash scripts/setup-dev.sh\n\
         ================================================================\n\
        "
    );
    std::process::exit(1);
}
