include!("src/cli.rs");

fn main() -> Result<(), std::io::Error> {
    use std::{env, fs, path::Path};

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR exists");
    let path = Path::new(&out_dir).join("manpages");
    fs::create_dir_all(&path).expect("OUT_DIR is writable");

    let cmd = <Cli as clap::CommandFactory>::command();
    clap_mangen::generate_to(cmd, &path)?;

    Ok(())
}
