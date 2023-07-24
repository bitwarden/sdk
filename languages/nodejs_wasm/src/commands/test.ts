import { Command } from 'commander';
import * as program from "commander";
import * as benny from "benny";
import Bench from 'tinybench';


export function register_test(rootCommand: Command) {
  rootCommand.command("test")
    .action(async (options: program.OptionValues) => {
      await benny.suite("Benny",
        benny.add("should be 2 ops/sec", async () => {
          await new Promise((resolve) => setTimeout(resolve, 500));
        }),
        benny.add('No work, should score well', () => {
        }),
        benny.add('Should be 1000 ops/sec', async () => {
          await new Promise(r => setTimeout(r, 1)); // we wait 1ms :)
        }),
        benny.cycle(),
        benny.complete()
      );
    });
  return rootCommand;
}
