# PyCN

Write Python code in Chinese, just for fun ～(∠・ω< )⌒★

# Usage

## CLI

You can use `pycn` to run normal Python, **Or `.pycn` codes**. 

Keywords mapping: `(key, value)`

```
("定义", "def"),
("打印", "print"),
("空", "None"),
("真", "True"),
("假", "False"),
("如果", "if"),
("要不然", "elif"),
("否则", "else"),
("返回", "return"),
("迭代", "for"),
("在", "in"),
("范围", "range"),
("整数", "int"),
("小数", "float"),
("字符串", "str"),
("长度", "len"),
("索引迭代", "enumerate"),
("解析", "eval"),
("字典", "dict"),
("类", "class"),
("导入", "import"),
("是", "is"),
("不是", "not"),
```

Example: 

> [!NOTE]
> You can run this example by `pycn run -f examples/demo.pycn`

`examples/demo.pycn`: This function 是否是质数 is to check a prime number.
```
定义 是否是质数（被判断的数）：
    如果 被判断的数 《 2：
        返回 假
    迭代 数1 在 范围（2，整数（被判断的数 ** 0.5）+ 1）：
        如果 被判断的数 % 数1 == 0：
            返回 假
    返回 真

定义 主函数（）：
    迭代 数字 在 范围（1，100）：
        如果 是否是质数（数字）：
            打印（数字）

    布尔值1 = 1 》 2 和 1 《 3
    布尔值2 = 1 》 2 或 1 《 3
    
    打印（布尔值1）
    打印（布尔值2）

主函数（）
```

Result:
```
2
3
5
7
11
13
17
19
23
29
31
37
41
43
47
53
59
61
67
71
73
79
83
89
97
False
True
```

## Any other environment

PyCN provides dynamic link libraries(dylibs) for any other environment, like Node.js.

You can download dylibs from release.

Example in Node.js:

> [!NOTE]
> You can run this example by `pnpm run tsx examples/demo.ts`

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
    如果 被判断的数 《 2：
        返回 假
    迭代 数1 在 范围（2，整数（被判断的数 ** 0.5）+ 1）：
        如果 被判断的数 % 数1 == 0：
            返回 假
    返回 真

定义 主函数（）：
    迭代 数字 在 范围（1，100）：
        如果 是否是质数（数字）：
            打印（数字）
    
    布尔值1 = 1 》 2 和 1 《 3
    布尔值2 = 1 》 2 或 1 《 3
    
    打印（布尔值1）
    打印（布尔值2）

主函数（）
`]
})

close(library)
```