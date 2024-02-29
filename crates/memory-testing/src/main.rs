use std::{io::Read, str::FromStr};

use bitwarden_crypto::{AsymmetricCryptoKey, SymmetricCryptoKey};

fn main() {
    let now = std::time::Instant::now();

    let mut test_string = String::new();
    test_string.push_str("THIS IS USED TO CHECK THAT ");
    test_string.push_str("THE MEMORY IS DUMPED CORRECTLY");

    // In HEX:
    // KEY: 15f8 5554 ff1f 9852 1963 55a6 46cc cf99 1995 0b15 cd59 5709 7df3 eb6e 4cb0 4cfb
    // MAC: 4136 481f 8581 93f8 3f6c 5468 b361 7acf 7dfb a3db 2a32 5aa3 3017 d885 e5a3 1085
    let symm_key = SymmetricCryptoKey::from_str(
        "FfhVVP8fmFIZY1WmRszPmRmVCxXNWVcJffPrbkywTPtBNkgfhYGT+D9sVGizYXrPffuj2yoyWqMwF9iF5aMQhQ==",
    )
    .unwrap();

    let symm_key_vec = symm_key.to_vec();

    // Make a memory dump before the variables are freed
    println!("Waiting for initial dump at {:?} ...", now.elapsed());
    std::io::stdin().read_exact(&mut [1u8]).unwrap();
    println!("Dumped at {:?}!", now.elapsed());

    // Use all the variables so the compiler doesn't decide to remove them
    println!("{test_string} {symm_key:?} {symm_key_vec:?}");

    drop(test_string); // Note that this won't clear anything from the memory

    drop(symm_key);
    drop(symm_key_vec);

    // After the variables are dropped, we want to make another dump
    println!("Waiting for final dump at {:?} ...", now.elapsed());
    std::io::stdin().read_exact(&mut [1u8]).unwrap();
    println!("Dumped at {:?}!", now.elapsed());

    println!("Done!")
}
