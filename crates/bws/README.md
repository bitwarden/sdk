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

You can also use the `bws` Docker image:

<!-- TODO: remove the build step once the Docker image is published to the Docker Hub -->

```bash
# From the root of the repository, build the Docker image:
docker build -f crates/bws/Dockerfile --no-cache -t bitwarden/bws .

# Run with Docker:
docker run --rm -it bitwarden/bws --help
```

The Docker image is run with a non-root user named `app`. If you need to pass your config file to
the container, you can use the `-v`/`--volume` flag to mount your local `.bws` directory to the
default location within the container:

```bash
docker run --rm -it -v "$HOME"/.bws:/home/app/.bws bitwarden/bws --help
```

Alternatively, you can use the `BWS_CONFIG_FILE` environment variable to specify the location of the
config file within the container:

```bash
docker run --rm -it -e BWS_CONFIG_FILE="/path/to/config/file" -v /path/to/config/file:"$BWS_CONFIG_FILE" bitwarden/bws --help
```

Or, more concisely:

```bash
# Set the BWS_CONFIG_FILE environment variable on your host
export BWS_CONFIG_FILE="/path/to/config/file"

# Pass the BWS_CONFIG_FILE environment variable to the container
docker run --rm -it -e BWS_CONFIG_FILE="$BWS_CONFIG_FILE" -v "$BWS_CONFIG_FILE":"$BWS_CONFIG_FILE" bitwarden/bws --help
```

Note that if you want to use identical config file paths on your host and in the container, the
parent directory must exist on both.
