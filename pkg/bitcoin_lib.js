import * as wasm from "./bitcoin_lib_bg.wasm";
import { __wbg_set_wasm } from "./bitcoin_lib_bg.js";
__wbg_set_wasm(wasm);
export * from "./bitcoin_lib_bg.js";
