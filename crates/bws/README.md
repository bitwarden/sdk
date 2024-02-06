# Bitwarden Secrets Manager CLI

A Rust CLI for interacting with the
[Bitwarden Secrets Manager](https://bitwarden.com/products/secrets-manager/). This is a beta release
and might be missing some functionality.

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

## Docker

We also provide a docker image preloaded with the `bws` cli.

```bash
# From the root of the repository
docker build -f crates/bws/Dockerfile -t bitwarden/bws .

docker run --rm -it bitwarden/bws --help
```

To use a configuration file, utilize docker [bind mounting](https://docs.docker.com/storage/bind-mounts/)
to expose it to the container:

```bash
docker run --rm -it -v "$HOME"/.bws:/home/app/.bws bitwarden/bws --help
```
