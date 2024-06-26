# Instructions

This guide is for developers who want to use the Bitwarden Go SDK module in their own Go projects.

The Bitwarden SDK is written in Rust, and we utilize FFI via [cgo](https://pkg.go.dev/cmd/cgo). We currently support static linking the Bitwarden Go module on Linux (x86-64 & arm64), Mac (x86-64 & arm64), and Windows (x86-64).

Please see the main [README](./README.md) and [example](./example/example.go) for examples.

## Linux

1. Make sure you have the following on your system:
    - [Go](https://go.dev/dl)
    - A C/C++ toolchain. We recommend the [MUSL toolchain](https://musl.cc). You can install this on most debian based systems with: `sudo apt install musl-tools`
2. Verify cgo is enabled and the `CC` and `CXX` flags are set with: `go env`.
    - You can enable cgo by setting the flag: `go env -w CGO_ENABLED=1`
    - If `CC` and `CXX` are not set, you can set them with: `go env -w CC=clang CXX=clang++`
3. Make sure you have added the Bitwarden Go SDK with go get: `go get github.com/bitwarden/sdk-go`
4. You can build your project with the appropriate libraries linked statically with: `go build -ldflags '-linkmode external -extldflags "-static -Wl,-unresolved-symbols=ignore-all"' -o myapp`, where `myapp` is the name of the output binary.

## Mac

1. Make sure you have the following on your system:
    - [Go](https://go.dev/dl)
    - A C/C++ toolchain. [Clang](https://clang.llvm.org/get_started.html) is the default Xcode compiler on Mac OS. The easiest way to ensure you have the toolchain is to install the Xcode Command Line tools.
        - You can install them with: `xcode-select -–install`
2. Verify cgo is enabled and the `CC` and `CXX` flags are set with: `go env`.
    - You can enable cgo by setting the flag: `go env -w CGO_ENABLED=1`
    - If `CC` and `CXX` are not set, you can set them with: `go env -w CC=clang CXX=clang++`
3. Make sure you have added the Bitwarden Go SDK with go get: `go get github.com/bitwarden/sdk-go`
4. Your project should build with: `go build`

## Windows

1. Make sure you have the following on your system:
    - [Go](https://go.dev/dl)
    - GCC via [MSYS2](https://www.msys2.org)
        - GCC is required on Windows according to [this cgo](https://go.dev/wiki/cgo) page. Microsoft recommends using the MinGW-w64 toolchain via [MSYS2](https://www.msys2.org) for up-to-date native builds of GCC [here](https://code.visualstudio.com/docs/cpp/config-mingw#_installing-the-mingww64-toolchain). MSYS2 installation instructions can be found [here](https://www.msys2.org).
        - After following the MSYS2 installation instructions, you should be able to run the following to verify everything is installed:
            - `gcc --version`
            - `g++ --version`
            - `gdb --version`
2. Verify cgo is enabled and the `CC` and `CXX` flags are set with: `go env`.
    - You can enable cgo by setting the flag: `go env -w CGO_ENABLED=1`
    - If `CC` and `CXX` are not set, you can set them with: `go env -w CC=clang CXX=clang++`
3. Make sure you have added the Bitwarden Go SDK with go get: `go get github.com/bitwarden/sdk-go`
4. Your project should build with: `go build`
