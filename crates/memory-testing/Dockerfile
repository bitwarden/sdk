###############################################
#                 Build stage                 #
###############################################
FROM rust:1.76 AS build

# Copy required project files
COPY . /app

# Build project
WORKDIR /app
RUN cargo build -p memory-testing

###############################################
#                  App stage                  #
###############################################
FROM debian:bookworm-slim

# This specifically needs to run as root to be able to capture core dumps
USER root

RUN apt-get update && apt-get install -y --no-install-recommends gdb=13.1-3 && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy built project from the build stage
COPY --from=build /app/target/debug/memory-testing /app/target/debug/capture-dumps /app/crates/memory-testing/cases.json ./

CMD [ "/capture-dumps", "./memory-testing", "/output" ]
