# Generate an xcframework for the Swift bindings.

# Cleanup dirs
rm -r BitwardenFFI.xcframework
rm -r tmp

mkdir tmp

cargo run -p uniffi-bindgen generate ../src/sdk.udl --language swift --out-dir tmp/bindings

# Massage the generated files to fit xcframework
mkdir tmp/Headers
mv ./tmp/bindings/BitwardenFFI.h ./tmp/Headers/
mv ./tmp/bindings/BitwardenFFI.modulemap ./tmp/Headers/module.modulemap
mv ./tmp/bindings/BitwardenSDK.swift ./Sources/BitwardenSdk/

cargo build --package bitwarden-uniffi --target aarch64-apple-ios-sim --release

xcodebuild -create-xcframework \
  -library ../../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.dylib \
  -headers ./tmp/Headers \
  -output ./BitwardenFFI.xcframework

# Cleanup temporary files
rm -r tmp
