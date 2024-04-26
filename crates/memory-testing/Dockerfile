###############################################
#                 Build stage                 #
###############################################
FROM rust:1.76 AS build

WORKDIR /app

# Copy dependency files and create dummy files to allow cargo to build the dependencies in a separate stage
COPY Cargo.toml Cargo.lock /app/
COPY crates/bitwarden-crypto/Cargo.toml /app/crates/bitwarden-crypto/
COPY crates/memory-testing/Cargo.toml   /app/crates/memory-testing/

RUN mkdir -p /app/crates/bitwarden-crypto/src \
    && mkdir -p /app/crates/memory-testing/src \
    && touch /app/crates/bitwarden-crypto/src/lib.rs \
    && echo 'fn main(){}' > /app/crates/memory-testing/src/main.rs \
    && cargo build -p memory-testing

# Delete dummy files and copy the actual source code
RUN rm /app/crates/bitwarden-crypto/src/lib.rs /app/crates/memory-testing/src/main.rs
COPY crates/bitwarden-crypto/src /app/crates/bitwarden-crypto/src
COPY crates/memory-testing/src   /app/crates/memory-testing/src

# Build the project. We use touch to force a rebuild of the now real files
RUN touch /app/crates/bitwarden-crypto/src/lib.rs /app/crates/memory-testing/src/main.rs
RUN cargo build -p memory-testing

###############################################
#                  App stage                  #
###############################################
FROM debian:bookworm-slim

# This specifically needs to run as root to be able to capture core dumps
USER root

RUN apt-get update && apt-get install -y --no-install-recommends gdb=13.1-3 && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy built project from the build stage and the cases.json file
COPY --from=build /app/target/debug/memory-testing /app/target/debug/capture-dumps ./
COPY crates/memory-testing/cases.json .

CMD [ "/capture-dumps", "./memory-testing", "/output" ]
