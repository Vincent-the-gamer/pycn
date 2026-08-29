---
layout: doc
title: "入坑简介"
lastUpdated: true
---

# 入坑简介

PyCN，顾名思义就是`Python Chinese`，它使用和Python相同的语法，但是关键词，字面量，内置函数和运算符均使用中文。

我们来看看这个例子：

```pycn
定义 是否是质数（被判断的数）：
    如果 被判断的数 小于 二：
        返回 假
    迭代 数一 在 范围（二，整数（被判断的数 取幂 零点五）加 一）：
        如果 被判断的数 取余 数一 等于 零：
            返回 假
    返回 真
```

上述代码相当于：

```python
def is_prime(num):
    if num < 2:
        return false
    for i in range(2, int(num ** 0.5) + 1):
        if num % i == 0:
            return false
    return true
```

没错！PyCN和Python的代码风格完全一致，然而如你所见，和我上面说的一样，代码都是中文的！

## 安装 PyCN

预编译的二进制文件已发布在 [GitHub Releases](https://github.com/Vincent-the-gamer/pycn/releases)。下载对应平台的压缩包，解压即可使用。

每个发布包已包含 `pycn` 二进制文件和 Python 标准库——无需安装系统 Python 或 Rust 工具链。

### 支持平台

| 平台   | 架构        |
|--------|------------|
| Linux  | x64, arm64 |
| macOS  | x64, arm64 |
| Windows| x64, arm64 |

下载解压后，直接运行：

```shell
./pycn run 你的文件.pycn
```

## 从源码构建（面向贡献者）

### 前置要求

- [Rust](https://rustup.rs/) 工具链
- 网络连接（首次构建需下载预编译 Python）

### 开发构建（推荐）

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

### 其他 crate 构建

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

### 发布构建（打包独立运行时）

```shell
bash scripts/build-release.sh
```

该脚本会编译 pycn，并将二进制与 Python 标准库打包到 `target/release/pycn-standalone/`，生成一个不依赖系统 Python 的独立分发包。

## 语法高亮

目前，PyCN在VSCode编辑器中支持**语法高亮**。

只需要安装[`Pycn`](https://marketplace.visualstudio.com/items?itemName=vincent-the-gamer.vscode-pycn) 插件，就可以自动高亮显示了。

![highlight](/imgs/highlight.png)