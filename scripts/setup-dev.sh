#!/bin/bash
# ============================================================
# PyCN 开发环境一键配置
# 下载预编译的独立 Python（python-build-standalone），
# 使 pycn 完全脱离系统 Python 运行。
#
# 只需执行一次，后续 cargo build / cargo run 开箱即用。
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
CACHE_DIR="${PROJECT_DIR}/build"
CARGO_CONFIG="${PROJECT_DIR}/.cargo/config.toml"

PYTHON_VERSION="${PYTHON_VERSION:-3.12.13}"
PBS_RELEASE="${PBS_RELEASE:-20260718}"
PBS_BASE_URL="https://github.com/astral-sh/python-build-standalone/releases/download/${PBS_RELEASE}"

# ── 检测当前平台 ──────────────────────────────────────────
detect_platform() {
    local os arch
    case "$(uname -s)" in
        Linux)  os="unknown-linux-gnu" ;;
        Darwin) os="apple-darwin" ;;
        MINGW*|MSYS*|CYGWIN*) os="pc-windows-msvc" ;;
        *) echo "错误: 不支持的平台 $(uname -s)"; exit 1 ;;
    esac
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) echo "错误: 不支持的架构 $(uname -m)"; exit 1 ;;
    esac
    echo "${arch}-${os}"
}

PLATFORM=$(detect_platform)
PYTHON_HOME="${CACHE_DIR}/pbs-python"
ARCHIVE="cpython-${PYTHON_VERSION}+${PBS_RELEASE}-${PLATFORM}-install_only.tar.gz"

echo "=============================================="
echo " PyCN 开发环境配置"
echo " 平台:   ${PLATFORM}"
echo " Python: ${PYTHON_VERSION} (预编译)"
echo "=============================================="

# ── Step 1: 下载预编译的静态 Python ──────────────────────
if [ ! -f "${PYTHON_HOME}/.ok" ]; then
    echo ""
    echo "[1/4] 下载独立 Python 运行时..."
    mkdir -p "${PYTHON_HOME}"

    DOWNLOAD_URL="${PBS_BASE_URL}/${ARCHIVE}"
    echo "  URL: ${DOWNLOAD_URL}"

    if command -v curl &>/dev/null; then
        curl -fSL --retry 3 --progress-bar "${DOWNLOAD_URL}" -o "${CACHE_DIR}/pbs-archive.tar.gz"
    elif command -v wget &>/dev/null; then
        wget -q --show-progress "${DOWNLOAD_URL}" -O "${CACHE_DIR}/pbs-archive.tar.gz"
    else
        echo "错误: 需要 curl 或 wget 来下载文件"; exit 1
    fi

    echo "  解压..."
    tar -xzf "${CACHE_DIR}/pbs-archive.tar.gz" -C "${PYTHON_HOME}"
    rm -f "${CACHE_DIR}/pbs-archive.tar.gz"

    touch "${PYTHON_HOME}/.ok"
    echo "  ✅ 完成"
else
    echo ""
    echo "[1/4] 独立 Python 已缓存，跳过下载"
fi

# PBS 解压后可能是 python/ 子目录，统一处理
if [ -d "${PYTHON_HOME}/python" ]; then
    PYTHON_HOME="${PYTHON_HOME}/python"
fi

# ── Step 2: 生成 pyo3 配置 ────────────────────────────────
echo ""
echo "[2/4] 生成 pyo3 配置..."

# 定位 Python.h
INCLUDE_DIR=$(find "${PYTHON_HOME}" -name "Python.h" -type f -exec dirname {} \; 2>/dev/null | head -1)
if [ -z "${INCLUDE_DIR}" ]; then
    echo "错误: 找不到 Python.h，Python 安装可能损坏"
    echo "目录内容:"
    find "${PYTHON_HOME}" -type d | head -20
    exit 1
fi

