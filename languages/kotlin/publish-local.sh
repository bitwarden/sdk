#!/usr/bin/env bash
set -e

cd "$(dirname "$0")"

mkdir -p ./sdk/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64,x86}

# Build arm64 for emulator
cross build -p bitwarden-uniffi --release --target=aarch64-linux-android
mv ../../target/aarch64-linux-android/release/libbitwarden_uniffi.so ./sdk/src/main/jniLibs/arm64-v8a/libbitwarden_uniffi.so

# Generate latest bindings
./build-schemas.sh

# Publish to local maven
./gradlew sdk:publishToMavenLocal -Pversion=LOCAL
