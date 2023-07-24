import { Command } from 'commander';
import * as program from "commander";
import * as benny from "benny";
import { pbkdf2 as forgePbkdf2 } from '../crypto_performance/forge';
import { pbkdf2 as sdkPbkdf2 } from '../crypto_performance/rust';
import { parseIntOption } from './option_parsing';
import { pbkdf2 as webPbkdf2 } from '../crypto_performance/web_crypto';

export function register_pbkdf2(rootCommand: Command) {
  rootCommand.command("pbkdf2")
    .addOption(new program.Option("-l, --libraries <libraries...>", "Libraries to test").choices(["forge", "sdk", "web"]))
    .addOption(new program.Option("-i, --iterations <iterations>", "Number of iterations").argParser(parseIntOption).default("600000"))
    .action(async (options: program.OptionValues) => {
      const libraries = options.libraries;
      const iterations = options.iterations as number;

      const bennyOperations = []
      if (libraries.includes("forge")) {
        bennyOperations.push(benny.add("Forge", async () => {
          await forgePbkdf2(iterations);
        }));
      }

      if (libraries.includes("web")) {
        bennyOperations.push(benny.add("WebCrypto", async () => {
          await webPbkdf2(iterations);
        }));
      }

      if (libraries.includes("sdk")) {
        bennyOperations.push(benny.add("sdk", async () => {
          await sdkPbkdf2(iterations);
        }));
      }
      bennyOperations.push(benny.cycle());
      bennyOperations.push(benny.complete());

      await benny.suite("PBKDF2", ...bennyOperations);
    });
  return rootCommand;
}
