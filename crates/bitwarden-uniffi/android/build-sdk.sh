cargo run -p uniffi-bindgen generate ../src/sdk.udl --language kotlin --out-dir sdk/src/main/java
cargo ndk -t arm64-v8a -o sdk/src/main/jniLibs --manifest-path ../Cargo.toml build
