import { Command } from "commander";
import * as program from "commander";
import Bench from "tinybench";
import { decrypt as forgeDecrypt } from "../crypto_performance/forge";
import {
  normalizeRustResult,
  decrypt as sdkDecrypt,
  decryptDirect as sdkDecryptDirect,
} from "../crypto_performance/rust";

const librariesOption = new program.Option(
  "-l, --libraries <libraries...>",
  "Libraries to test"
).choices(["forge", "sdk", "sdkCommand"]);
const iterationsOption = new program.Option(
  "-i, --iterations <iterations>",
  "Number of iterations"
).default("1");
const sdkIterationOption = new program.Option(
  "--sdk-iterations <iterations>",
  "Number of iterations"
).default("10000");

export function register_decrypt(rootCommand: Command) {
  rootCommand
    .command("decrypt")
    .addOption(librariesOption)
    .addOption(iterationsOption)
    .addOption(sdkIterationOption)
    .action(async (options: program.OptionValues) => {
      const libraries = options.libraries;
      const iterations = options.iterations;
      const sdkIterations = options.sdkIterations;
      const bench = new Bench();

      if (libraries.includes("forge")) {
        bench.add("Forge", function () {
          for (let i = 0; i < iterations; i++) {
            forgeDecrypt();
          }
        });
      }

      if (libraries.includes("sdkCommand")) {
        bench.add("sdkCommand", async function () {
          await sdkDecrypt(sdkIterations);
        });
      }

      if (libraries.includes("sdk")) {
        bench.add("sdk", async function () {
          await sdkDecryptDirect(sdkIterations);
        });
      }

      await bench.warmup();
      await bench.run();

      const result = bench.table();

      console.table(normalizeRustResult(result, sdkIterations));
    });
  return rootCommand;
}
