#!/bin/bash
# ============================================================
# PyCN 快速本地构建脚本（使用预编译的静态 Python）
# 无需从源码编译 CPython，下载即可用
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
CACHE_DIR="${PROJECT_DIR}/build"

# 配置
PYTHON_VERSION="3.12.13"
PBS_RELEASE="20260718"
PBS_BASE_URL="https://github.com/astral-sh/python-build-standalone/releases/download/${PBS_RELEASE}"

# 检测当前平台
detect_platform() {
    local os arch
    case "$(uname -s)" in
        Linux)  os="unknown-linux-gnu" ;;
        Darwin) os="apple-darwin" ;;
        MINGW*|MSYS*|CYGWIN*) os="pc-windows-msvc" ;;
        *) echo "未知平台: $(uname -s)"; exit 1 ;;
    esac
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) echo "未知架构: $(uname -m)"; exit 1 ;;
    esac
    echo "${arch}-${os}"
}

PLATFORM=$(detect_platform)
PYTHON_HOME="${CACHE_DIR}/python-static"
ARCHIVE="cpython-${PYTHON_VERSION}+${PBS_RELEASE}-${PLATFORM}-install_only.tar.gz"

echo "=============================================="
echo " PyCN - 独立运行时构建"
echo " 平台: ${PLATFORM}"
echo " Python: ${PYTHON_VERSION} (预编译)"
echo "=============================================="

# ---- Step 1: 下载预编译的静态 Python ----
if [ ! -f "${PYTHON_HOME}/.ok" ]; then
    echo ""
    echo "[1/4] 下载静态 Python..."
    mkdir -p "${PYTHON_HOME}"

    DOWNLOAD_URL="${PBS_BASE_URL}/${ARCHIVE}"
    echo "  下载: ${DOWNLOAD_URL}"

    curl -fSL --retry 3 --progress-bar "${DOWNLOAD_URL}" -o "${CACHE_DIR}/pbs-archive.tar.gz"

    echo "  解压..."
    tar -xzf "${CACHE_DIR}/pbs-archive.tar.gz" -C "${PYTHON_HOME}"
    rm -f "${CACHE_DIR}/pbs-archive.tar.gz"

    # python-build-standalone 解压后是 python/ 目录
    if [ -d "${PYTHON_HOME}/python" ]; then
        # 文件已经在 python/ 下，PYTHON_HOME 指向 python/
        # 这里保持 PYTHON_HOME 为外层，后续脚本会处理
        true
    fi

    touch "${PYTHON_HOME}/.ok"
    echo "  ✅ 静态 Python 就绪"
else
    echo ""
    echo "[1/4] 静态 Python 已缓存，跳过下载"
fi

# 确保 PYTHON_HOME 指向实际的 python 目录
if [ -d "${PYTHON_HOME}/python" ]; then
    PYTHON_HOME="${PYTHON_HOME}/python"
fi

# ---- Step 2: 生成 pyo3 配置 ----
echo ""
echo "[2/4] 生成 pyo3 配置..."

INCLUDE_DIR=$(find "${PYTHON_HOME}" -name "Python.h" -type f -exec dirname {} \; 2>/dev/null | head -1)

if [ "$(uname -s)" = "MINGW"* ] || [ "$(uname -s)" = "MSYS"* ] || [ "$(uname -s)" = "CYGWIN"* ]; then
    LIB_DIR="${PYTHON_HOME}/libs"
    LIB_NAME="python312"
    # fallback
    [ -d "$LIB_DIR" ] || LIB_DIR="${PYTHON_HOME}/lib"
else
    LIB_DIR="${PYTHON_HOME}/lib"
    LIB_NAME="python3.12"
fi

cat > "${CACHE_DIR}/pyo3-config.txt" << EOF
implementation=CPython
version=3.12
shared=false
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
echo "  lib:     ${LIB_DIR}/lib${LIB_NAME}.*"

# ---- Step 3: 编译 pycn ----
echo ""
echo "[3/4] 编译 pycn（静态链接 Python）..."

export PYO3_CONFIG_FILE="${CACHE_DIR}/pyo3-config.txt"
export PYCN_STATIC_PYTHON=1

cd "${PROJECT_DIR}"
cargo build -p pycn --release

# ---- Step 4: 打包产物 ----
echo ""
echo "[4/4] 打包发布产物..."

RELEASE_DIR="${PROJECT_DIR}/target/release"
PKG_DIR="${RELEASE_DIR}/pycn-standalone"
mkdir -p "${PKG_DIR}/python-stdlib"

# 复制二进制
if [ -f "${RELEASE_DIR}/pycn.exe" ]; then
    cp "${RELEASE_DIR}/pycn.exe" "${PKG_DIR}/"
else
    cp "${RELEASE_DIR}/pycn" "${PKG_DIR}/"
fi

# 复制标准库
STDLIB_SRC=$(find "${PYTHON_HOME}" -maxdepth 3 -type d -name "python3.*" 2>/dev/null | grep -v __pycache__ | head -1)
if [ -z "$STDLIB_SRC" ]; then
    STDLIB_SRC=$(find "${PYTHON_HOME}" -maxdepth 2 -type d -name "Lib" 2>/dev/null | head -1)
fi

if [ -n "$STDLIB_SRC" ]; then
    STDLIB_DEST="${PKG_DIR}/python-stdlib/$(basename ${STDLIB_SRC})"
    mkdir -p "${STDLIB_DEST}"
    cp -r "${STDLIB_SRC}"/* "${STDLIB_DEST}/"
    # 清理不必要文件
    rm -rf "${STDLIB_DEST}/test" "${STDLIB_DEST}/tkinter" "${STDLIB_DEST}/idlelib" \
           "${STDLIB_DEST}/ensurepip" "${STDLIB_DEST}/lib2to3" "${STDLIB_DEST}/distutils" \
           "${STDLIB_DEST}/__pycache__" 2>/dev/null || true
    find "${STDLIB_DEST}" -name "*.pyc" -delete 2>/dev/null || true
    echo "  标准库: ${STDLIB_DEST}"
fi

echo ""
echo "=============================================="
echo " ✅ 构建完成!"
echo "=============================================="
echo ""
echo "产物位置: ${PKG_DIR}/"
echo ""
echo "快速测试:"
echo "  cd ${PKG_DIR} && ./pycn run ${PROJECT_DIR}/examples/打印.pycn"
echo ""
