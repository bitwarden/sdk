# Bitwarden-wasm

Requirements:

- `wasm32-unknown-unknown` rust target.
- `wasm-bindgen-cli` installed.
- `binaryen` installed for `wasm-opt` and `wasm2js`.

```bash
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
brew install binaryen
```

#### Build

```bash
# dev
cargo build -p bitwarden-wasm -p bitwarden --target wasm32-unknown-unknown --features wasm-bindgen
wasm-bindgen --target bundler --out-dir languages/js/wasm ./target/wasm32-unknown-unknown/debug/bitwarden_wasm.wasm
wasm-bindgen --target nodejs --out-dir languages/js/wasm/node ./target/wasm32-unknown-unknown/debug/bitwarden_wasm.wasm

# release
cargo build -p bitwarden -p bitwarden-wasm --target wasm32-unknown-unknown --features wasm-bindgen --release
wasm-bindgen --target bundler --out-dir languages/js/wasm ./target/wasm32-unknown-unknown/release/bitwarden_wasm.wasm
wasm-bindgen --target nodejs --out-dir languages/js/wasm/node ./target/wasm32-unknown-unknown/release/bitwarden_wasm.wasm

# Optimize size
wasm-opt -Os ./languages/js/wasm/bitwarden_wasm_bg.wasm -o ./languages/js/wasm/bitwarden_wasm_bg.wasm
wasm-opt -Os ./languages/js/wasm/node/bitwarden_wasm_bg.wasm -o ./languages/js/wasm/node/bitwarden_wasm_bg.wasm

# Transpile to JS
wasm2js ./languages/js/wasm/bitwarden_wasm_bg.wasm -o ./languages/js/wasm/bitwarden_wasm_bg.wasm.js
npx terser ./languages/js/wasm/bitwarden_wasm_bg.wasm.js -o ./languages/js/wasm/bitwarden_wasm_bg.wasm.js
```
