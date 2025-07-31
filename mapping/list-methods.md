# Pycn List method mapping / Pycn List方法中文映射

- `list.append(element)` => 列表.添加（元素）
- `list.insert(index, element)` => 列表.插入（索引，元素）
- `list.remove(element)` => 列表.移除（元素）
- `list.pop() / list.pop(index)` => 列表.弹出（）/ 列表.弹出（索引）
- `list.clear()` => 列表.清空（）
- `list.copy()` => 列表.复制（）
- `list.count` => 列表.计数（）
- `list.extend(other_list)` => 列表.扩展（其它列表）
- `list.index(element)` => 列表.索引（元素）
- `list.reverse()` => 列表.反转（）
- `list.sort()` => 列表.排序（）

本文档列出了支持的Pycn list方法的中文名称及其对应的英文方法：

# Methods instruction / 方法说明

| 中文名称 | Method Name | 说明 | 示例 |
|---------|----------|------|------|
| 添加 | append | 在列表末尾添加一个元素 | `列表.添加(元素)` |
| 插入 | insert | 在指定位置插入元素 | `列表.插入(索引, 元素)` |
| 移除 | remove | 移除列表中第一个匹配的元素 | `列表.移除(元素)` |
| 弹出 | pop | 移除并返回指定位置的元素，默认为最后一个 | `列表.弹出()` 或 `列表.弹出(索引)` |
| 清空 | clear | 清空列表中的所有元素 | `列表.清空()` |
| 复制 | copy | 返回列表的浅拷贝 | `新列表 赋值为 列表.复制()` |
| 计数 | count | 返回指定元素在列表中出现的次数 | `列表.计数(元素)` |
| 扩展 | extend | 用另一个可迭代对象扩展列表 | `列表.扩展(其他列表)` |
| 索引 | index | 返回指定元素第一次出现的索引 | `列表.索引(元素)` |
| 反转 | reverse | 就地反转列表 | `列表.反转()` |
| 排序 | sort | 就地排序列表 | `列表.排序()` |

## Use cases / 使用示例

- [列表方法测试](../examples/列表方法测试.pycn)
