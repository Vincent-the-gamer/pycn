<div align="center">
    <img src=".github/pycn-logo.png" style="height: 90px;"/>
    <h1>PyCN</h1>
    <b>Write Python code in Chinese, just for fun ～(∠・ω< )⌒★</b>
    <br/>
    <div><a href="./README.md" target="_blank">中文文档</a> | English</div>
</div>

<br/>

# Preview

![preview](.github/preview.png)

# Documentation

See PyCN docs at: https://pycn.vincentthegamer.dpdns.org

# Project Structure

```
pycn/
├── pycn/            # CLI main program (pyo3-based, static Python link by default)
├── pycn-dylib/      # C dynamic library wrapper
├── parser/          # Core parser (logos lexer + hand-written recursive descent)
├── parser-wasm/     # WASM bindings for the parser (Web / Node.js)
├── http-server/     # HTTP code execution server (Axum-based)
├── scripts/         # Build scripts
│   └── build-release.sh  # Release build & packaging
├── build/           # Build cache (PBS Python, auto-downloaded on first build)
├── python-stdlib/   # Python stdlib copy (auto-downloaded on first run)
└── examples/        # Example code
```

# Install

Download the pre-built package for your platform from [GitHub Releases](https://github.com/Vincent-the-gamer/pycn/releases), extract it, and you're ready to go — no Python or Rust installation required.

### Supported Platforms

| Platform | Architectures |
|----------|--------------|
| Linux    | x64, arm64  |
| macOS    | x64, arm64  |
| Windows  | x64, arm64  |

After extracting, run PyCN directly:

```shell
./pycn run examples/打印.pycn
```

# Build (for contributors)

## Prerequisites

- [Rust](https://rustup.rs/) toolchain
- Network connection (pre-built Python is downloaded on first setup)

## Development Build (Recommended)

No setup required — the first `cargo build` automatically downloads a pre-built standalone Python (~25 MB) for static linking; the standard library itself is downloaded on first run when not found:

```shell
# Build pycn (auto-downloads standalone Python on first build)
cargo build -p pycn --release

# Run an example
cargo run -p pycn --release -- run examples/打印.pycn
```

What the first build does automatically:

1. Downloads a pre-built standalone Python from [python-build-standalone](https://github.com/astral-sh/python-build-standalone) into `build/python-static/`
2. Statically links `libpython3.12`, producing a binary independent of the system Python

The standard library is not handled at build time: on first run, `pycn` auto-downloads it (~25 MB, once) if `python-stdlib/` is not found next to the binary.

> [!NOTE]
> - To use the system Python instead, build with `cargo build --no-default-features`
> - `cargo clean` will not remove the `build/` cache (it lives at the project root, not inside `target/`); `python-stdlib/` is generated at runtime

## Other Crates

```shell
# C dynamic library
cargo build -p pycn-dylib --release

# Node.js / Web WASM
cd parser-wasm
wasm-pack build --out-dir output            # ES Module (--target bundler by default)
wasm-pack build --target nodejs --out-dir output  # CommonJS
wasm-pack build --target web --out-dir output     # Web

# HTTP Server
cargo build -p http-server --release
```

## Release Build (Standalone Package)

```shell
bash scripts/build-release.sh
```

This script compiles pycn and packages the binary together with the Python standard library into `target/release/pycn-standalone/`, producing a standalone distribution that does not depend on a system Python installation.

# License

[MIT License](./LICENSE.md)

Copyright (c) 2025-PRESENT Vincent-the-gamer <https://github.com/Vincent-the-gamer>
