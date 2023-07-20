import { benchmark_decrypt, benchmark_encrypt } from './src/crypto_performance/benchmark';

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

document.getElementById("encrypt").addEventListener("click", async (e) => {
  e.preventDefault();
  e.stopImmediatePropagation();
  await run_benchmark("encrypting", benchmark_encrypt);
});
document.getElementById("decrypt").addEventListener("click", async (e) => {
  e.preventDefault();
  e.stopImmediatePropagation();
  await run_benchmark("decrypting", benchmark_decrypt);
});
