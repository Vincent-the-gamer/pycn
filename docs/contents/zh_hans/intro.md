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

## 安装PyCN解释器

想要运行你自己的PyCN代码，我推荐你下载我编译好的PyCN解释器：

- [GitHub Release](https://github.com/Vincent-the-gamer/pycn/releases)

如果你使用`macOS`, 可以从`HomeBrew`安装:

```shell
brew tap vincent-the-gamer/homebrew-tap
brew install pycn
```

## 在线游玩

如果你只是想浅尝辄止，我把WebAssemly包部署在了我的网站

[https://mayu.vince-g.xyz/code-runner](https://mayu.vince-g.xyz/code-runner)

切换语言为`pycn`即可游玩。

## 语法高亮

目前，PyCN在VSCode编辑器中支持**语法高亮**。

只需要安装[`Pycn`](https://marketplace.visualstudio.com/items?itemName=vincent-the-gamer.vscode-pycn) 插件，就可以自动高亮显示了。

![highlight](/imgs/highlight.png)