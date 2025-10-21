---
layout: doc
title: "基本使用方法"
lastUpdated: true
---

# 基本使用方法

## 如何运行代码

你可以用多种方式运行：可执行二进制文件，Node.js/Web WASM包，动态链接库以及HTTP API.

如何构建这些文件： [看这一部分](./intro.md#构建并使用pycn)

## 编写你的第一个PyCN程序

我们还是从一个「Hello, world!」程序开始 ~~传统艺能了~~。

- 创建一个`你好.pycn`文件.

```pycn
# print("Hello, world!")
打印（"你好，世界！"）
```

- 使用CLI运行:

```shell
pycn run 你好.pycn
```

## 和虚拟环境交互

Pycn支持和Python虚拟环境交互，如`pip`, `poetry`等包管理器创建的虚拟环境，注意此时运行命令有所不同。

```shell
# poetry
PYTHONPATH=.venv/lib/python<VERSION>/site-packages pycn run ./包管理器测试.pycn

# pip
PYTHONPATH=venv/lib/python<VERSION>/site-packages pycn run ./包管理器测试.pycn
```

- [包管理器测试.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/包管理器测试.pycn)

## 案例

我写了一些案例来帮助大家理解PyCN:

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

