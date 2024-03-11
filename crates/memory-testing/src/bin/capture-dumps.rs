use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

fn dump_process_to_bytearray(pid: u32, output_dir: &Path, output_name: &Path) -> io::Result<u64> {
    Command::new("gcore")
        .args(["-a", &pid.to_string()])
        .output()?;

    let core_path = format!("core.{}", pid);
    let output_path = output_dir.join(output_name);
    let len = fs::copy(&core_path, output_path)?;
    fs::remove_file(&core_path)?;
    Ok(len)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: ./capture_dumps <binary_path> <base_dir>");
        std::process::exit(1);
    }

    let binary_path = &args[1];
    let base_dir: &Path = args[2].as_ref();

    println!("Memory dump capture script started");

    let mut proc = Command::new(binary_path)
        .arg(base_dir)
        .stdout(Stdio::inherit())
        .stdin(Stdio::piped())
        .spawn()?;
    let id = proc.id();
    println!("Started memory testing process with PID: {}", id);
    let stdin = proc.stdin.as_mut().expect("Valid stdin");

    // Wait a bit for it to process
    sleep(Duration::from_secs(3));

    // Dump the process before the variables are freed
    let initial_core =
        dump_process_to_bytearray(id, &base_dir.join("output"), "initial_dump.bin".as_ref())?;
    println!("Initial core dump file size: {}", initial_core);

    stdin.write_all(b".")?;
    stdin.flush()?;

    // Wait a bit for it to process
    sleep(Duration::from_secs(1));

    // Dump the process after the variables are freed
    let final_core =
        dump_process_to_bytearray(id, &base_dir.join("output"), "final_dump.bin".as_ref())?;
    println!("Final core dump file size: {}", final_core);

    stdin.write_all(b".")?;
    stdin.flush()?;

    // Wait for the process to finish and print the output
    let output = proc.wait()?;
    println!("Return code: {}", output);

    std::process::exit(output.code().unwrap_or(1));
}
