use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    process::{ChildStdin, ChildStdout, Command, Stdio},
};

fn dump_process_to_bytearray(pid: u32, output_dir: &Path, output_name: &Path) -> io::Result<u64> {
    let output = Command::new("gcore")
        .args(["-a", &pid.to_string()])
        .output()?;

    if !output.status.success() {
        return io::Result::Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to dump process: {:?}", output),
        ));
    }

    let core_path = format!("core.{}", pid);
    let output_path = output_dir.join(output_name);
    let len = fs::copy(&core_path, output_path)?;
    fs::remove_file(&core_path)?;
    Ok(len)
}

fn wait_dump_and_continue(
    stdin: &mut ChildStdin,
    stdout: &mut ChildStdout,
    id: u32,
    base_dir: &Path,
    name: &Path,
) -> Result<(), io::Error> {
    // Read the input from the process until we get the "Waiting for dump..." message
    // That way we know the process is ready to be dumped, and we don't need to just sleep a fixed
    // amount of time
    loop {
        let mut buf = [0u8; 1024];
        let read = stdout.read(&mut buf).unwrap();
        let buf_str = std::str::from_utf8(&buf[..read]).unwrap();
        if buf_str.contains("Waiting for dump...") {
            break;
        }
    }
    let dump_size = dump_process_to_bytearray(id, &base_dir.join("output"), name)?;
    println!("Got memory dump of file size: {}", dump_size);

    stdin.write_all(b".")?;
    stdin.flush()?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: ./capture_dumps <binary_path> <base_dir>");
        std::process::exit(1);
    }

    let binary_path = &args[1];
    let base_dir: &Path = args[2].as_ref();

    let mut proc = Command::new(binary_path)
        .arg(base_dir)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;
    let id = proc.id();
    println!("Started memory testing process with PID: {}", id);

    let stdin = proc.stdin.as_mut().expect("Valid stdin");
    let stdout = proc.stdout.as_mut().expect("Valid stdin");

    wait_dump_and_continue(stdin, stdout, id, base_dir, "initial_dump.bin".as_ref())?;
    wait_dump_and_continue(stdin, stdout, id, base_dir, "final_dump.bin".as_ref())?;

    // Wait for the process to finish and print the output
    let output = proc.wait()?;
    println!("Return code: {}", output);

    std::process::exit(output.code().unwrap_or(1));
}
