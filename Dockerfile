# FROM ubuntu:latest AS runtime

# WORKDIR app

# RUN apt-get -y update && apt-get -y install openssl pkg-config libssl-dev sqlite3

# ENV RUST_LOG=debug

# # COPY ./hairdressers.db ./hairdressers.db
# # COPY ./Cargo.toml ./Cargo.toml
# COPY ./distfiles/backend-lacquer /usr/local/bin

# ENTRYPOINT ["/usr/local/bin/backend-lacquer"]

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin backend-lacquer

# We do not need the Rust toolchain to run the binary!
FROM ubuntu:latest AS runtime
WORKDIR app

RUN apt-get -y update && apt-get -y install openssl pkg-config libssl-dev sqlite3

# COPY ./hairdressers.db ./hairdressers.db
COPY --from=builder /app/target/release/backend-lacquer /usr/local/bin

ENV RUST_LOG=debug
ENTRYPOINT ["/usr/local/bin/backend-lacquer"]
