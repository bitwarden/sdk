# CMAKE build

## INTRODUCTION

Cmake is used to build the c++ Bitwarden client library. Output should be placed in the build directory. The output contains two dynamic libraries: one that we are building `BitwardenClient` and another that the building library uses `bitwarden_c`.

## PREREQUISITES

- Cmake installed, minimum version 3.15
- `schemas.hpp` generated into `include` directory
- installed `nlohmann-json` library
- installed `boost` library

## BUILD commands

One should be in the root directory of the c++ wrapper (the same level where is CMakeLists.txt placed). Paths of the three libraries should be placed inside the cmake build command:

```bash
mkdir build
cd build
cmake .. -DNLOHMANN=/path/to/include/nlohmann -DBOOST=/path/to/include/boost -DTARGET=relative/path/to/libbitwarden_c
cmake --build .
```

## Example

### macOS

#### Install prerequisites

```bash
brew install cmake
brew install boost
brew install nlohmann-json
```

#### Build

```bash
mkdir -p build
cd build
cmake .. -DNLOHMANN=/opt/homebrew/include -DBOOST=/opt/homebrew/include -DTARGET=../../target/release/libbitwarden_c.dylib
cmake --build .
```
