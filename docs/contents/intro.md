---
layout: doc
title: "Introduction"
lastUpdated: true
---

# Introduction

PyCN is literally means `Python Chinese`, which uses the same syntaxes as Python. However, it's keywords, literals, built-in functions and operators is using Chinese.

Let's have a look to this example:

```pycn
定义 是否是质数（被判断的数）：
    如果 被判断的数 小于 二：
        返回 假
    迭代 数一 在 范围（二，整数（被判断的数 取幂 零点五）加 一）：
        如果 被判断的数 取余 数一 等于 零：
            返回 假
    返回 真
```

the code above means:

```python
def is_prime(num):
    if num < 2:
        return false
    for i in range(2, int(num ** 0.5) + 1):
        if num % i == 0:
            return false
    return true
```

That's right! PyCN has exactly the same coding style as Python, but as you can see, the PyCN code is in Chinese like I said before.

## Install PyCN interpreter

To run your own PyCN code, I recommend you to downlaod the compiled PyCN interpreter:

- [GitHub Release](https://github.com/Vincent-the-gamer/pycn/releases)

if you are using `macOS`, you can get pycn from `HomeBrew`:

```shell
brew tap vincent-the-gamer/homebrew-tap
brew install pycn
```

## Online playground

Besides, if you just want to play PyCN for a while, I put the wasm pack to my website:

[https://mayu.vince-g.xyz/code-runner](https://mayu.vince-g.xyz/code-runner)

Switch language to `pycn` and you are ready.

## Syntax Highlighting

Currently, PyCN supports **syntax highlighting** in VSCode.

Install [`Pycn`](https://marketplace.visualstudio.com/items?itemName=vincent-the-gamer.vscode-pycn) extension and get highlight automatically.

![highlight](/imgs/highlight.png)
