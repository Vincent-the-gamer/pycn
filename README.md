<div align="center">
    <img src=".github/pycn-logo.png" style="height: 90px;"/>
    <h1>PyCN</h1>
    <b>用中文写Python代码, 图一乐～(∠・ω< )⌒★</b>
    <br/>
    <div>中文文档 | <a href="./README_en.md" target="_blank">English</a></div>
</div>

<br/>

# 游乐场

快速体验**Pycn**：https://mayu.vince-g.xyz/code-runner

记得把语言切换至 `pycn`

![playground](.github/playground.png)

# 预览

![preview](.github/preview.png)

# 使用文档

在这里查看PyCN使用文档：https://pycn.vince-g.xyz/zh_hans/

# 构建

> [!IMPORTANT]
> Pycn静态编译依赖于特定Python版本，所以你需要在一个拥有相同版本的环境才能正常运行这个构建。
>
> 举个例子：如果你使用Python 3.12.x版本编译你的Pycn(Cargo会调用你系统中默认Python版本来编译)，那么你需要一个安装了Python3.12.x的环境来运行这个构建。

```shell
# pycn
cargo build -p pycn --release

# pycn-dylib
cargo build -p pycn-dylib --release

# Node.js/Web WASM
cd parser-wasm
wasm-pack build --out-dir output # ES Module (默认参数：--target bundler)
wasm-pack build --target nodejs --out-dir output # CommonJS
wasm-pack build --target web --out-dir output # Web

# HTTP Server
cargo build -p http-server --release
```

# 开源证书

[MIT 证书](./LICENSE.md)

版权所有 (c) 2025-现在 Vincent-the-gamer <https://github.com/Vincent-the-gamer>