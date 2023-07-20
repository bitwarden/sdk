import { Bench } from "tinybench";

import { encrypt as forgeEncrypt, decrypt as forgeDecrypt, makeKey as forgeMakeKey } from "./forge";
import { encrypt as webCryptoEncrypt, decrypt as webCryptoDecrypt, makeDerivedKey, makeKey as webCryptoMakeKey } from "./web_crypto";
import { encrypt as rustEncrypt, decrypt as rustDecrypt, normalizeRustResult, encryptDirect as rustEncryptDirect, decryptDirect as rustDecryptDirect } from "./rust";

export async function benchmark_pbkdf2() {
  const bench = new Bench();
  
  bench.add("Forge", function () {
    forgeMakeKey();
  }).add("WebCrypto", async function () {
    webCryptoMakeKey();
  }).todo("Rust", async function () {
    // await rust_make_key();
  });

  bench.warmup();
  await bench.run();

  return bench.table();
}

export async function benchmark_encrypt() {
  const bench = new Bench();
  const rustNormalization = 50;
  const webCryptoKey = await makeDerivedKey();
  bench.add("Forge", function () {
      forgeEncrypt("message");
  }).add("WebCrypto", async function () {
    await webCryptoEncrypt("message", webCryptoKey);
  }).todo("Rust through command", async function () {
    await rustEncrypt(rustNormalization);
  }).add("Rust", async function () {
    await rustEncryptDirect(rustNormalization);
  });

  bench.warmup();
  await bench.run();

  return normalizeRustResult(bench.table(), rustNormalization);
}

export async function benchmark_decrypt() {
  const bench = new Bench();
  const rustNormalization = 50;
  const webCryptoKey = await makeDerivedKey();
  bench.add("Forge", function () {
    forgeDecrypt();
  }).add("WebCrypto", async function () {
    await webCryptoDecrypt(webCryptoKey);
  }).todo("Rust through command", async function () {
    await rustDecrypt(rustNormalization);
  }).add("Rust", async function () {
    await rustDecryptDirect(rustNormalization);
  });

  await bench.warmup();
  await bench.run();

  return normalizeRustResult(bench.table(), rustNormalization);
}