# 定位库目录
case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*)
        LIB_DIR="${PYTHON_HOME}/libs"
        [ -d "$LIB_DIR" ] || LIB_DIR="${PYTHON_HOME}/lib"
        LIB_NAME="python312"
        ;;
    *)
        LIB_DIR="${PYTHON_HOME}/lib"
        LIB_NAME="python3.12"
        ;;
esac

PYO3_CONFIG="${CACHE_DIR}/pyo3-config.txt"
cat > "${PYO3_CONFIG}" << EOF
	implementation=CPython
	version=3.12
	shared=true
	abi3=false
	lib_name=${LIB_NAME}
	lib_dir=${LIB_DIR}
	includes=${INCLUDE_DIR}
	executable=${PYTHON_HOME}/bin/python3
	pointer_width=64
	build_flags=
	suppress_build_script_link_lines=false
	EOF

echo "  include: ${INCLUDE_DIR}"
echo "  lib:     ${LIB_DIR}"

# ── Step 3: 复制 stdlib 到项目根目录 ──────────────────────
echo ""
echo "[3/4] 复制 Python 标准库到项目根目录..."

STDLIB_SRC=$(find "${PYTHON_HOME}" -maxdepth 3 -type d -name "python3.*" 2>/dev/null | grep -v __pycache__ | head -1)
if [ -z "$STDLIB_SRC" ]; then
    STDLIB_SRC=$(find "${PYTHON_HOME}" -maxdepth 2 -type d -name "Lib" 2>/dev/null | head -1)
fi

if [ -z "$STDLIB_SRC" ]; then
    echo "  警告: 未找到 Python 标准库目录"
else
    STDLIB_DEST="${PROJECT_DIR}/python-stdlib/$(basename "${STDLIB_SRC}")"
    mkdir -p "${STDLIB_DEST}"
    cp -r "${STDLIB_SRC}"/* "${STDLIB_DEST}/"

    # 清理不必要文件以减小体积
    rm -rf "${STDLIB_DEST}/test" "${STDLIB_DEST}/tkinter" "${STDLIB_DEST}/idlelib" \
           "${STDLIB_DEST}/ensurepip" "${STDLIB_DEST}/lib2to3" "${STDLIB_DEST}/distutils" \
           "${STDLIB_DEST}/__pycache__" 2>/dev/null || true
    find "${STDLIB_DEST}" -name "*.pyc" -delete 2>/dev/null || true

    echo "  ✅ python-stdlib/$(basename "${STDLIB_SRC}")/"
fi

# ── Step 4: 写入 .cargo/config.toml ──────────────────────
echo ""
echo "[4/4] 写入 .cargo/config.toml（自动设置环境变量）..."

mkdir -p "$(dirname "${CARGO_CONFIG}")"

cat > "${CARGO_CONFIG}" << EOF
# 由 scripts/setup-dev.sh 自动生成，请勿手动编辑
# 删除此文件即可恢复为默认行为（需配合 --no-default-features）

[env]
PYO3_CONFIG_FILE = "${PYO3_CONFIG}"
PYCN_STATIC_PYTHON = "1"
EOF

echo "  ✅ ${CARGO_CONFIG}"

# ── Done ──────────────────────────────────────────────────
echo ""
echo "=============================================="
echo " ✅ 开发环境配置完成！"
echo "=============================================="
echo ""
echo "项目结构："
echo "  build/pbs-python/     ← 编译时链接的静态 Python"
echo "  python-stdlib/        ← 运行时加载的标准库"
echo "  .cargo/config.toml    ← 自动环境变量配置"
echo ""
echo "现在可以直接运行："
echo "  cargo build -p pycn --release"
echo "  cargo run -p pycn -- run examples/打印.pycn"
echo ""
echo "注意事项："
echo "  • cargo clean 不会删除 python-stdlib/（在项目根目录，不在 target/）"
echo "  • 若要恢复使用系统 Python：删除 .cargo/config.toml 并用 --no-default-features 构建"
echo ""
