import { __wbg_set_wasm } from "./bitwarden_wasm_internal_bg.js";

export function init(wasm) {
  __wbg_set_wasm(wasm);
}

export * from "./bitwarden_wasm_internal_bg.js";
