import { Command } from "commander";
import * as program from "commander";
import * as benny from "benny";
import { decrypt as forgeDecrypt } from "../crypto_performance/forge";
import {
  decrypt as sdkDecrypt,
  decryptDirect as sdkDecryptDirect,
} from "../crypto_performance/rust";
import { parseIntOption } from './option_parsing';
import { decrypt as webDecrypt, makeDerivedKey } from '../crypto_performance/web_crypto';

const librariesOption = new program.Option(
  "-l, --libraries <libraries...>",
  "Libraries to test"
).choices(["forge", "web", "sdk", "sdkCommand"]);
const iterationsOption = new program.Option(
  "-i, --iterations <iterations>",
  "Number of iterations"
).argParser(parseIntOption).default("1");
const sdkIterationOption = new program.Option(
  "--sdk-iterations <iterations>",
  "Number of iterations"
).argParser(parseIntOption).default("10000");

export function register_decrypt(rootCommand: Command) {
  rootCommand
    .command("decrypt")
    .addOption(librariesOption)
    .addOption(iterationsOption)
    .addOption(sdkIterationOption)
    .action(async (options: program.OptionValues) => {
      const libraries = options.libraries;
      const iterations = options.iterations as number;
      const sdkIterations = options.sdkIterations as number;

      const bennyOperations = []
      if (libraries.includes("forge")) {
        bennyOperations.push(benny.add("Forge", async () => {
          for (let i = 0; i < iterations; i++) {
            await forgeDecrypt();
          }
        }));
      }

      if (libraries.includes("web")) {
        const webCryptoKey = await makeDerivedKey(5000);
        bennyOperations.push(benny.add("WebCrypto", async () => {
          for (let i = 0; i < iterations; i++) {
            await webDecrypt(webCryptoKey);
          }
        }));
      }

      if (libraries.includes("sdk")) {
        bennyOperations.push(benny.add(`sdk (${sdkIterations} decryptions per op)`, async () => {
          await sdkDecrypt(sdkIterations);
        }));
      }

      if (libraries.includes("sdkCommand")) {
        bennyOperations.push(benny.add(`sdkCommand (${sdkIterations} decryptions per op)`, async () => {
          await sdkDecryptDirect(sdkIterations);
        }));
      }

      bennyOperations.push(benny.cycle());
      bennyOperations.push(benny.complete());

      await benny.suite("Decrypt", ...bennyOperations);
    });
  return rootCommand;
}
