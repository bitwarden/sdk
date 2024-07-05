# PHP Installation

## Introduction

Composer is used to build PHP Bitwarden client library.

## Prerequisites

- PHP >= 8.0
- FFI extension enabled in PHP configuration
- Composer
- Bitwarden SDK native library, which is expected in one of below locations, depending on the OS and architecture.
  If you prefer to build SDK yourself, see [SDK README.md](../../README.md) for instructions.
    - Windows x86_64: `/lib/windows-x64/bitwarden_c.dll`
    - Linux x86_64: `/lib/linux-x64/libbitwarden_c.so`
    - macOS x86_64: `/lib/macos-x64/libbitwarden_c.dylib`
    - macOS aarch64: `/lib/macos-arm64/libbitwarden_c.dylib`

## Build Commands

```shell
composer install
```

## Example

### macOS

#### Install Prerequisites

Use brew Composer and PHP

```shell
brew install php
brew install composer
```

#### Build Commands

```shell
composer install
```

## Example SDK Usage Project

```shell
export ACCESS_TOKEN="<access_token>"
export ORGANIZATION_ID="<organization_id>"
export API_URL="https://api.bitwarden.com"
export IDENTITY_URL="https://identity.bitwarden.com"

php example.php
```
