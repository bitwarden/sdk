# Delete old directory to ensure all files are updated
rm -rf crates/bitwarden-api-api/src

# Generate new API bindings
npx openapi-generator-cli generate \
    -i ../server/api.json \
    -g rust \
    -o crates/bitwarden-api-api \
    --package-name bitwarden-api-api \
    -t ./support/openapi-template \
    --additional-properties=packageVersion=1.0.0

# Delete old directory to ensure all files are updated
rm -rf crates/bitwarden-api-identity/src

# Generate new Identity bindings
npx openapi-generator-cli generate \
    -i ../server/identity.json \
    -g rust \
    -o crates/bitwarden-api-identity \
    --package-name bitwarden-api-identity \
    -t ./support/openapi-template \
    --additional-properties=packageVersion=1.0.0

rustup toolchain install nightly
cargo +nightly fmt
npm run prettier
