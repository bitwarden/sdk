# CMAKE build

## INTRODUCTION

Cmake is used to build the c++ Bitwarden client library. Output should be placed in the build directory. The output contains two dynamic libraries: one that we are building `BitwardenClient` and another that the building library uses `bitwarden_c`.

## PREREQUISITES

- Cmake installed, minimum version 3.15
- `schemas.cpp` generated into `include` directory
- installed `nlohmann-json` library
- installed `boost` library

## BUILD commands

One should be in the root directory of the c++ wrapper (the same level where is CMakeLists.txt placed). Paths of the three libraries should be placed iside the cmake build command:

$ mkdir build
$ cd build
$ cmake ..
# cmake --build .