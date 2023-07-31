# Generate an xcframework for the Swift bindings.

# Cleanup dirs
rm -r BitwardenFFI.xcframework
rm -r tmp

mkdir tmp

cargo run -p uniffi-bindgen generate ../src/sdk.udl --language swift --out-dir tmp/bindings

# Move generated swift bindings
mv ./tmp/bindings/BitwardenSDK.swift ./Sources/BitwardenSdk/

# Massage the generated files to fit xcframework
mkdir tmp/Headers
mv ./tmp/bindings/BitwardenFFI.h ./tmp/Headers/
mv ./tmp/bindings/BitwardenFFI.modulemap ./tmp/Headers/module.modulemap

# Build native library
cargo build --package bitwarden-uniffi --target aarch64-apple-ios-sim --release

# Build xcframework
xcodebuild -create-xcframework \
  -library ../../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.dylib \
  -headers ./tmp/Headers \
  -output ./BitwardenFFI.xcframework

# Cleanup temporary files
rm -r tmp
