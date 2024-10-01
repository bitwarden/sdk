#!/bin/sh
# expects to be executed from the root of the language-specific directory
# expects the system architecture to be located in the ARCH environment variable
cargo build --package bitwarden-c
export PREV_GOPATH=$(go env GOPATH)
cp ../../target/debug/libbitwarden_c.* internal/cinterface/lib/$ARCH/
go env -w GOPATH="$PWD:$(go env GOPATH)"
go test -v ./e2e_test
go env -w GOPATH=$PREV_GOPATH
