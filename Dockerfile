ARG ARCH=
FROM ${ARCH}rust:1.73 as builder
RUN apt update && \
  apt install --no-install-recommends -y \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .
WORKDIR /app/crates/bws
RUN cargo build --release
FROM ${ARCH}debian:trixie-slim as runner
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/bws .
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

ENTRYPOINT ["bws"]
