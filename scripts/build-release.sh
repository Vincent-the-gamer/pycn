#!/bin/bash
# ============================================================
# PyCN 快速本地发布构建
# 编译（首次构建会自动下载预编译的静态 Python，无需手动配置）
# 并把二进制与 Python 标准库打包为独立运行时目录。
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "编译 pycn（首次构建会自动下载预编译的静态 Python）..."
cd "${PROJECT_DIR}"
cargo build -p pycn --release

RELEASE_DIR="${PROJECT_DIR}/target/release"
PKG_DIR="${RELEASE_DIR}/pycn-standalone"
mkdir -p "${PKG_DIR}/python-stdlib"

# ---- 复制二进制 ----
if [ -f "${RELEASE_DIR}/pycn.exe" ]; then
    cp "${RELEASE_DIR}/pycn.exe" "${PKG_DIR}/"
else
    cp "${RELEASE_DIR}/pycn" "${PKG_DIR}/"
fi

# ---- 复制标准库（来自构建缓存的 PBS Python；编译期不再安装到项目根目录） ----
STDLIB_SRC=$(find "${PROJECT_DIR}/build/python-static" -maxdepth 3 -type d -name "python3.*" ! -path "*/__pycache__/*" 2>/dev/null | head -1)

if [ -z "${STDLIB_SRC}" ]; then
    echo "::warning::Python stdlib not found in build cache"
else
    echo "Stdlib source: ${STDLIB_SRC}"
    STDLIB_DEST="${PKG_DIR}/python-stdlib/$(basename "${STDLIB_SRC}")"
    mkdir -p "${STDLIB_DEST}"

    cp -r "${STDLIB_SRC}"/* "${STDLIB_DEST}/" 2>/dev/null || true

    # 删除不需要的目录以减小体积（与 CI 发布流程一致）
    rm -rf "${STDLIB_DEST}/test" "${STDLIB_DEST}/tkinter" "${STDLIB_DEST}/idlelib" \
           "${STDLIB_DEST}/ensurepip" "${STDLIB_DEST}/lib2to3" "${STDLIB_DEST}/distutils" \
           "${STDLIB_DEST}/site-packages" "${STDLIB_DEST}/turtledemo" "${STDLIB_DEST}/__pycache__" 2>/dev/null || true

    # 删除 .pyc/.pyo 文件
    find "${STDLIB_DEST}" -name "*.pyc" -delete 2>/dev/null || true
    find "${STDLIB_DEST}" -name "*.pyo" -delete 2>/dev/null || true

    echo "Stdlib size: $(du -sh "${STDLIB_DEST}" | cut -f1)"
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
