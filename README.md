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
dotnet swagger tofile --output ../../api.json ./bin/Debug/net6.0/Api.dll internal

# src/Identity
ASPNETCORE_ENVIRONMENT=development dotnet swagger tofile --output ../../identity.json ./bin/Debug/net6.0/Identity.dll v1
```

### OpenApi Generator

Runs from the root of the SDK project.

```bash
npx openapi-generator-cli generate \
    -i ../server/api.json \
    -g rust \
    -o crates/bitwarden-api-api \
    --package-name bitwarden-api-api \
    -t ./support/openapi-template \
    --additional-properties=packageVersion=1.0.0

npx openapi-generator-cli generate \
    -i ../server/identity.json \
    -g rust \
    -o crates/bitwarden-api-identity \
    --package-name bitwarden-api-identity \
    -t ./support/openapi-template \
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

## Tests

Many of the SDK tests are based on encrypted data provided by the other Bitwarden clients. In order
to provide a consistent method of retrieving the data we provide a test account with user keys.

**Disclaimer:** The server typically encrypts and protects certain fields. In order to allow
accounts to be used on other servers this protection was explicitly removed from these data dumps.

### `test@bitwarden.com`

- Email: `test@bitwarden.com`
- Password: `asdfasdfasdf`
- PBKDF2: `600_000` iterations

```sql
INSERT INTO vault_dev.dbo.[User] (
  Id, Name, Email, EmailVerified, MasterPassword,
  MasterPasswordHint, Culture, SecurityStamp,
  TwoFactorProviders, TwoFactorRecoveryCode,
  EquivalentDomains, ExcludedGlobalEquivalentDomains,
  AccountRevisionDate, [Key], PublicKey,
  PrivateKey, Premium, PremiumExpirationDate,
  Storage, MaxStorageGb, Gateway, GatewayCustomerId,
  GatewaySubscriptionId, LicenseKey,
  CreationDate, RevisionDate, RenewalReminderDate,
  Kdf, KdfIterations, ReferenceData,
  ApiKey, ForcePasswordReset, UsesKeyConnector,
  FailedLoginCount, LastFailedLoginDate,
  AvatarColor, KdfMemory, KdfParallelism,
  LastPasswordChangeDate, LastKdfChangeDate,
  LastKeyRotationDate, LastEmailChangeDate
)
VALUES
  (
    N 'b1fd4bf2-9643-4787-87f3-b0f00189c33b',
    N 'Test', N 'test@bitwarden.com',
    0, N 'AQAAAAEAAYagAAAAEJ3ky9F/Zt5sy3/UAHVvBarMR+tBXYOM5IGgXy4/mx82uptgHgItauyCN+UZTvAqiA==',
    null, N 'en-US', N 'F3KL7SCJKEXO4LJFVLGZITPEHM7SAVSZ',
    null, null, null, null, N '2024-01-07 23:56:48.2600000',
    N '2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=',
    N 'MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0Ww2chogqCpaAR7Uw448am4b7vDFXiM5kXjFlGfXBlrAdAqTTggEvTDlMNYqPlCo+mBM6iFmTTUY9rpZBvFskMnKvsvpJ47/fehAH2o2e3Ulv/5NFevaVCMCmpkBDtbMbO1A4a3btdRtCP8DsKWMefHauEpaoLxNTLWnOIZVfCMjsSgx2EvULHAZPTtbFwm4+UVKniM4ds4jvOsD85h4jn2aLs/jWJXFfxN8iVSqEqpC2TBvsPdyHb49xQoWWfF0Z6BiNqeNGKEU9Uos1pjL+kzhEzzSpH31PZT/ufJ/oo4+93wrUt57hb6f0jxiXhwd5yQ+9F6wVwpbfkq0IwhjOwIDAQAB',
    N '2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=',
    0, null, null, null, null, null, null,
    null, N '2024-01-07 23:53:38.5900000',
    N '2024-01-07 23:53:38.5900000',
    null, 0, 600000, N '{"id":null}', N '7gp59kKHt9kMlks0BuNC4IjNXYkljR',
    0, 0, 0, null, null, null, null, null,
    null, null, null
  );
```
