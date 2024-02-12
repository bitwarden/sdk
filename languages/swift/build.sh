# Generate an xcframework for the Swift bindings.

# Cleanup dirs
rm -r BitwardenFFI.xcframework
rm -r tmp

mkdir tmp
mkdir -p tmp/target/universal-ios-sim/release

# Build native library
export IPHONEOS_DEPLOYMENT_TARGET="13.0"
export RUSTFLAGS="-C link-arg=-Wl,-application_extension"
cargo build --package bitwarden-uniffi --target aarch64-apple-ios-sim --release
cargo build --package bitwarden-uniffi --target aarch64-apple-ios --release
cargo build --package bitwarden-uniffi --target x86_64-apple-ios --release

# Create universal libraries
lipo -create ../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.a \
  ../../target/x86_64-apple-ios/release/libbitwarden_uniffi.a \
  -output ./tmp/target/universal-ios-sim/release/libbitwarden_uniffi.a

# Generate swift bindings
cargo run -p uniffi-bindgen generate \
  ../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.dylib \
  --library \
  --language swift \
  --no-format \
  --out-dir tmp/bindings

# Move generated swift bindings
mv ./tmp/bindings/*.swift ./Sources/BitwardenSdk/

# Massage the generated files to fit xcframework
mkdir tmp/Headers
mv ./tmp/bindings/*.h ./tmp/Headers/
cat ./tmp/bindings/*.modulemap > ./tmp/Headers/module.modulemap

# Build xcframework
xcodebuild -create-xcframework \
  -library ../../target/aarch64-apple-ios/release/libbitwarden_uniffi.a \
  -headers ./tmp/Headers \
  -library ./tmp/target/universal-ios-sim/release/libbitwarden_uniffi.a \
  -headers ./tmp/Headers \
  -output ./BitwardenFFI.xcframework

# Cleanup temporary files
rm -r tmp
