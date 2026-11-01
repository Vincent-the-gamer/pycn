# ============================================================
# PyCN Windows 本地发布构建
# 编译（首次构建会自动下载预编译的静态 Python，无需手动配置）
# 并把二进制与 Python 标准库打包为独立运行时目录。
# ============================================================
$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectDir = Split-Path -Parent $ScriptDir

Set-Location $ProjectDir

# 若未设置 PYO3_CONFIG_FILE，则尝试使用 build/python-static 缓存；否则提示
if (-not $env:PYO3_CONFIG_FILE) {
    $cache = Join-Path $ProjectDir "build\python-static"
    if (Test-Path $cache) {
        Write-Host "使用缓存的静态 Python: $cache"
    } else {
        Write-Host "未找到缓存的静态 Python，cargo build 将自动下载（需要网络）。"
    }
}

Write-Host "编译 pycn（首次构建会自动下载预编译的静态 Python）..."
cargo build -p pycn --release

$ReleaseDir = Join-Path $ProjectDir "target\release"
$PkgDir = Join-Path $ReleaseDir "pycn-standalone"
if (Test-Path $PkgDir) { Remove-Item -Recurse -Force $PkgDir }
New-Item -ItemType Directory -Force -Path "$PkgDir\python-stdlib\Lib" | Out-Null
New-Item -ItemType Directory -Force -Path "$PkgDir\python-stdlib\DLLs" | Out-Null

# 复制二进制
Copy-Item (Join-Path $ReleaseDir "pycn.exe") $PkgDir

# 复制 Python 动态库
$PyHome = $null
$pythonStatic = Join-Path $ProjectDir "build\python-static"
if (Test-Path $pythonStatic) {
    $PyHome = Get-ChildItem -Path $pythonStatic -Recurse -Filter "Python.h" | Select-Object -First 1 | ForEach-Object { $_.Directory.Parent.FullName }
}
if (-not $PyHome) {
    # 回退到系统 Python（仅用于构建/打包，运行时仍使用自带库）
    $PyHome = "C:\Users\$env:USERNAME\AppData\Local\Programs\Python\Python312"
}
Write-Host "Python home: $PyHome"

foreach ($dll in @("python3.dll","python312.dll","vcruntime140.dll","vcruntime140_1.dll")) {
    $src = Get-ChildItem -Path $PyHome -Recurse -Filter $dll -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($src) { Copy-Item $src.FullName $PkgDir -Force }
}

# 复制标准库
$LibSrc = Join-Path $PyHome "Lib"
if (-not (Test-Path $LibSrc)) {
    $LibSrc = Get-ChildItem -Path $PyHome -Recurse -Directory -Filter "Lib" | Select-Object -First 1 | ForEach-Object { $_.FullName }
}
if (Test-Path $LibSrc) {
    Copy-Item "$LibSrc\*" "$PkgDir\python-stdlib\Lib" -Recurse -Force
}
$DllsSrc = Join-Path $PyHome "DLLs"
if (-not (Test-Path $DllsSrc)) {
    $DllsSrc = Get-ChildItem -Path $PyHome -Recurse -Directory -Filter "DLLs" | Select-Object -First 1 | ForEach-Object { $_.FullName }
}
if (Test-Path $DllsSrc) {
    Copy-Item "$DllsSrc\*" "$PkgDir\python-stdlib\DLLs" -Recurse -Force
}

# 清理
foreach ($name in @("test","tkinter","idlelib","ensurepip","lib2to3","distutils","site-packages","turtledemo","__pycache__")) {
    Remove-Item -Recurse -Force (Join-Path "$PkgDir\python-stdlib\Lib" $name) -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force (Join-Path "$PkgDir\python-stdlib\DLLs" $name) -ErrorAction SilentlyContinue
}
Get-ChildItem -Path "$PkgDir\python-stdlib" -Recurse -Filter *.pyc -ErrorAction SilentlyContinue | Remove-Item -Force
Get-ChildItem -Path "$PkgDir\python-stdlib" -Recurse -Filter *.pyo -ErrorAction SilentlyContinue | Remove-Item -Force

Write-Host ""
Write-Host "=============================================="
Write-Host " 构建完成!"
Write-Host "=============================================="
Write-Host "产物位置: $PkgDir"
Write-Host "快速测试: cd $PkgDir; .\pycn.exe run ..\..\examples\打印.pycn"