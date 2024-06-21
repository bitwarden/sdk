# EXAMPLES

## PREREQUISITES

### BITWARDEN Libraries

Have the two Bitwarden libraries at the same path:

- `BitwardeClient`
- `bitwarden_c`

For each OS the library files will be the following:

- macOS: `libBitwardeClient.dylib` and `libbitwarden_c.dylib`
- Linux: `libBitwardeClient.so` and `libbitwarden_c.so`
- Windows:  `BitwardeClient.dll` and `bitwarden_c.dll`

Follow the [cmake build guide](CMakeBuild.md) to create the libraries locally.

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

- `nlohmann-json` (<https://github.com/nlohmann/json>)
- `boost` (<https://www.boost.org/>)

### COMPILING

Use g++/clang++ for compiling.

Example of the folder structure (macOS):

```text
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
```

Add the environment path for the Bitwarden libraries.

For macOS:

```bash
export DYLD_LIBRARY_PATH=/path/to/your/library:$DYLD_LIBRARY_PATH
```

For Linux:

```bash
export LD_LIBRARY_PATH=/path/to/your/library:$LD_LIBRARY_PATH
```

For Windows:

```shell
 set PATH=%PATH%;C:\path\to\your\library
```

Export environment variables used in `Wrapper.cpp`:

```bash
export ACCESS_TOKEN=<"access-token">
export ORGANIZATION_ID=<"organization-id">
export API_URL=http://localhost:4000
export IDENTITY_URL=http://localhost:33656
```

Compile:

```bash
cd examples
clang++ -std=c++20 -I../include -I/path/to/include/nlohmann  -I/path/to/include/boost -L../build/ -o MyBitwardenApp Wrapper.cpp -lBitwardenClient -ldl
```

for Windows `-ldl` should be excluded,

The result is `MyBitwardenApp` in the `examples` directory, and one can run it from the `examples` directory:

```bash
./MyBitwardenApp
```

## Example

### macOS

Export:

```bash
export DYLD_LIBRARY_PATH=/path/to/your/library:$DYLD_LIBRARY_PATH
export ACCESS_TOKEN=<"access-token">
export ORGANIZATION_ID=<"organization-id">
export API_URL=http://localhost:4000
export IDENTITY_URL=http://localhost:33656
```

Compile:

```bash
clang++ -std=c++20 -I../include -I/opt/homebrew/include -L../build/ -o MyBitwardenApp Wrapper.cpp -lBitwardenClient -ldl
```
