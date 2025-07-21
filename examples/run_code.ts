import { close, DataType, load, open } from "ffi-rs"

// library_name, don't add suffix
// e.g. dylink-darwin-aarch64
const library = "libpycn" 

// path to library
const path = "../target/debug/libpycndylib.dylib"

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

主函数（）
`]
})

close(library)