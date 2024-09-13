# WASM SDK Instructions

## Steps

1. Build the wasm binding
  - Follow the instructions [here](../../crates/bitwarden-wasm/README.md)
  - `cd sdk/crates/bitwarden-wasm`
  - `./build.sh`
2. Change directory to the sdk-client and build it
  - `cd sdk/languages/js/sdk-client/`
  - `npm run build`
3. Change directory to the example directory and build it
  - `cd sdk/languages/js/example`
  - `npm run start`
