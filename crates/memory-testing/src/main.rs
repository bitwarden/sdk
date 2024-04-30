use std::{env, io::Read, path::Path, process};

use bitwarden_crypto::{BitString, MasterKey, SensitiveString, SensitiveVec, SymmetricCryptoKey};
use zeroize::Zeroizing;

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

    for case in cases.symmetric_key {
        let key = SensitiveString::new(Box::new(case.key));
        let key = SymmetricCryptoKey::try_from(key).unwrap();
        symmetric_keys.push((key.to_vec(), key));
    }

    let mut master_keys = Vec::new();

    for case in cases.master_key {
        let password: SensitiveVec = SensitiveString::new(Box::new(case.password)).into();

        let key = MasterKey::derive(&password, case.email.as_bytes(), &case.kdf).unwrap();
        let hash = key
            .derive_master_key_hash(
                &password,
                bitwarden_crypto::HashPurpose::ServerAuthorization,
            )
            .unwrap();

        master_keys.push((key, hash));
    }

    let mut bit_strings = Vec::new();
    for case in cases.bit_string {
        let mut left = BitString::new(case.left);
        let right = BitString::new(case.right);
        left.push_str(right.as_str());

        bit_strings.push(left);
    }

    // Make a memory dump before the variables are freed
    wait_for_dump();

    // Use all the variables so the compiler doesn't decide to remove them
    let len = bit_strings.len();
    println!("{test_string} {symmetric_keys:?} {master_keys:?} {len:?} ");

    drop(symmetric_keys);
    drop(master_keys);
    drop(bit_strings);

    // After the variables are dropped, we want to make another dump
    wait_for_dump();

    println!("Done!")
}
