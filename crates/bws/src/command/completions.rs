use clap::CommandFactory;
use clap_complete::Shell;
use color_eyre::eyre::Result;

use crate::Cli;

pub(crate) fn execute(shell: Option<Shell>) -> Result<()> {
    let Some(shell) = shell.or_else(Shell::from_env) else {
        eprintln!("Couldn't autodetect a valid shell. Run `bws completions --help` for more info.");
        std::process::exit(1);
    };

    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());

    Ok(())
}
