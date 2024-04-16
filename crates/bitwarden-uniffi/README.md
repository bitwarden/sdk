# Bitwarden-uniffi

## Generating documentation

If desired we have some scripts that generates markdown documentation from the rustdoc output.

```bash
cargo +nightly rustdoc -p bitwarden -- -Zunstable-options --output-format json
cargo +nightly rustdoc -p bitwarden-uniffi -- -Zunstable-options --output-format json
npm run schemas

npx ts-node ./support/docs/docs.ts > doc.md
```
