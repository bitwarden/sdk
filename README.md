# Bitwarden Secrets Manager SDK

This repository houses the Bitwarden Secrets Manager SDK. The core SDK is written in Rust and
provides a Rust API, CLI and Node-API bindings. In the future more language bindings might be added.

# We're Hiring!

Interested in contributing in a big way? Consider joining our team! We're hiring for many positions.
Please take a look at our [Careers page](https://bitwarden.com/careers/) to see what opportunities
are currently open as well as what it's like to work at Bitwarden.

## Getting Started

```bash
cargo build
```

## Crates

The project is structured as a monorepo using cargo workspaces.

- [`bitwarden`](./crates/bitwarden/): Rust friendly API for interacting with the secrets manager.
- [`bitwarden-api-api`](./crates/bitwarden-api-api/): Auto-generated API bindings for the API
  server.
- [`bitwarden-api-identity`](./crates/bitwarden-api-identity/): Auto-generated API bindings for the
  Identity server.
- [`bitwarden-c`](./crates/bitwarden-c/): C bindings for FFI interop.
- [`bitwarden-json`](./crates/bitwarden-json/): JSON wrapper around the `bitwarden` crate. Powers
  the other language bindings.
- [`bitwarden-napi`](./crates/bitwarden-napi/): Node-API bindings.
- [`bws`](./crates/bws/): CLI for interacting with the secrets manager.
- [`sdk-schemas`](./crates/sdk-schemas/): Generator for the _json schemas_.

## Schemas

To minimize the amount of work required to support additional bindings the project is structured
around a `json` based API. With every binding only needing to implement one method, namely
`run_command`.

To ensure type safety in the API, _json schemas_ are generated from the rust structs in `bitwarden`
using [schemars](https://crates.io/crates/schemars). The _json schemas_ are later used to generate
the API bindings for each language using [QuickType](https://github.com/quicktype/quicktype).

```bash
npm run schemas
```

## API Bindings

We autogenerate the server bindings using
[openapi-generator](https://github.com/OpenAPITools/openapi-generator). To do this we first need to
build the internal swagger documentation.

### Swagger generation

The first step is to generate the swagger documents from the server repository.

```bash
# src/Api
dotnet swagger tofile --output ../../api.json .\bin\Debug\net6.0\Api.dll internal

# src/Identity
dotnet swagger tofile --output ../../identity.json .\bin\Debug\net6.0\Identity.dll v1
```

### OpenApi Generator

Runs from the root of the SDK project.

```bash
npx openapi-generator-cli generate `
    -i ../server/api.json `
    -g rust `
    -o crates/bitwarden-api-api `
    --package-name bitwarden-api-api `
    -t ./support/openapi-template `
    --additional-properties=packageVersion=1.0.0

npx openapi-generator-cli generate `
    -i ../server/identity.json `
    -g rust `
    -o crates/bitwarden-api-identity `
    --package-name bitwarden-api-identity `
    -t ./support/openapi-template `
    --additional-properties=packageVersion=1.0.0
```

OpenApi Generator works using templates, we have customized our templates to work better with our
codebase.

- https://github.com/OpenAPITools/openapi-generator/issues/10977
- https://github.com/OpenAPITools/openapi-generator/issues/12464

There is also a scenario where we have a negative integer enum which completely breaks the openapi
generation. In that case we excluded the file from being generated and manually patched it.
`crates/bitwarden-api-api/src/models/organization_user_status_type.rs`

The hope going forward is that we can continue to use the generator with minimal manual
intervention.

### WASM - WebAssembly

Install `wasm32-unknown-unknown` rust target.

```bash
rustup target add wasm32-unknown-unknown
```

Install wasm-bindgen-cli

```bash
cargo install -f wasm-bindgen-cli
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
```
