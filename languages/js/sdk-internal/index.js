import { __wbg_set_wasm } from "./bitwarden_wasm_internal_bg.js";

// In order to support a fallback strategy for web we need to conditionally load the wasm file
export function init(wasm) {
  __wbg_set_wasm(wasm);
}

export * from "./bitwarden_wasm_internal_bg.js";
