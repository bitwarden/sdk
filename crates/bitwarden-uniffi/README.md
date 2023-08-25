# Bitwarden-uniffi

## Generating documentation

```bash
cargo +nightly rustdoc -p bitwarden -- -Zunstable-options --output-format json
cargo +nightly rustdoc -p bitwarden-uniffi -- -Zunstable-options --output-format json

npx ts-node ./support/docs/docs.ts > languages/kotlin/doc.md
```
