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
    funcName: "run_my_pycn_file",
    retType: DataType.Void,
    paramsType: [DataType.String],
    paramsValue: ["./import.pycn"]
})

close(library)