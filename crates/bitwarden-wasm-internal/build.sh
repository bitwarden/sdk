# Move to the root of the repository
cd "$(dirname "$0")"
cd ../../

if [ "$1" != "-r" ]; then
  # Dev
  cargo build -p bitwarden-wasm-internal --target wasm32-unknown-unknown
  wasm-bindgen --target bundler --out-dir languages/js/sdk-internal ./target/wasm32-unknown-unknown/debug/bitwarden_wasm_internal.wasm
  wasm-bindgen --target nodejs --out-dir languages/js/sdk-internal/node ./target/wasm32-unknown-unknown/debug/bitwarden_wasm_internal.wasm
else
  # Release
  cargo build -p bitwarden-wasm-internal --target wasm32-unknown-unknown --release
  wasm-bindgen --target bundler --out-dir languages/js/sdk-internal ./target/wasm32-unknown-unknown/release/bitwarden_wasm_internal.wasm
  wasm-bindgen --target nodejs --out-dir languages/js/sdk-internal/node ./target/wasm32-unknown-unknown/release/bitwarden_wasm_internal.wasm
fi

# Format
npx prettier --write ./languages/js/sdk-internal

# Optimize size
wasm-opt -Os ./languages/js/sdk-internal/bitwarden_wasm_internal_bg.wasm -o ./languages/js/sdk-internal/bitwarden_wasm_internal_bg.wasm
wasm-opt -Os ./languages/js/sdk-internal/node/bitwarden_wasm_internal_bg.wasm -o ./languages/js/sdk-internal/node/bitwarden_wasm_internal_bg.wasm

# Transpile to JS
wasm2js ./languages/js/sdk-internal/bitwarden_wasm_internal_bg.wasm -o ./languages/js/sdk-internal/bitwarden_wasm_internal_bg.wasm.js
npx terser ./languages/js/sdk-internal/bitwarden_wasm_internal_bg.wasm.js -o ./languages/js/sdk-internal/bitwarden_wasm_internal_bg.wasm.js
