# Generate an xcframework for the Swift bindings.

# Cleanup dirs
rm -r BitwardenFFI.xcframework
rm -r tmp

mkdir tmp
mkdir -p tmp/target/universal-ios-sim/release

# Build native library
cargo build --package bitwarden-uniffi --target aarch64-apple-ios-sim --release
cargo build --package bitwarden-uniffi --target aarch64-apple-ios --release
cargo build --package bitwarden-uniffi --target x86_64-apple-ios --release

# Create universal libraries
lipo -create ../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.dylib \
  ../../target/x86_64-apple-ios/release/libbitwarden_uniffi.dylib  \
  -output ./tmp/target/universal-ios-sim/release/libbitwarden_uniffi.dylib

# Generate swift bindings
cargo run -p uniffi-bindgen generate \
  ../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.dylib \
  --library \
  --language swift \
  --no-format \
  --out-dir tmp/bindings

# Move generated swift bindings
mv ./tmp/bindings/BitwardenSDK.swift ./Sources/BitwardenSdk/
mv ./tmp/bindings/BitwardenCore.swift ./Sources/BitwardenSdk/

# Massage the generated files to fit xcframework
mkdir tmp/Headers
mv ./tmp/bindings/BitwardenFFI.h ./tmp/Headers/
mv ./tmp/bindings/BitwardenCoreFFI.h ./tmp/Headers/
cat ./tmp/bindings/BitwardenFFI.modulemap ./tmp/bindings/BitwardenCoreFFI.modulemap > ./tmp/Headers/module.modulemap

# Build xcframework
xcodebuild -create-xcframework \
  -library ../../target/aarch64-apple-ios/release/libbitwarden_uniffi.dylib \
  -headers ./tmp/Headers \
  -library ./tmp/target/universal-ios-sim/release/libbitwarden_uniffi.dylib \
  -headers ./tmp/Headers \
  -output ./BitwardenFFI.xcframework

# Cleanup temporary files
rm -r tmp
