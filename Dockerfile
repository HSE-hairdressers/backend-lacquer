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
FROM debian:buster-slim AS runtime
WORKDIR app
COPY ./hairdressers.db ./hairdressers.db
COPY --from=builder /app/target/release/backend-lacquer /usr/local/bin
RUN apt -y update
RUN apt -y install openssl pkg-config libssl-dev
RUN apt -y install libssl1.1
RUN apt -y install sqlite3
ENV RUST_LOG=debug
ENTRYPOINT ["/usr/local/bin/backend-lacquer"]
