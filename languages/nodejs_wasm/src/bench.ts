import { Command } from 'commander';
import { register_pbkdf2 } from './commands/pkbdf2';
import { register_encrypt } from './commands/encrypt';
import { register_decrypt } from './commands/decrypt';

class Main {

  private program: Command;

  async init() {

  }
  async run() {
    await this.init();
    this.program = new Command();
    this.program.description("Bitwarden SDK Benchmarking");
    this.program.version("0.0.1");
    
    register_pbkdf2(this.program);
    register_encrypt(this.program);
    register_decrypt(this.program);

    await this.program.parseAsync();
  }
}

const main = new Main();
main.run();
