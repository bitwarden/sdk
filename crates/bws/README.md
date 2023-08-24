# Bitwarden Secrets Manager CLI

A Rust CLI for interacting with the
[Bitwarden Secrets Manager](https://bitwarden.com/products/secrets-manager/). This is a beta
release and might be missing some functionality.

## Install

```bash
cargo install bws
```

Or download a pre-built binary from the [Releases](https://github.com/bitwarden/sdk/releases) page.

## Usage

```bash
bws --help
```

## How to enable shell autocompletions

### Zsh

If completion is not enabled already, you need to enable it first:

```zsh
echo "autoload -U compinit; compinit" >> ~/.zshrc
```

Enable autocompletions for the current user:

```zsh
echo 'source <(/path/to/bws completions zsh)' >> ~/.zshrc
```

### Bash

Enable autocompletions for the current user:

```zsh
echo 'source <(/path/to/bws completions bash)' >> ~/.bashrc
```

For more detailed documentation, please refer to the
[Secrets Manager CLI help article](https://bitwarden.com/help/secrets-manager-cli/).
