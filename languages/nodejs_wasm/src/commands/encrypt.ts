import { Command } from "commander";
import * as program from "commander";
import Bench from "tinybench";
import { encrypt as forgeEncrypt } from "../crypto_performance/forge";
import {
  encryptDirect as sdkEncryptDirect,
  encrypt as sdkEncrypt,
  normalizeRustResult,
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

export function register_encrypt(rootCommand: Command) {
  rootCommand
    .command("encrypt")
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
            forgeEncrypt(i.toString());
          }
        });
      }

      if (libraries.includes("sdkCommand")) {
        bench.add("sdkCommand", async function () {
          await sdkEncrypt(sdkIterations);
        });
      }

      if (libraries.includes("sdk")) {
        bench.add("sdk", async function () {
          await sdkEncryptDirect(sdkIterations);
        });
      }

      await bench.warmup();
      await bench.run();

      const result = bench.table();

      console.table(normalizeRustResult(result, sdkIterations));
    });
  return rootCommand;
}
