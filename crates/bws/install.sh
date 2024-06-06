#!/bin/sh
set -u

##################################################
# An installer for the bws command line utility. #
##################################################

BWS_VERSION="${BWS_VERSION:-0.5.0}"

main() {
  check_required
  platform_detect
  arch_detect
  download_bws
  install_bws
}

error() {
  echo "$1" >&2
  echo "Exiting..." >&2
  exit 1
}

check_required() {
  if ! command -v curl >/dev/null && ! command -v wget >/dev/null; then
    error "curl or wget is required to download bws."
  fi

  if ! command -v unzip >/dev/null; then
    error "unzip is required to install bws."
  fi
}

can_sudo() {
  if command -v sudo >/dev/null; then
    echo "Attempting to install bws with sudo. Please enter your password if prompted."
    if sudo -v 2>/dev/null; then
      echo "sudo is available and we have the necessary permissions."
      echo "Installing bws to /usr/local/bin..."
      return 0
    else
      echo "sudo is available, but we failed to authenticate. Trying to install to your \$HOME directory..."
      return 1
    fi
  else
    echo "sudo is not available. Trying to install to your \$HOME directory..."
    return 1
  fi
}

platform_detect() {
  if [ "$(uname -s)" = "Linux" ]; then
    PLATFORM="unknown-linux-gnu"
  elif [ "$(uname -s)" = "Darwin" ]; then
    PLATFORM="apple-darwin"
  else
    error "Unsupported platform: $(uname -s)"
  fi
}

arch_detect() {
  if [ "$(uname -m)" = "x86_64" ]; then
    ARCH="x86_64"
  elif [ "$(uname -m)" = "aarch64" ]; then
    ARCH="aarch64"
  else
    error "Unsupported architecture: $(uname -m)"
  fi
}

downloader() {
  if command -v curl >/dev/null; then
    curl -L -o "$2" "$1"
  else
    wget -O "$2" "$1"
  fi
}

extract() {
  # TODO: support other zip utilities
  unzip -o "$1" -d "$2"
}

download_bws() {
  bws_url="https://github.com/bitwarden/sdk/releases/download/bws-v${BWS_VERSION}/bws-${ARCH}-${PLATFORM}-${BWS_VERSION}.zip"
  echo "Downloading bws from: $bws_url"
  tmp_dir="$(mktemp -d)"
  downloader "$bws_url" "$tmp_dir/bws.zip"
}

install_bws() {
  echo "Installing bws..."
  extract "$tmp_dir/bws.zip" "$tmp_dir"
  chmod +x "$tmp_dir/bws"

  if can_sudo; then
    sudo install -m 755 "$tmp_dir/bws" /usr/local/bin/bws

    if ! command -v bws >/dev/null; then
      error "Installation failed. bws was not found in /usr/local/bin"
    fi

    echo "bws installed to /usr/local/bin/bws"
  else
    user_bin_dir="${HOME}/.local/bin"
    mkdir -p "${user_bin_dir}"
    install -m 755 "$tmp_dir/bws" "${user_bin_dir}/bws"

    if ! command -v "${user_bin_dir}/bws" >/dev/null; then
      error "Installation failed. bws was not found in ${user_bin_dir}"
    fi

    echo "bws installed to ${user_bin_dir}/bws"
    echo "Please add ${user_bin_dir} to your PATH by adding the following line to your ~/.profile or shell rc file:"
    echo "export PATH=\"\$PATH:${user_bin_dir}\""
  fi

  rm -rf "$tmp_dir"
}

main
