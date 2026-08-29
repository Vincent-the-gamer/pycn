<div align="center">
    <img src=".github/pycn-logo.png" style="height: 90px;"/>
    <h1>PyCN</h1>
    <b>用中文写Python代码, 图一乐～(∠・ω< )⌒★</b>
    <br/>
    <div>中文文档 | <a href="./README_en.md" target="_blank">English</a></div>
</div>

<br/>

# 预览

![preview](.github/preview.png)

# 使用文档

在这里查看PyCN使用文档：https://pycn.vincentthegamer.dpdns.org/zh_hans/

# 项目结构

```
pycn/
├── pycn/            # CLI 主程序（基于 pyo3，默认静态链接 Python）
├── pycn-dylib/      # C 动态库封装
├── parser/          # 核心解析器（logos 词法分析 + 手写递归下降解析）
├── parser-wasm/     # 解析器的 WASM 绑定（用于 Web / Node.js）
├── http-server/     # HTTP 代码执行服务（基于 Axum）
├── scripts/         # 构建脚本
│   └── build-release.sh  # 发布构建与打包
├── build/           # 构建缓存（首次构建自动下载 PBS Python）
├── python-stdlib/   # Python 标准库副本（首次运行时自动下载安装）
└── examples/        # 示例代码
```

# 安装

从 [GitHub Releases](https://github.com/Vincent-the-gamer/pycn/releases) 下载对应平台的预编译包，解压即可使用，无需安装 Python 或 Rust。

### 支持平台

| 平台    | 架构        |
|---------|------------|
| Linux   | x64, arm64 |
| macOS   | x64, arm64 |
| Windows | x64, arm64 |

下载解压后直接运行：

```shell
./pycn run examples/打印.pycn
```

# 构建（面向开发者）

## 前置要求

- [Rust](https://rustup.rs/) 工具链
- 网络连接（首次构建需下载预编译 Python）

## 开发构建（推荐）

无需任何配置，首次 `cargo build` 会自动下载预编译的静态 Python（约 25 MB）用于静态链接；Python 标准库则留到首次运行时按需下载：

```shell
# 编译 pycn（首次构建自动下载独立 Python）
cargo build -p pycn --release

# 运行示例
cargo run -p pycn --release -- run examples/打印.pycn
```

首次构建自动完成的工作：

1. 下载 [python-build-standalone](https://github.com/astral-sh/python-build-standalone) 预编译的独立 Python 到 `build/python-static/`
2. 静态链接 `libpython3.12`，生成不依赖系统 Python 的二进制

标准库不在编译期处理：首次运行 `pycn` 时若在二进制附近找不到 `python-stdlib/`，会自动下载（约 25 MB，仅需一次）。

> [!NOTE]
> - 若需使用系统 Python，运行 `cargo build --no-default-features` 构建
> - `cargo clean` 不会删除 `build/` 缓存（在项目根目录而非 `target/` 中）；`python-stdlib/` 由运行时自动生成

## 其他 crate 构建

```shell
# C 动态库
cargo build -p pycn-dylib --release

# Node.js / Web WASM
cd parser-wasm
wasm-pack build --out-dir output            # ES Module（默认 --target bundler）
wasm-pack build --target nodejs --out-dir output  # CommonJS
wasm-pack build --target web --out-dir output     # Web

# HTTP Server
cargo build -p http-server --release
```

## 发布构建（打包独立运行时）

```shell
bash scripts/build-release.sh
```

该脚本会编译 pycn，并将二进制与 Python 标准库打包到 `target/release/pycn-standalone/`，生成一个不依赖系统 Python 的独立分发包。

# 开源证书

[MIT 证书](./LICENSE.md)

版权所有 (c) 2025-现在 Vincent-the-gamer <https://github.com/Vincent-the-gamer>
