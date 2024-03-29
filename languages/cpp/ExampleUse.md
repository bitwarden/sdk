# EXAMPLES


## PREREQUISITES

### BITWARDEN Libraries
One should have two libraries at the same path:
- `BitwardeClient`
- `bitwarden_c`

It should look like `libBitwardeClient.dylib` and `libbitwarden_c.dylib` for the macOS.

For Linux: `libBitwardeClient.so` and `libbitwarden_c.so`
For Windows: `BitwardeClient.dll` and `bitwarden_c.dll`

### INCLUDE directory

`include` directory contains:
- `BitwardenLibrary.h`
- `BitwardenClient.h`
- `BitwardenSettings.h`
- `CommandRunner.h`
- `Projects.h`
- `Secrets.h`
- `schemas.hpp`

### Other libraries
- `nlohmann-json` (https://github.com/nlohmann/json) 
- `boost` (https://www.boost.org/)


### COMPILING

One could use g++/clang++ for compiling.
Example of the folder structure (macOS):

--root
    --build
        `libBitwardenClient.dylib`
        `libbitwarden_c.dylib`
    --include
        --`BitwardenLibrary.h`
        --`BitwardenClient.h`
        --`BitwardenSettings.h`
        --`CommandRunner.h`
        --`Projects.h`
        --`Secrets.h`
        --`schemas.hpp`
    --examples
        --`Wrapper.cpp`


1. $ export ACCESS_TOKEN=<"access-token">
2. $ export ORGANIZATION_ID=<"organization-id">
3. $ export DYLD_LIBRARY_PATH=/path/to/your/library:$DYLD_LIBRARY_PATH

The last step is neccessary to add the path for the dynamic library (macOS).
For the Linux one should use:
$ export LD_LIBRARY_PATH=/path/to/your/library:$LD_LIBRARY_PATH
For the Windows:
$ set PATH=%PATH%;C:\path\to\your\library

4. $ cd examples
5. $ clang++ -std=c++20 -I../include -I/path/to/include/nlohmann  -I/path/to/include/boost -L../build/ -o MyBitwardenApp Wrapper.cpp -lBitwardenClient -ldl

for Windows `-ldl` should be excluded,

The result is `MyBitwardenApp` in the `examples` directory, and one can run it from the `examples` directory:

6. $ ./MyBitwardenApp
