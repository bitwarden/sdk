import { Bench } from "tinybench";

import { encrypt as forge_encrypt, decrypt as forge_decrypt, encrypt } from "./forge";
import { encrypt as web_crypto_encrypt, decrypt as web_crypto_decrypt } from "./web_crypto";
import { encrypt as rust_encrypt, decrypt as rust_decrypt, normalizeRustResult, encrypt_direct, decrypt_direct } from "./rust";

export async function benchmark_encrypt() {
  const bench = new Bench();
  const rustNormalization = 50;
  bench.add("Forge", function () {
      forge_encrypt("message");
  }).add("WebCrypto", async function () {
    await web_crypto_encrypt("message");
  }).todo("Rust through command", async function () {
    await rust_encrypt(rustNormalization);
  }).add("Rust", async function () {
    await encrypt_direct(rustNormalization);
  });

  bench.warmup();
  await bench.run();

  return normalizeRustResult(bench.table(), rustNormalization);
}

export async function benchmark_decrypt() {
  const bench = new Bench();
  const rustNormalization = 50;
  bench.add("Forge", function () {
    forge_decrypt();
  }).add("WebCrypto", async function () {
    await web_crypto_decrypt();
  }).todo("Rust through command", async function () {
    await rust_decrypt(rustNormalization);
  }).add("Rust", async function () {
    await decrypt_direct(rustNormalization);
  });

  await bench.warmup();
  await bench.run();

  return normalizeRustResult(bench.table(), rustNormalization);
}
