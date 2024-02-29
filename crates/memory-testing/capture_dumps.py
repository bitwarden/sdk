from os import remove
from shutil import copy2
from sys import argv
from subprocess import Popen, run, PIPE, STDOUT
from time import sleep


def read_file_to_byte_array(file_path):
    with open(file_path, "rb") as file:
        byte_array = bytearray(file.read())
    return byte_array


def dump_process_to_bytearray(pid, output):
    run(["gcore", "-a", str(pid)], capture_output=True, check=True)
    core_file = "core." + str(pid)
    core = read_file_to_byte_array(core_file)
    copy2(core_file, output)
    remove(core_file)
    return core


if len(argv) < 3:
    print("Usage: python3 capture_dumps.py <binary_path> <output_dir>")
    exit(1)

binary_path = argv[1]
output_dir = argv[2]

print("Memory dump capture script started")

proc = Popen(binary_path, stdout=PIPE, stderr=STDOUT, stdin=PIPE, text=True)
print("Started memory testing process with PID:", proc.pid)

# Wait a bit for it to process
sleep(1)

# Dump the process before the variables are freed
initial_core = dump_process_to_bytearray(proc.pid, output_dir + "/initial_dump.bin")
print("Initial core dump file size:", len(initial_core))

proc.stdin.write(".")
proc.stdin.flush()

# Wait a bit for it to process
sleep(1)

# Dump the process after the variables are freed
final_core = dump_process_to_bytearray(proc.pid, output_dir + "/final_dump.bin")
print("Final core dump file size:", len(final_core))

# Wait for the process to finish and print the output
stdout_data, _ = proc.communicate(input=".")
print("STDOUT:", repr(stdout_data))
print("Return code:", proc.wait())
