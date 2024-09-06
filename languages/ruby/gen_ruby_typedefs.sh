#!/usr/bin/env bash
# shellcheck disable=SC3044,SC3020

# bail if rbs is not installed
if ! command -v rbs &>/dev/null; then
  echo "rbs could not be found"
  exit
fi

# use consistent repository root to avoid relative paths
REPO_ROOT="$(git rev-parse --show-toplevel)"
pushd "$REPO_ROOT"/languages/ruby || exit

# delete existing typedefs
rm -rf bitwarden_sdk_secrets/sig/*
mkdir -p bitwarden_sdk_secrets/sig

# generate typedefs
RUBY_LIB_FILES="$(find bitwarden_sdk_secrets/lib -name "*.rb")"

for file in $RUBY_LIB_FILES; do
  rbs prototype rb "$file" >bitwarden_sdk_secrets/sig/"$(basename "$file" .rb).rbs"
  rm -f bitwarden_sdk_secrets/sig/schemas.rbs
done

popd || exit
