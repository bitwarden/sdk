use std::{env, fmt::Display, io, path::Path, process};

use memory_testing::*;

fn find_subarrays(needle: &[u8], haystack: &[u8]) -> Vec<usize> {
    let needle_len = needle.len();
    let haystack_len = haystack.len();
    let mut subarrays = vec![];

    if needle_len == 0 || haystack_len == 0 || needle_len > haystack_len {
        return vec![];
    }

    for i in 0..=(haystack_len - needle_len) {
        if &haystack[i..i + needle_len] == needle {
            subarrays.push(i);
        }
    }

    subarrays
}

const OK: &str = "✅";
const FAIL: &str = "❌";

fn comma_sep(nums: &[usize]) -> String {
    nums.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ")
}

fn add_row<N: Display>(
    table: &mut comfy_table::Table,
    name: N,
    initial_pos: &[usize],
    final_pos: &[usize],
    ok_cond: bool,
) -> bool {
    table.add_row(vec![
        name.to_string(),
        comma_sep(initial_pos),
        comma_sep(final_pos),
        if ok_cond {
            OK.to_string()
        } else {
            FAIL.to_string()
        },
    ]);
    !ok_cond
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./analyze-dumps <base_dir>");
        process::exit(1);
    }
    let base_dir: &Path = args[1].as_ref();

    println!("Memory testing script started");

    let initial_core = std::fs::read(base_dir.join("output/initial_dump.bin"))?;
    let final_core = std::fs::read(base_dir.join("output/final_dump.bin"))?;

    let mut error = false;
    let mut table = comfy_table::Table::new();
    table.set_header(vec!["Name", "Initial", "Final", "OK"]);

    let cases = memory_testing::load_cases(base_dir);

    let test_string: Vec<u8> = TEST_STRING.as_bytes().to_vec();
    let test_initial_pos = find_subarrays(&test_string, &initial_core);
    let test_final_pos = find_subarrays(&test_string, &final_core);

    error |= add_row(
        &mut table,
        "Test String",
        &test_initial_pos,
        &test_final_pos,
        !test_final_pos.is_empty(),
    );

    if test_initial_pos.is_empty() {
        println!("ERROR: Test string not found in initial core dump, is the dump valid?");
        error = true;
    }

    for (idx, case) in cases.symmetric_key.iter().enumerate() {
        let key_part: Vec<u8> = hex::decode(&case.decrypted_key_hex).unwrap();
        let mac_part: Vec<u8> = hex::decode(&case.decrypted_mac_hex).unwrap();
        let key_in_b64: Vec<u8> = case.key.as_bytes().to_vec();

        let key_initial_pos = find_subarrays(&key_part, &initial_core);
        let mac_initial_pos = find_subarrays(&mac_part, &initial_core);
        let b64_initial_pos = find_subarrays(&key_in_b64, &initial_core);

        let key_final_pos = find_subarrays(&key_part, &final_core);
        let mac_final_pos = find_subarrays(&mac_part, &final_core);
        let b64_final_pos = find_subarrays(&key_in_b64, &final_core);

        error |= add_row(
            &mut table,
            format!("Symm. Key, case {}", idx),
            &key_initial_pos,
            &key_final_pos,
            key_final_pos.is_empty(),
        );

        error |= add_row(
            &mut table,
            format!("Symm. MAC, case {}", idx),
            &mac_initial_pos,
            &mac_final_pos,
            mac_final_pos.is_empty(),
        );

        error |= add_row(
            &mut table,
            format!("Symm. Key in Base64, case {}", idx),
            &b64_initial_pos,
            &b64_final_pos,
            b64_final_pos.is_empty(),
        );
    }

    println!("{table}");

    process::exit(if error { 1 } else { 0 });
}
