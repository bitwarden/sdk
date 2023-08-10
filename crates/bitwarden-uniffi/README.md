# Bitwarden-uniffi

## Generating documentation

```bash
cargo +nightly rustdoc -p bitwarden -- -Zunstable-options --output-format json
cargo +nightly rustdoc -p bitwarden-uniffi -- -Zunstable-options --output-format json

node ./support/docs/docs.js > doc.md
```
