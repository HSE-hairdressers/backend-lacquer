FROM debian:latest AS runtime

WORKDIR app

COPY ./hairdressers.db ./hairdressers.db
COPY ./Cargo.toml ./Cargo.toml
COPY ./backend-lacquer /usr/local/bin

RUN apt-get -y update
RUN apt-get -y install openssl pkg-config libssl-dev
RUN apt-get -y install libssl1.1
RUN apt-get -y install sqlite3

ENV RUST_LOG=debug

ENTRYPOINT ["/usr/local/bin/backend-lacquer"]
