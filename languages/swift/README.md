# Swift

Ensure the necessary targets are installed.

```bash
rustup target install aarch64-apple-ios-sim
rustup target install aarch64-apple-ios
rustup target install x86_64-apple-ios
```

## Build

```bash
./build.sh
```

## Deploy

Checkout `https://github.com/bitwarden/sdk-swift`.

Copy the following files to the root of the sdk-swift repository.

- `BitwardenFFI.xcframework`
- `Sources/BitwardenSdk/BitwardenSDK.swift`

Push the modified files.
