from sys import argv
from typing import *


def find_subarrays(needle: bytearray, haystack: bytearray) -> List[int]:
    needle_len, haystack_len = len(needle), len(haystack)
    subarrays = []

    if needle_len == 0 or haystack_len == 0 or needle_len > haystack_len:
        return []

    for i in range(haystack_len - needle_len + 1):
        if haystack[i : i + needle_len] == needle:
            subarrays.append(i)

    return subarrays


# Check that I implemented this correctly lol
assert find_subarrays([1, 2, 3], [1, 2, 3, 4, 5]) == [0]
assert find_subarrays([1, 2, 3], [1, 2, 3, 4, 1, 2, 3, 5]) == [0, 4]
assert find_subarrays([1, 2, 3], [1, 2, 3]) == [0]
assert find_subarrays([1, 2, 3], [1, 2, 4, 3, 5]) == []


def read_file_to_byte_array(file_path: str) -> bytearray:
    with open(file_path, "rb") as file:
        return bytearray(file.read())


# ---------------------------------------------------------------------------


TEST_STRING = b"THIS IS USED TO CHECK THAT THE MEMORY IS DUMPED CORRECTLY"
SYMMETRIC_KEY = bytearray.fromhex(
    "15f8 5554 ff1f 9852 1963 55a6 46cc cf99 1995 0b15 cd59 5709 7df3 eb6e 4cb0 4cfb"
)
SYMMETRIC_MAC = bytearray.fromhex(
    "4136 481f 8581 93f8 3f6c 5468 b361 7acf 7dfb a3db 2a32 5aa3 3017 d885 e5a3 1085"
)
SYMMETRIC_B64 = bytearray.fromhex(
    "4666 6856 5650 3866 6d46 495a 5931 576d 5273 7a50 6d52 6d56 4378 584e 5756 634a 6666 5072 626b 7977 5450 7442 4e6b 6766 6859 4754 2b44 3973 5647 697a 5958 7250 6666 756a 3279 6f79 5771 4d77 4639 6946 3561 4d51 6851 3d3d"
)


# ---------------------------------------------------------------------------

if len(argv) < 2:
    print("Usage: python3 test.py <output_dir>")
    exit(1)

output_dir = argv[1]
print("Memory testing script started in", output_dir)

print("------------- Processing initial core dump -------------")

initial_core = read_file_to_byte_array(output_dir + "/initial_dump.bin")

key_initial_matches = find_subarrays(SYMMETRIC_KEY, initial_core)
mac_initial_matches = find_subarrays(SYMMETRIC_MAC, initial_core)
b64_initial_matches = find_subarrays(SYMMETRIC_B64, initial_core)
test_initial_matches = find_subarrays(TEST_STRING, initial_core)

print("-------------- Processing final core dump --------------")

final_core = read_file_to_byte_array(output_dir + "/final_dump.bin")

key_final_matches = find_subarrays(SYMMETRIC_KEY, final_core)
mac_final_matches = find_subarrays(SYMMETRIC_MAC, final_core)
b64_final_matches = find_subarrays(SYMMETRIC_B64, final_core)
test_final_matches = find_subarrays(TEST_STRING, final_core)


debug = True
if debug:
    print("-------------- Printing matches for debug --------------")
    print("Initial matches")
    print("    Key:", key_initial_matches)
    print("    MAC:", mac_initial_matches)
    print("    Base64 Key:", b64_initial_matches)
    print("    Test:", test_initial_matches)
    print("Final matches")
    print("    Key:", key_final_matches)
    print("    MAC:", mac_final_matches)
    print("    Base64 Key:", b64_final_matches)
    print("    Test:", test_final_matches)

print("------------------ Checking for leaks  -----------------")

error = False

if len(test_initial_matches) == 0:
    print("ERROR: Test string not found in initial core dump")
    error = True

if len(test_final_matches) > len(test_initial_matches):
    print(
        "ERROR: Test string found more times in final core dump than in initial core dump"
    )
    error = True

if len(key_final_matches) > 0:
    print(
        "ERROR: Symmetric key found in final core dump at positions:", key_final_matches
    )
    error = True

if len(mac_final_matches) > 0:
    print(
        "ERROR: Symmetric MAC found in final core dump at positions:", mac_final_matches
    )
    error = True

if error:
    print("Memory testing script finished with errors")
    exit(1)
else:
    print("Memory testing script finished successfully")
    exit(0)
