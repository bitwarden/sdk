#!/bin/env bash

rm -r bindings

mkdir bindings
mkdir bindings/swift
mkdir bindings/kotlin

rm -r dist
mkdir dist/ios

# Swift

cargo run -p uniffi-bindgen generate src/sdk.udl --language swift  --out-dir bindings/swift
cargo build --package bitwarden-uniffi --target aarch64-apple-ios-sim --release

mkdir bindings/swift/Headers

mv ./bindings/swift/bitwardenFFI.h ./bindings/swift/Headers/
mv ./bindings/swift/bitwardenFFI.modulemap ./bindings/swift/Headers/module.modulemap

xcodebuild -create-xcframework -library ../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.a -headers ./bindings/swift/Headers -output ./bindings/swift/BitwardenSdk.xcframework

cp src/bitwarden.swift platforms/apple/Hello/Sources/Hello

# Kotlin

cargo run -p uniffi-bindgen generate src/sdk.udl --language kotlin --out-dir bindings/kotlin
cargo ndk -t arm64-v8a -o ./bindings/kotlin/jniLibs build

# Copy files to android project
