###############################################
#                 Build stage                 #
###############################################
FROM rust:1.76 AS build

# Copy required project files
COPY . /app

# Build project
WORKDIR /app
RUN ls -la /
RUN cargo build -p memory-testing
# RUN cargo build -p memory-testing --release

###############################################
#                  App stage                  #
###############################################
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends python3 gdb && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy built project from the build stage
COPY --from=build /app/target/debug/memory-testing .
COPY --from=build /app/crates/memory-testing/capture_dumps.py .

CMD [ "python3", "/capture_dumps.py", "./memory-testing", "/output" ]
