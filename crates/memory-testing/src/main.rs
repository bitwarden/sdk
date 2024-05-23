use std::{env, io::Read, path::Path, process};

use bitwarden_crypto::{MasterKey, SensitiveString, SensitiveVec, SymmetricCryptoKey};

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
    let mut asymmetric_keys = Vec::new();
    let mut master_keys = Vec::new();
    let mut strings = Vec::new();

    for case in cases.cases {
        match case.command {
            memory_testing::CaseCommand::SymmetricKey { key } => {
                let key = SensitiveString::new(Box::new(key));
                let key = SymmetricCryptoKey::try_from(key).unwrap();
                symmetric_keys.push((key.to_vec(), key));
            }
            memory_testing::CaseCommand::AsymmetricKey { private_key } => {
                let private_key = SensitiveString::new(Box::new(private_key));
                let key = bitwarden_crypto::AsymmetricCryptoKey::from_pem(private_key).unwrap();
                asymmetric_keys.push(key);
            }
            memory_testing::CaseCommand::MasterKey {
                password,
                email,
                kdf,
            } => {
                let password: SensitiveVec = SensitiveString::new(Box::new(password)).into();
                let key = MasterKey::derive(&password, email.as_bytes(), &kdf).unwrap();
                let hash = key
                    .derive_master_key_hash(
                        &password,
                        bitwarden_crypto::HashPurpose::ServerAuthorization,
                    )
                    .unwrap();

                master_keys.push((key, hash));
            }
            memory_testing::CaseCommand::String { parts } => {
                let s = parts.join(" ");
                strings.push(s);
            }
        }
    }

    // Make a memory dump before the variables are freed
    wait_for_dump();

    // Put all the variables through a black box to prevent them from being optimized out before we
    // get to this point, and then drop them
    let _ = std::hint::black_box((
        test_string,
        symmetric_keys,
        asymmetric_keys,
        master_keys,
        strings,
    ));

    // After the variables are dropped, we want to make another dump
    wait_for_dump();

    println!("Done!")
}
