import { BitwardenClient } from '../../bitwarden_client';
import { LoggingLevel } from '../../bitwarden_client/logging_level';
import { DeviceType } from '../../bitwarden_client/schemas';

/// Load wasm. Not sure why, but this isn't happening when we import pkg/index.js
import * as wasm from "../../pkg/index_bg.wasm";
import { __wbg_set_wasm } from "../../pkg/index_bg.js";
__wbg_set_wasm(wasm);
export * from "../../pkg/index_bg.js";
/// WASM loaded

const client = new BitwardenClient({
  apiUrl: "http://localhost:8081/api",
  identityUrl: "http://localhost:8081/identity",
  deviceType: DeviceType.SDK,
  userAgent: "Bitwarden JS SDK",
}, LoggingLevel.Debug);

const key = "UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ==";
const cipherText = "2.wdCOHqz8UoAezZBcBaXilQ==|/HNKsVacSuL0uh2FoSIl2w==|zL4gnsP+zU3rG0bF9SQ5uphhy5HDTH26GNGzMyYVK1o=";

export async function encrypt(numOperations: number) {
  return await client.performance.encrypt(key, numOperations);
}

export async function decrypt(numOperations: number) {
  return await client.performance.decrypt(cipherText, key, numOperations);
}

export async function encryptDirect(numOperations: number) {
  return await client.performance.perf_encrypt(numOperations);
}

export async function decryptDirect(numOperations: number) {
  return await client.performance.perf_decrypt(numOperations);
}

export async function pbkdf2(iterations: number) {
  return await client.performance.pbkdf2(iterations, "mypassword");
}

export function normalizeRustResult(tableData: any[], numOperations: number) {
  return tableData.map(r => {
    if (r["Task Name"] !== "Rust" && r["Task Name"] !== "Rust (command)") {
      return r;
    }
    r["Task Name"] = `${r["Task Name"]} (normalized by ${numOperations})`;
    r["ops/sec"] = (Number.parseInt(r["ops/sec"].replace(/,/g,"")) * numOperations).toLocaleString("en-US");
    r["Average Time (ns)"] = r["Average Time (ns)"] / numOperations;
    r["Samples"] = r["Samples"] * numOperations;
    return r;
  })
}
