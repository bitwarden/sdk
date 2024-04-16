use std::{env, io::Read, path::Path, process};

use bitwarden_crypto::{SensitiveString, SymmetricCryptoKey};

fn wait_for_dump() {
    println!("Waiting for dump...");
    std::io::stdin().read_exact(&mut [1u8]).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./memory-testing <base_dir>");
        process::exit(1);
    }
    let base_dir: &Path = args[1].as_ref();

    let test_string = String::from(memory_testing::TEST_STRING);

    let cases = memory_testing::load_cases(base_dir);

    let mut symmetric_keys = Vec::new();
    let mut symmetric_keys_as_vecs = Vec::new();

    for case in cases.symmetric_key {
        let key = SensitiveString::new(Box::new(case.key));
        let key = SymmetricCryptoKey::try_from(key).unwrap();
        symmetric_keys_as_vecs.push(key.to_vec());
        symmetric_keys.push(key);
    }

    // Make a memory dump before the variables are freed
    wait_for_dump();

    // Use all the variables so the compiler doesn't decide to remove them
    println!("{test_string} {symmetric_keys:?} {symmetric_keys_as_vecs:?}");

    drop(symmetric_keys);
    drop(symmetric_keys_as_vecs);

    // After the variables are dropped, we want to make another dump
    wait_for_dump();

    println!("Done!")
}
