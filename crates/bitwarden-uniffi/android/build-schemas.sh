cargo build --package bitwarden-uniffi --release --all-features
cargo run -p uniffi-bindgen generate \
  ../../../target/aarch64-linux-android/release/libbitwarden_uniffi.so \
  --library \
  --language kotlin \
  --no-format \
  --out-dir sdk/src/main/java
