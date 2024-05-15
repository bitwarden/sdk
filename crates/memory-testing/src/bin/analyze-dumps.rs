use std::{env, io, path::Path, process};

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

    if test_initial_pos.is_empty() {
        println!("ERROR: Test string not found in initial core dump, is the dump valid?");
        error = true;
    }

    for (idx, case) in cases.cases.iter().enumerate() {
        for lookup in &case.memory_lookups {
            let value = match &lookup.value {
                MemoryLookupValue::String { string } => string.as_bytes().to_vec(),
                MemoryLookupValue::Binary { hex } => hex::decode(hex).unwrap(),
            };

            let initial_pos = find_subarrays(&value, &initial_core);
            let final_pos = find_subarrays(&value, &final_core);

            let name = format!("{idx}: {} / {}", case.name, lookup.name);
            let ok_cond = final_pos.len() <= lookup.allowed_count.unwrap_or(0);

            table.add_row([
                name.as_str(),
                &comma_sep(&initial_pos),
                &comma_sep(&final_pos),
                if ok_cond { OK } else { FAIL },
            ]);

            if !ok_cond {
                error = true;
            }
        }
    }

    println!("{table}");

    process::exit(if error { 1 } else { 0 });
}
