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

RUN apt-get update && apt-get install -y --no-install-recommends gdb && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy built project from the build stage
COPY --from=build /app/target/debug/memory-testing .
COPY --from=build /app/target/debug/capture-dumps .
COPY --from=build /app/crates/memory-testing/cases.json .

CMD [ "/capture-dumps", "./memory-testing", "/output" ]
