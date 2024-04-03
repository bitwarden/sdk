include!("src/cli.rs");

fn main() -> Result<(), std::io::Error> {
    use std::{env, fs, path::Path};

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("manpages");
    fs::create_dir_all(&path).unwrap();

    let cmd = <Cli as clap::CommandFactory>::command();
    clap_mangen::generate_to(cmd, &path)?;

    println!("cargo:warning=man files generated: {path:?}");

    Ok(())
}
