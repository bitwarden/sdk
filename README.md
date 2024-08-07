# Bitwarden SDK

This repository houses the Bitwarden SDKs. We currently provide a public Secrets Manager SDK and an
internal SDK for the Bitwarden Password Manager which is used for the native mobile applications.
The SDK is written in Rust and provides a Rust API, CLI and various language bindings.

### Disclaimer

The password manager SDK is not intended for public use and is not supported by Bitwarden at this
stage. It is solely intended to centralize the business logic and to provide a single source of
truth for the internal applications. As the SDK evolves into a more stable and feature complete
state we will re-evaluate the possibility of publishing stable bindings for the public. **The
password manager interface is unstable and will change without warning.**

# We're Hiring!

Interested in contributing in a big way? Consider joining our team! We're hiring for many positions.
Please take a look at our [Careers page](https://bitwarden.com/careers/) to see what opportunities
are currently open as well as what it's like to work at Bitwarden.

## Getting Started

### Linux / Mac / Windows

```bash
cargo build
```

### Windows on ARM

To build, you will need the following in your PATH:

- [Python](https://www.python.org)
- [Clang](https://clang.llvm.org)
  - We recommend installing this via
    [Visual Studio](https://learn.microsoft.com/en-us/cpp/build/clang-support-msbuild?view=msvc-170#install-1)

## Crates

The project is structured as a monorepo using cargo workspaces. Some of the more noteworthy crates
are:

- [`bitwarden`](./crates/bitwarden/): Rust friendly API for interacting with the secrets manager.
- [`bitwarden-api-api`](./crates/bitwarden-api-api/): Auto-generated API bindings for the API
  server.
- [`bitwarden-api-identity`](./crates/bitwarden-api-identity/): Auto-generated API bindings for the
  Identity server.
- [`bitwarden-c`](./crates/bitwarden-c/): C bindings for FFI interop.
- [`bitwarden-json`](./crates/bitwarden-json/): JSON wrapper around the `bitwarden` crate. Powers
  the other language bindings.
- [`bitwarden-napi`](./crates/bitwarden-napi/): Node-API bindings.
- [`bws`](./crates/bws/): CLI for interacting with the [Bitwarden Secrets Manager][secrets-manager].
  Review the [CLI documentation][bws-help].
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

Note: If you don't have the nightly toolchain installed, the `build-api.sh` script will install it
for you.

## Developer tools

This project recommends the use of certain developer tools, and also includes configurations for
them to make developers lives easier. The use of these tools is optional and they might require a
separate installation step.

The list of developer tools is:

- `Visual Studio Code`: We provide a recommended extension list which should show under the
  `Extensions` tab when opening this project with the editor. We also offer a few launch settings
  and tasks to build and run the SDK
- `bacon`: This is a CLI background code checker. We provide a configuration file with some of the
  most common tasks to run (`check`, `clippy`, `test`, `doc` - run `bacon -l` to see them all). This
  tool needs to be installed separately by running `cargo install bacon --locked`.
- `nexttest`: This is a new and faster test runner, capable of running tests in parallel and with a
  much nicer output compared to `cargo test`. This tool needs to be installed separately by running
  `cargo install cargo-nextest --locked`. It can be manually run using
  `cargo nextest run --all-features`

[secrets-manager]: https://bitwarden.com/products/secrets-manager/
[bws-help]: https://bitwarden.com/help/secrets-manager-cli/

## Cargo fmt

We use certain unstable features for formatting which require the nightly version of cargo-fmt.

To install:

```
rustup component add rustfmt --toolchain nightly
```

To run:

```
cargo +nightly fmt
```

## Contribute

Code contributions are welcome! Please commit any pull requests against the `main` branch. Learn
more about how to contribute by reading the
[Contributing Guidelines](https://contributing.bitwarden.com/contributing/). Check out the
[Contributing Documentation](https://contributing.bitwarden.com/) for how to get started with your
first contribution.

Security audits and feedback are welcome. Please open an issue or email us privately if the report
is sensitive in nature. You can read our security policy in the [`SECURITY.md`](SECURITY.md) file.
We also run a program on [HackerOne](https://hackerone.com/bitwarden).

No grant of any rights in the trademarks, service marks, or logos of Bitwarden is made (except as
may be necessary to comply with the notice requirements as applicable), and use of any Bitwarden
trademarks must comply with
[Bitwarden Trademark Guidelines](https://github.com/bitwarden/server/blob/main/TRADEMARK_GUIDELINES.md).
