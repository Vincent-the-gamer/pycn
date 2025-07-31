<div align="center">
    <img src=".github/pycn-logo.png" style="height: 90px;"/>
    <h1>PyCN</h1>
    <b>Write Python code in Chinese, just for fun ～(∠・ω< )⌒★</b>
    <br/>
    <div><a href="./README.md" target="_blank">中文文档</a> | Engligh</div>
</div>

<br/>

# Playground

Play **Pycn** at: https://mayu.vince-g.xyz/code-runner

Remember to switch language to `pycn`

![playground](.github/playground.png)

# Preview

![preview](.github/preview.png)

# Installation 

## Binary

### Quick install
If you are `macOS` user, using `homebrew`.

```shell
brew tap vincent-the-gamer/homebrew-tap
brew install pycn
```

### Manually
1. Download binary from [release](https://github.com/Vincent-the-gamer/pycn/releases), then `rename it` to pycn. (don't change suffix if there's a suffix), you can also copy the link and download in terminal.
```shell
# example
curl -LJO https://github.com/Vincent-the-gamer/pycn/releases/download/v1.0.0/pycn-darwin-aarch64
```

2. Put it anywhere, then add the folder to your PATH
3. Test env variable by running `pycn --help`.
4. Run pycn code by `pycn run --file xxx.pycn`

## Dylibs

PyCN provides dynamic link libraries(dylibs) for any other environment, like Node.js.

You can download dylibs from [release](https://github.com/Vincent-the-gamer/pycn/releases).

Example in Node.js:

```ts
import { close, DataType, load, open } from "ffi-rs"

// library_name, don't add suffix
// e.g. dylink-darwin-aarch64
const library = "library_name"

// path to library
const path = "/path/to/.dylib or .so or .dll"

open({
    library,
    path
})

load({
    library,
    funcName: "run_my_pycn",
    retType: DataType.Void,
    paramsType: [DataType.String],
    paramsValue: [`定义 是否是质数（被判断的数）：
    如果 被判断的数 小于 二：
        返回 假
    迭代 数一 在 范围（二，整数（被判断的数 取幂 零点五）加 一）：
        如果 被判断的数 取余 数一 等于 零：
            返回 假
    返回 真

定义 主函数（）：
    迭代 数字 在 范围（一，二十）：
        如果 是否是质数（数字）：
            打印（数字）

    布尔值一 赋值为 一 大于 二 和 一 小于 三
    布尔值二 赋值为 一 大于 二 或 一 小于 三
    
    打印（布尔值一）
    打印（布尔值二）
    
主函数（）
`]
})

close(library)
```

## WebAssembly code parser
This wasm packages is only to parse `.pycn` to Python code.

You will need a tool to run your code, like [Pyoxidizer](https://github.com/indygreg/PyOxidizer), which can package your Python code into an executable file.

In website, you can run it with [Pyodide](https://github.com/pyodide/pyodide), it's a Wasm Python runtime.

Download wasm package from [release](https://github.com/Vincent-the-gamer/pycn/releases).

# Syntax highlighting

For now, you can get syntax highlighted in VS Code by installing `pycn` extension.

# Examples

You can read the mappings about keywords, built-in functions, operators and so on.

- [Keywords mapping](mapping/keywords.md)
- [Built-in functions mapping](mapping/builtin-functions.md)
- [Operators mapping](mapping/operators.md)

> [!NOTE]
> You can run these examples by `pycn run -f examples/xxx.pycn`

- [函数.pycn](examples/函数.pycn)
- [索引迭代.pycn](examples/索引迭代.pycn)
- [打印.pycn](examples/打印.pycn)
- [导入.pycn](examples/导入.pycn)
- [类.pycn](examples/类.pycn)
- [位运算.pycn](examples/位运算.pycn)
- [中文数字.pycn](examples/中文数字.pycn)
- [匿名函数.pycn](examples/匿名函数.pycn)
- [交换变量.pycn](examples/交换变量.pycn)
- [切片.pycn](examples/切片.pycn)
- [列表方法测试.pycn](examples/列表方法测试.pycn)
- [字典.pycn](examples/字典.pycn)

# Build

## Locally

```shell
# pycn
cargo build -p pycn --release

# pycn-dylib
cargo build -p pycn-dylib --release

# wasm-nodejs
cd parser-wasm
wasm-pack build --target nodejs --out-dir output

# wasm-web
cd parser-wasm
wasm-pack build --target web --out-dir output
```

## Cross Platform

Use `Docker` image.

### Linux arm64
```shell
docker pull vincentthegamer/rust-python-ubuntu:latest

# Enter image bash
docker run -it --rm \
           -v $(pwd):/home/pycn \
           vincentthegamer/rust-python-ubuntu bash

# Change directory to your volume map.
cd /home/pycn

# Build project
cargo build -p pycn --release
cargo build -p pycn-dylib --release
```

### Linux amd64(x64)
```shell
docker pull vincentthegamer/rust-python-ubuntu-amd64:latest

# Enter image bash
docker run -it --rm \
           -v $(pwd):/home/pycn \
           vincentthegamer/rust-python-ubuntu-amd64 bash

# Change directory to your volume map.
cd /home/pycn

# Build project
cargo build -p pycn --release
cargo build -p pycn-dylib --release
```