#!/bin/bash

if [ -n "$BITWARDEN_LIB_PATH" ]; then
    sed "s/{{.LibPath}}/$BITWARDEN_LIB_PATH/g" bitwarden_library.go > bitwarden_library.go
    go build -tags custom
else
    go build
fi
