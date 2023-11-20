###############################################
#                 Build stage                 #
###############################################
FROM --platform=$BUILDPLATFORM rust:1.73 AS build

# Docker buildx supplies the value for this arg
ARG TARGETPLATFORM

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Copy required project files
COPY . /app

# Build project
WORKDIR /app/crates/bws
RUN cargo build --release

###############################################
#                  App stage                  #
###############################################
FROM debian:bookworm-slim

ARG TARGETPLATFORM
LABEL com.bitwarden.product="bitwarden"

# Copy built project from the build stage
WORKDIR /usr/local/bin
COPY --from=build /app/target/release/bws .
COPY --from=build /etc/ssl/certs /etc/ssl/certs

ENTRYPOINT ["bws"]

