import { benchmark_decrypt, benchmark_encrypt, benchmark_pbkdf2 } from './src/crypto_performance/benchmark';

var benchmark_running = false;

async function run_benchmark(name: string, callback: () => Promise<any>): Promise<void> {
  if (benchmark_running) {
    return;
  }
  benchmark_running = true;

  const result = await callback();

  console.log("Benchmark results for ", name)
  console.table(result);
  benchmark_running = false;
}

const pbkdf2_button = document.getElementById("pbkdf2") as HTMLButtonElement;
const encrypt_button = document.getElementById("encrypt") as HTMLButtonElement;
const decrypt_button = document.getElementById("decrypt") as HTMLButtonElement;

function disableButtons(disable: boolean) {
  pbkdf2_button.disabled = disable;
  encrypt_button.disabled = disable;
  decrypt_button.disabled = disable;
}

encrypt_button.addEventListener("click", async (e) => {
  e.preventDefault();
  e.stopImmediatePropagation();
  disableButtons(true);
  await run_benchmark("encrypting", benchmark_encrypt);
  disableButtons(false);
});
decrypt_button.addEventListener("click", async (e) => {
  e.preventDefault();
  e.stopImmediatePropagation();
  disableButtons(true);
  run_benchmark("decrypting", benchmark_decrypt)
    .then(() => disableButtons(false));
});
pbkdf2_button.addEventListener("click", async (e) => {
  e.preventDefault();
  e.stopImmediatePropagation();
  disableButtons(true);
  run_benchmark("pbkdf2", benchmark_pbkdf2)
    .then(() => disableButtons(false));
});
