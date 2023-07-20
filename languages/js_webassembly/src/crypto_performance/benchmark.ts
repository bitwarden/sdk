import { Bench } from "tinybench";
import { Suite } from "benchmark";

import { encrypt as forge_encrypt, decrypt as forge_decrypt } from "./forge";
import { encrypt as web_crypto_encrypt, decrypt as web_crypto_decrypt } from "./web_crypt";
import { encrypt as rust_encrypt, decrypt as rust_decrypt, normalizeRustResult } from "./rust";

export async function benchmark_encrypt() {
  const bench = new Bench();
  const rustNormalization = 1000;
  bench.add("Forge", function () {
      forge_encrypt("message");
  }).add("WebCrypto", async function () {
    await web_crypto_encrypt("message");
  }).add("Rust", async function () {
    await rust_encrypt(rustNormalization);
  });

  bench.warmup();
  await bench.run();

  return normalizeRustResult(bench.table(), rustNormalization);
}

export async function benchmark_decrypt() {
  const bench = new Bench();
  const rustNormalization = 10;
  bench.add("Forge", function () {
    forge_decrypt();
  }).add("WebCrypto", async function () {
    await web_crypto_decrypt();
  }).add("Rust", async function () {
    await rust_decrypt(rustNormalization);
  });

  await bench.warmup();
  await bench.run();

  return normalizeRustResult(bench.table(), rustNormalization);
}
