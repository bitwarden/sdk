import { Command } from "commander";
import * as program from "commander";
import * as benny from "benny";
import { encrypt as forgeEncrypt } from "../crypto_performance/forge";
import {
  encryptDirect as sdkEncryptDirect,
  encrypt as sdkEncrypt,
  normalizeRustResult,
} from "../crypto_performance/rust";
import { parseIntOption } from './option_parsing';
import { makeDerivedKey, encrypt as webEncrypt } from '../crypto_performance/web_crypto';

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

export function register_encrypt(rootCommand: Command) {
  rootCommand
    .command("encrypt")
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
            await forgeEncrypt(i.toString());
          }
        }));
      }

      if (libraries.includes("web")) {
        bennyOperations.push(benny.add("WebCrypto", async () => {
          const webCryptoKey = await makeDerivedKey(5000);
          for (let i = 0; i < iterations; i++) {
            await webEncrypt(i.toString(), webCryptoKey);
          }
        }));
      }

      if (libraries.includes("sdk")) {
        bennyOperations.push(benny.add(`sdk (${sdkIterations} encryptions per op)`, async () => {
          await sdkEncryptDirect(sdkIterations);
        }));
      }

      if (libraries.includes("sdkCommand")) {
        bennyOperations.push(benny.add(`sdkCommand (${sdkIterations} encryptions per op)`, async () => {
          await sdkEncrypt(sdkIterations);
        }));
      }

      bennyOperations.push(benny.cycle());
      bennyOperations.push(benny.complete());

      await benny.suite("Encrypt", ...bennyOperations);
    });
  return rootCommand;
}
