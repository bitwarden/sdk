import { Command } from 'commander';
import * as program from "commander";
import Bench from 'tinybench';
import { pbkdf2 as forgePbkdf2 } from '../crypto_performance/forge';
import { pbkdf2 as sdkPbkdf2 } from '../crypto_performance/rust';

export function register_pbkdf2(rootCommand: Command) {
  rootCommand.command("pbkdf2")
    .addOption(new program.Option("-l, --libraries <libraries...>", "Libraries to test").choices(["forge", "sdk"]))
    .addOption(new program.Option("-i, --iterations <iterations>", "Number of iterations").default("600000"))
    .action(async (options: program.OptionValues) => {
      const libraries = options.libraries;
      const iterations = options.iterations;
      const bench = new Bench();

      if (libraries.includes("forge")) {
        bench.add("Forge", function () {
          forgePbkdf2(iterations);
        });
      }

      if (libraries.includes("sdk")) {
        bench.add("sdk", async function () {
          await sdkPbkdf2(iterations);
        });
      }

      await bench.warmup();
      await bench.run();

      console.table(bench.table());
    });
  return rootCommand;
}
