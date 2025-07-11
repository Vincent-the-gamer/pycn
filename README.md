# PyCN

Write Python code in Chinese, just for fun ～(∠・ω< )⌒★

# Usage

## CLI

You can use `pycn` to run normal Python, **Or `.pycn` codes**. 

Keywords mapping: `key => value`

```
def => 定义
if => 如果
else => 否则
elif => 要不然
For => 迭代
while => 当
in => 在
is => 是
not => 不是
and => 和
or => 或
None => 空
True => 真
False => 假
return => 返回
break => 跳出
continue => 继续
pass => 过
import => 导入
from => 从
as => 作为
class => 类
try => 尝试
except => 意外情况
finally => 最终
raise => 举起
assert => 断言
del => 删除
global => 全局的
nonlocal => 非局部
lambda => 拉姆达
yield => 产出
await => 等待
async => 异步的
with => 带上
match => 匹配
case => 情况
print => 打印
```

Example 1: 

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

# Build

## For local OS

```shell
# pycn
cargo build -p pycn --release

# pycn-dylib
cargo build -p pycn-dylib --release
```

## Cross platform

Requires `Docker`.

```shell
# if you don't have cross, install it.  
# p.s. show your installed binaries by: cargo install --list
cargo install cross --git https://github.com/cross-rs/cross

# use cross
cross build -p pycn --release --target x86_64-pc-windows-gnu
cross build -p pycn-dylib --release --target x86_64-pc-windows-gnu
```

Targets in the project:
- x86_64-unknown-linux-gnu
- x86_64-pc-windows-gnu
