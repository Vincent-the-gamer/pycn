---
layout: doc
title: "基本使用方法"
lastUpdated: true
---

# 基本使用方法

## 如何运行代码

PyCN 通过 `pycn` 命令行运行 `.pycn` 文件。安装方式见[安装 PyCN](./intro.md#安装-pycn)。

目前 CLI 只有一个命令：

```shell
pycn run <文件>
```

## 编写你的第一个 PyCN 程序

我们还是从一个「Hello, world!」程序开始 ~~传统艺能了~~。

- 创建一个 `你好.pycn` 文件：

```pycn
# print("Hello, world!")
打印（"你好，世界！"）
```

- 使用 CLI 运行：

```shell
pycn run 你好.pycn
```

## 查看生成的 Python 代码

PyCN 会先把 `.pycn` 文件翻译成 Python 再执行。加上 `--show-python` 可以只打印翻译结果，而不运行代码：

```shell
pycn run 你好.pycn --show-python
```

想确认某个写法会翻译成什么，或者想把生成的代码拿到普通 Python 项目里复用，这个选项都很有用。

## 导入本地 `.pycn` 模块

本地 `.pycn` 文件可以直接按文件名导入。CLI 会在导入文件所在目录解析模块，并把它们注册到内存中，无需任何打包步骤。

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

目前只解析普通模块名，不支持点号分隔的模块路径。

## 和虚拟环境交互

PyCN 可以和 `pip`、`poetry` 等包管理器创建的虚拟环境交互。运行前把 `PYTHONPATH` 指向虚拟环境的 `site-packages` 即可：

```shell
# poetry
PYTHONPATH=.venv/lib/python<VERSION>/site-packages pycn run ./包管理器测试.pycn

# pip
PYTHONPATH=venv/lib/python<VERSION>/site-packages pycn run ./包管理器测试.pycn
```

- [包管理器测试.pycn](https://github.com/Vincent-the-gamer/pycn/blob/main/examples/包管理器测试.pycn)

## 案例

仓库中提供了覆盖主要语言特性的可运行案例：

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

完整的映射表见[语法参考](./syntax.md)。
