---
layout: doc
title: "Basic Usage"
lastUpdated: true
---

# Basic Usage

## How to run

You can run PyCN code using executable binary file, Node.js/Web WASM pack, dynamic link library or through http API.

How to build: [See this section](./intro.md#build-and-use-pycn)

## Write your first PyCN program

Let's get started with 「Hello, world!」program.

- Create a `你好.pycn` (which means `hello.pycn`) file.

```pycn
# print("Hello, world!")
打印（"你好，世界！"）
```

- Run with pycn CLI:

```shell
pycn run 你好.pycn
```

## Interact with Python virtualenvs

Worrying about abilities? PyCN is able to interact with virtualenvs created by `pip`, `poetry` and etc. Please notice that the run command is something different now.

```shell
# poetry
PYTHONPATH=.venv/lib/python<VERSION>/site-packages pycn run ./包管理器测试.pycn

# pip
PYTHONPATH=venv/lib/python<VERSION>/site-packages pycn run ./包管理器测试.pycn
```

- [包管理器测试.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/包管理器测试.pycn)

## Examples

I've given some examples for you to know more about Pycn:

- [函数.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/函数.pycn)
- [索引迭代.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/索引迭代.pycn)
- [打印.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/打印.pycn)
- [导入.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/导入.pycn)
- [类.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/类.pycn)
- [位运算.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/位运算.pycn)
- [中文数字.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/中文数字.pycn)
- [匿名函数.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/匿名函数.pycn)
- [交换变量.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/交换变量.pycn)
- [切片.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/切片.pycn)
- [列表方法测试.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/列表方法测试.pycn)
- [字典.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/字典.pycn)
- [装饰器完整测试.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/装饰器完整测试.pycn)