---
layout: doc
title: "Basic Usage"
lastUpdated: true
---

# Basic Usage

## How to run

PyCN runs `.pycn` files through the `pycn` CLI. For installation, see [Install PyCN](./intro.md#install-pycn).

The CLI currently exposes one command:

```shell
pycn run <file>
```

## Write your first PyCN program

Let's start with the traditional “Hello, world!” program.

- Create a file named `你好.pycn`:

```pycn
# print("Hello, world!")
打印（"你好，世界！"）
```

- Run it with the CLI:

```shell
pycn run 你好.pycn
```

## Inspect the generated Python

PyCN translates a `.pycn` file into Python before executing it. Add `--show-python` to print that translation instead of running it:

```shell
pycn run 你好.pycn --show-python
```

This is useful for checking how a construct maps to Python, or for reusing the generated code in a regular Python project.

## Import local `.pycn` modules

Local `.pycn` files can be imported by file name. The CLI resolves the module next to the importing file and registers it in memory, so no packaging step is needed.

```pycn
# 工具.pycn
定义 问候（名字）：
    返回 f"你好，{名字}！"
```

```pycn
# 主程序.pycn
导入 工具

打印（工具.问候（"世界"））
```

```shell
pycn run 主程序.pycn
```

Only plain module names are resolved; dotted module paths are not supported.

## Interact with Python virtualenvs

PyCN can use packages installed in virtualenvs created by `pip`, `poetry` and other tools. Point `PYTHONPATH` at the environment's `site-packages` before running:

```shell
# poetry
PYTHONPATH=.venv/lib/python<VERSION>/site-packages pycn run ./包管理器测试.pycn

# pip
PYTHONPATH=venv/lib/python<VERSION>/site-packages pycn run ./包管理器测试.pycn
```

- [包管理器测试.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/包管理器测试.pycn)

## Examples

The repository contains runnable examples for the main language features:

- [打印.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/打印.pycn)
- [函数.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/函数.pycn)
- [类.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/类.pycn)
- [装饰器完整测试.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/装饰器完整测试.pycn)
- [匿名函数.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/匿名函数.pycn)
- [索引迭代.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/索引迭代.pycn)
- [切片.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/切片.pycn)
- [列表方法测试.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/列表方法测试.pycn)
- [字典.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/字典.pycn)
- [导入.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/导入.pycn)
- [测试导入.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/测试导入.pycn)
- [位运算.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/位运算.pycn)
- [中文数字.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/中文数字.pycn)
- [交换变量.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/交换变量.pycn)

For the complete mapping, see the [Syntax Reference](./syntax.md).
