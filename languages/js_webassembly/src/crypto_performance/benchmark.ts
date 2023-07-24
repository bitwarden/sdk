import { Bench } from "tinybench";

import { encrypt as forgeEncrypt, decrypt as forgeDecrypt, pbkdf2 as forgePbkdf2 } from "./forge";
import { encrypt as webCryptoEncrypt, decrypt as webCryptoDecrypt, makeDerivedKey, pbkdf2 as webCryptoPbkdf2 } from "./web_crypto";
import { encrypt as rustEncrypt, decrypt as rustDecrypt, normalizeRustResult, encryptDirect as rustEncryptDirect, decryptDirect as rustDecryptDirect, pbkdf2 as rustPbkdf2 } from "./rust";

export async function benchmark_pbkdf2() {
  const bench = new Bench();
  const iterations = 1000;
  bench.todo("Forge", function () {
    forgePbkdf2(iterations);
  }).add("WebCrypto", async function () {
    webCryptoPbkdf2(iterations);
  }).add("Rust", async function () {
    await rustPbkdf2(iterations);
  });

  bench.warmup();
  await bench.run();

  return bench.table();
}

export async function benchmark_encrypt() {
  const bench = new Bench();
  const rustNormalization = 100;
  const webCryptoKey = await makeDerivedKey(5000);
  bench.add("Forge", function () {
      forgeEncrypt("message");
  }).add("WebCrypto", async function () {
    await webCryptoEncrypt("message", webCryptoKey);
  }).add("Rust (command)", async function () {
    await rustEncrypt(rustNormalization);
  }).todo("Rust", async function () {
    await rustEncryptDirect(rustNormalization);
  });

  bench.warmup();
  await bench.run();

  return normalizeRustResult(bench.table(), rustNormalization);
}

export async function benchmark_decrypt() {
  const bench = new Bench();
  const rustNormalization = 100;
  const webCryptoKey = await makeDerivedKey(5000); // Keep this iteration count constant -- it's the key used to encrypt the message being decrypted
  bench.add("Forge", function () {
    forgeDecrypt();
  }).add("WebCrypto", async function () {
    await webCryptoDecrypt(webCryptoKey);
  }).add("Rust (command)", async function () {
    await rustDecrypt(rustNormalization);
  }).todo("Rust", async function () {
    await rustDecryptDirect(rustNormalization);
  });

  await bench.warmup();
  await bench.run();

  return normalizeRustResult(bench.table(), rustNormalization);
}
