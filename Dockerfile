FROM rust:latest

COPY . /app/project
WORKDIR /app/project

RUN env
RUN cargo build --release
RUN cargo run --release
ENTRYPOIN
