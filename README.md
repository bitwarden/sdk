# Bitwarden SDK

This repository houses the Bitwarden SDKs. We currently provide a public Secrets Manager SDK and an
internal SDK for the Bitwarden Password Manager which is used for the native mobile applications.
The SDK is written in Rust and provides a Rust API, CLI and various language bindings.

### Disclaimer

The password manager SDK is not intended for public use and is not supported by Bitwarden at this
stage. It is solely intended to centralize the business logic and to provide a single source of
truth for the internal applications. As the SDK evolves into a more stable and feature complete
state we will re-evaluate the possibility of publishing stable bindings for the public.

# We're Hiring!

Interested in contributing in a big way? Consider joining our team! We're hiring for many positions.
Please take a look at our [Careers page](https://bitwarden.com/careers/) to see what opportunities
are currently open as well as what it's like to work at Bitwarden.

## Getting Started

```bash
cargo build
```

## Crates

The project is structured as a monorepo using cargo workspaces. Some of the more noteworthy crates
are:

- [`bitwarden`](./crates/bitwarden/): Rust friendly API for interacting with the secrets manager.
- [`bitwarden-api-api`](./crates/bitwarden-api-api/): Auto-generated API bindings for the API
  server.
- [`bitwarden-api-identity`](./crates/bitwarden-api-identity/): Auto-generated API bindings for the
  Identity server.
- [`bws`](./crates/bws/): CLI for interacting with the [Bitwarden Secrets Manager][secrets-manager].
  Review the [CLI documentation][bws-help].
- [`bitwarden-c`](./crates/bitwarden-c/): C bindings for FFI interop.
- [`bitwarden-json`](./crates/bitwarden-json/): JSON wrapper around the `bitwarden` crate. Powers
  the other language bindings.
- [`bitwarden-napi`](./crates/bitwarden-napi/): Node-API bindings.
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
dotnet swagger tofile --output ../../api.json ./bin/Debug/net8.0/Api.dll internal

# src/Identity
ASPNETCORE_ENVIRONMENT=development dotnet swagger tofile --output ../../identity.json ./bin/Debug/net8.0/Identity.dll v1
```

### OpenApi Generator

To generate a new version of the bindings run the following script from the root of the SDK project.

```bash
./support/build-api.sh
```

This project uses customized templates which lives in the `support/openapi-templates` directory.
These templates resolves some outstanding issues we've experienced with the rust generator. But we
strive towards modifying the templates as little as possible to ease future upgrades.

[secrets-manager]: https://bitwarden.com/products/secrets-manager/
[bws-help]: https://bitwarden.com/help/secrets-manager-cli/
