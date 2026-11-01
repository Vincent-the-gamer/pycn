---
layout: doc
title: "Syntax Reference"
lastUpdated: true
---

# Syntax Reference

PyCN keeps Python's syntax and replaces its vocabulary. English keywords are still accepted, and half-width and full-width punctuation can be mixed freely.

This page lists what the current parser implements. For a quick look at runnable code, see the [examples](./basic-usage.md#examples).

## Statements

| PyCN | Python |
|------|--------|
| `定义 函数名（参数）：` | `def name(params):` |
| `类 类名：` / `类 类名（基类）：` | `class Name:` / `class Name(Base):` |
| `如果 条件：` / `要不然 条件：` / `否则：` | `if` / `elif` / `else` |
| `迭代 变量 在 可迭代对象：` | `for var in iterable:` |
| `返回 值` / `返回` | `return value` / `return` |
| `跳出` / `继续` / `过` | `break` / `continue` / `pass` |
| `导入 模块 作为 别名` | `import module as alias` |
| `从 模块 导入 名称 作为 别名` | `from module import name as alias` |
| `拉姆达 参数：表达式` | `lambda params: expression` |
| `@装饰器` / `装饰 装饰器` | `@decorator` |
| `变量 赋值为 值` | `variable = value` |
| `甲，乙 赋值为 乙，甲` | `a, b = b, a` |
| `对象.属性 赋值为 值` | `obj.attr = value` |
| `字典【键】赋值为 值` | `d[key] = value` |

Function parameters support `*参数` for `*args` and `**键值对参数` for `**kwargs`. Named parameters keep their Chinese names.

```pycn
定义 求和（甲，乙，*参数，**键值对参数）：
    返回 甲 加 乙
```

## Keywords

| PyCN | Python | Notes |
|------|--------|-------|
| `定义` | `def` | Function definition |
| `如果` | `if` | |
| `要不然` | `elif` | |
| `否则` | `else` | |
| `迭代` | `for` | Multiple loop variables are supported |
| `在` | `in` | |
| `不是` | `not` | Unary negation |
| `和` | `and` | |
| `或` | `or` | |
| `空` | `None` | |
| `真` | `True` | |
| `假` | `False` | |
| `返回` | `return` | |
| `跳出` | `break` | |
| `继续` | `continue` | |
| `过` | `pass` | |
| `导入` | `import` | |
| `从` | `from` | |
| `作为` | `as` | |
| `类` | `class` | |
| `拉姆达` | `lambda` | |
| `装饰` | `@` | Decorator token |

The following keywords are recognized by the lexer but are not implemented by the parser yet: `当` (`while`), `是` (`is`), `尝试` / `意外情况` / `最终` (`try` / `except` / `finally`), `举起` (`raise`), `断言` (`assert`), `删除` (`del`), `全局的` (`global`), `非局部` (`nonlocal`), `产出` (`yield`), `等待` (`await`), `异步的` (`async`), `带上` (`with`), and `匹配` / `情况` (`match` / `case`).

## Operators

| PyCN | Python | Notes |
|------|--------|-------|
| `加` | `+` | |
| `减` | `-` | |
| `乘` | `*` | |
| `除以` | `/` | |
| `地板除` | `//` | |
| `取余` / `取模` | `%` | |
| `取幂` | `**` | |
| `赋值为` | `=` | Assignment |
| `等于` | `==` | |
| `不等于` / `！=` | `!=` | |
| `小于` / `《` | `<` | |
| `小于等于` / `《=` | `<=` | |
| `大于` / `》` | `>` | |
| `大于等于` / `》=` | `>=` | |
| `按位与` | `&` | |
| `按位或` | `|` | |
| `按位异或` | `^` | |
| `按位取反` | `~` | Unary |
| `左移` | `<<` | |
| `右移` | `>>` | |
| `和` | `and` | |
| `或` | `or` | |
| `不是` | `not` | Unary |

## Punctuation and literals

Chinese punctuation is optional: the lexer accepts both forms in the same file.

| PyCN | Python |
|------|--------|
| `（` `）` | `(` `)` |
| `【` `】` | `[` `]` |
| `，` | `,` |
| `：` | `:` |
| `“…”` / `‘…’` | `"..."` / `'...'` |
| `《` `》` | `<` `>` in comparisons |
| `！=` | `!=` |

Braces `{` `}`, the dot `.` and the decorator `@` stay half-width.

Data literals follow Python's model:

- Numbers can be written in Arabic or Chinese numerals: `十二`, `一百`, `零点一`, `负零点五`.
- Strings accept ASCII or Chinese quotes, and `f`-strings use the usual `f"..."` form.
- Lists use `【】`, dictionaries and sets use `{}`, and tuples use `()`.
- `真`, `假` and `空` map to `True`, `False` and `None`.

```pycn
名字 赋值为 “张三”
成绩 赋值为 【九十，八十五，一百】
信息 赋值为 {"姓名": 名字, "成绩": 成绩}
打印（f"{名字} 的成绩是 {成绩【零】}")
```

## Built-in functions

| PyCN | Python | PyCN | Python |
|------|--------|------|--------|
| `打印` | `print` | `范围` | `range` |
| `整数` | `int` | `小数` | `float` |
| `字符串` | `str` | `输入` | `input` |
| `长度` | `len` | `列表` | `list` |
| `字典` | `dict` | `集合` | `set` |
| `元组` | `tuple` | `索引迭代` | `enumerate` |
| `解析` | `eval` | `求和` | `sum` |
| `最小值` | `min` | `最大值` | `max` |
| `绝对值` | `abs` | `全部为真` | `all` |
| `有一个为真` | `any` | `映射` | `map` |
| `过滤` | `filter` | `拉链` | `zip` |
| `打开` | `open` | `执行` | `exec` |
| `类型` | `type` | `实例判断` | `isinstance` |
| `子类判断` | `issubclass` | `目录` | `dir` |
| `变量` | `vars` | `本地变量` | `locals` |
| `全局变量` | `globals` | `帮助` | `help` |
| `编号` | `id` | `排序` | `sorted` |
| `父类` | `super` | `对象` | `object` |
| `类方法` | `classmethod` | `静态方法` | `staticmethod` |
| `属性` | `property` | `异常` | `Exception` |

The built-in names can also be used as ordinary names where a variable is expected, such as function parameters.

## List methods

| PyCN | Python | Description |
|------|--------|-------------|
| `列表.添加（元素）` | `list.append(element)` | Append one element |
| `列表.插入（索引，元素）` | `list.insert(index, element)` | Insert at an index |
| `列表.移除（元素）` | `list.remove(element)` | Remove the first match |
| `列表.弹出（）` / `列表.弹出（索引）` | `list.pop()` / `list.pop(index)` | Remove and return |
| `列表.清空（）` | `list.clear()` | Remove all elements |
| `列表.复制（）` | `list.copy()` | Shallow copy |
| `列表.计数（元素）` | `list.count(element)` | Count occurrences |
| `列表.扩展（其它列表）` | `list.extend(other)` | Extend with an iterable |
| `列表.索引（元素）` | `list.index(element)` | First index of an element |
| `列表.反转（）` | `list.reverse()` | Reverse in place |
| `列表.排序（）` | `list.sort()` | Sort in place |

`排序` maps to the `sorted` built-in when called directly, and to `.sort()` when used as an attribute.

## Magic methods

| PyCN | Python | PyCN | Python |
|------|--------|------|--------|
| `魔法初始化` | `__init__` | `魔法新建` | `__new__` |
| `魔法销毁` | `__del__` | `魔法字符串表示` | `__str__` |
| `魔法表达式` | `__repr__` | `魔法字节表示` | `__bytes__` |
| `魔法格式化` | `__format__` | `魔法等于` | `__eq__` |
| `魔法不等于` | `__ne__` | `魔法小于` | `__lt__` |
| `魔法小于等于` | `__le__` | `魔法大于` | `__gt__` |
| `魔法大于等于` | `__ge__` | `魔法哈希` | `__hash__` |
| `魔法布尔` | `__bool__` | `魔法属性获取` | `__getattr__` |
| `魔法属性设置` | `__setattr__` | `魔法属性删除` | `__delattr__` |
| `魔法属性获取项` | `__getitem__` | `魔法属性设置项` | `__setitem__` |
| `魔法属性删除项` | `__delitem__` | `魔法长度` | `__len__` |
| `魔法可迭代` | `__iter__` | `魔法迭代下一个` | `__next__` |
| `魔法反转` | `__reversed__` | `魔法包含` | `__contains__` |
| `魔法加` | `__add__` | `魔法减` | `__sub__` |
| `魔法乘` | `__mul__` | `魔法除` | `__truediv__` |
| `魔法地板除` | `__floordiv__` | `魔法取余` | `__mod__` |
| `魔法取幂` | `__pow__` | `魔法左移` | `__lshift__` |
| `魔法右移` | `__rshift__` | `魔法按位与` | `__and__` |
| `魔法按位或` | `__or__` | `魔法按位异或` | `__xor__` |
| `魔法取反` | `__invert__` | `魔法可调用` | `__call__` |
| `魔法进入` | `__enter__` | `魔法退出` | `__exit__` |
| `魔法拷贝` | `__copy__` | `魔法深拷贝` | `__deepcopy__` |
| `魔法名称` | `__name__` | | |

## Current limitations

- Keyword arguments in calls (`name=value`) and comprehensions are not supported.
- `while`, `try` / `except`, `with`, `async` / `await`, `yield`, `match`, `raise`, `assert`, `del`, `global` and `nonlocal` are not implemented yet.
- Multi-variable assignment targets must be plain names; attribute and index targets are assigned one at a time.
- A local `.pycn` module is resolved by file name in the importing file's directory. Dotted module paths are not supported.
