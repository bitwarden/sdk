#!/usr/bin/env bash

# Usage: ./build.sh [--release]

check_command() {
  if ! command -v "$1" &>/dev/null; then
    printf '%s\n' "$1 is required to build locally. Please install $2."
    exit 1
  fi
}

check_command cargo Rust
check_command go Go
check_command npm Node.js

REPO_ROOT="$(git rev-parse --show-toplevel)"
GO_LIB_DIR="$REPO_ROOT/languages/go/internal/cinterface/lib"

pushd "$REPO_ROOT" || exit 1
printf '%s\n\n' "Cleaning old builds..."
rm -f languages/go/example/example && rm -rf "$GO_LIB_DIR"

printf '%s\n\n' "Building binaries..."

if [ "$1" = "--release" ]; then
  cargo build --release
else
  cargo build
fi

npm i && npm run schemas

printf '%s\n\n' "Copying Go bindings to $GO_LIB_DIR..."
mkdir -p "$GO_LIB_DIR"
find target/debug -maxdepth 1 -type f -name "libbitwarden_c.*" -exec cp {} "$GO_LIB_DIR"/ \;

printf '%s\n\n' "Build complete!"
printf '%s\n' "To run the Go example, set the following environment variables:
  export API_URL=\"http://localhost:4000\" # your Bitwarden API URL
  export IDENTITY_URL=\"http://localhost:33656\" # your Bitwarden Identity URL
  export ACCESS_TOKEN=\"your-access-token\" # your Bitwarden access token
  export STATE_PATH=\"your-absolute-path\" # the absolute path to your state file
  export ORGANIZATION_ID=\"your-org-id\" # your Bitwarden organization ID
  export PROJECT_NAME=\"your-project-name\" # an arbitrary project name
"

printf '%s\n' "Then, run the example with:
  pushd $REPO_ROOT/languages/go/example
  go mod tidy
  go run example.go
  popd
"
