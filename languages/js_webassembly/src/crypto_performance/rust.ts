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

export function normalizeRustResult(tableData: any[], numOperations: number) {
  return tableData.map(r => {
    if (r["Task Name"] !== "Rust") {
      return r;
    }
    r["Task Name"] = `Rust (normalized by ${numOperations})`;
    r["ops/sec"] = (Number.parseInt(r["ops/sec"].replace(/,/g,"")) * numOperations).toLocaleString("en-US");
    r["Average Time (ns)"] = r["Average Time (ns)"] / numOperations;
    r["Samples"] = r["Samples"] * numOperations;
    return r;
  })
}
