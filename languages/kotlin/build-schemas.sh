cargo run -p uniffi-bindgen generate ../../crates/bitwarden-uniffi/src/sdk.udl \
  --language kotlin \
  --out-dir sdk/src/main/java
