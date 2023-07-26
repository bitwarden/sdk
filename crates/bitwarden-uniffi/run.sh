#!/usr/bin/env bash

rm -r bindings

mkdir bindings
mkdir bindings/swift
mkdir bindings/kotlin

rm -r dist
mkdir dist/ios

# Swift

cargo build --package bitwarden-uniffi --target aarch64-apple-ios-sim --release --all-features
cargo run -p uniffi-bindgen generate ../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.dylib --library --language swift --no-format --out-dir bindings/swift

mkdir bindings/swift/Headers

mv ./bindings/swift/bitwardenFFI.h ./bindings/swift/Headers/
mv ./bindings/swift/bitwardenFFI.modulemap ./bindings/swift/Headers/module.modulemap

xcodebuild -create-xcframework -library ../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.a -headers ./bindings/swift/Headers -output ./bindings/swift/BitwardenSdk.xcframework

cp src/bitwarden.swift platforms/apple/Hello/Sources/Hello

# Kotlin

# Use the iOS build to generate the bindings for Android to avoid an extra compilation
cargo run -p uniffi-bindgen generate ../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.dylib --library --language kotlin --no-format --out-dir bindings/kotlin
cargo ndk -t arm64-v8a -o ./bindings/kotlin/jniLibs build --all-features

# Copy files to android project
