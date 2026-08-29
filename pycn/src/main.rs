mod bootstrap;

use pycn::cli::use_cli;

fn main() {
    // 手动初始化 Python 解释器（替代 auto-initialize，兼容静态/动态链接）
    pyo3::prepare_freethreaded_python();

    // static-python 模式下，从二进制所在目录查找自带的 Python 标准库；
    // 找不到时自动下载（与 scripts/build-release.sh 相同的 python-build-standalone 产物）。
    // 若下载失败则直接报错退出，绝不回退到系统 Python
    if cfg!(feature = "static-python") {
        setup_python_home();
    }

    use_cli();
}

/// 设置 PYTHONHOME 环境变量，使静态链接的 Python 能找到自带的 stdlib。
/// 只在 `static-python` feature 启用时调用；找不到 stdlib 会自动下载，
/// 下载失败则退出，绝不回退到系统 Python。
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

    match bootstrap::ensure_stdlib(&exe_dir) {
        Ok(stdlib_root) => {
            // SAFETY: 在 main 函数开始时调用，此时尚未启动其他线程
            unsafe { std::env::set_var("PYTHONHOME", stdlib_root) };
            eprintln!("[pycn] 使用自带 Python 标准库");
        }
        Err(err) => {
            eprintln!("[pycn] 错误: {err}");
            std::process::exit(1);
        }
    }
}
