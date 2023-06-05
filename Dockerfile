FROM ubuntu:latest AS runtime

WORKDIR app

RUN apt-get -y update && apt-get -y install openssl pkg-config libssl-dev sqlite3

ENV RUST_LOG=debug

COPY ./hairdressers.db ./hairdressers.db
COPY ./Cargo.toml ./Cargo.toml
COPY ./backend-lacquer /usr/local/bin

ENTRYPOINT ["/usr/local/bin/backend-lacquer"]
