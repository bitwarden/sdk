###############################################
#                 Build stage                 #
###############################################
FROM --platform=$BUILDPLATFORM rust:1.76 AS build

# Docker buildx supplies the value for this arg
ARG TARGETPLATFORM

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Copy required project files
COPY . /app

# Build project
WORKDIR /app/crates/bws
RUN cargo build --release --bin bws

# Bundle bws dependencies
RUN mkdir /lib-bws
RUN ldd /app/target/release/bws | tr -s '[:blank:]' '\n' | grep '^/' | xargs -I % cp % /lib-bws

###############################################
#                  App stage                  #
###############################################
FROM scratch

ARG TARGETPLATFORM
LABEL com.bitwarden.product="bitwarden"

# Copy built project from the build stage
WORKDIR /usr/local/bin
COPY --from=build /app/target/release/bws .

# Copy certs
COPY --from=build /etc/ssl/certs /etc/ssl/certs

# Copy bws dependencies
COPY --from=build /lib-bws /lib

ENTRYPOINT ["bws"]
