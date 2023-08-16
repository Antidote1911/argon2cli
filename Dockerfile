FROM rust:1.62.0-slim-buster

WORKDIR /app

COPY Cargo.* ./
COPY Cargo.toml ./Cargo.source.toml

RUN sed -e 's/"dexios-gui",//' Cargo.source.toml > Cargo.toml \
  && cat Cargo.toml

COPY ./argon2cli ./argon2cli

RUN cargo install --bin argon2cli --path ./argon2cli \
  && rm -rf ./argon2cli* Cargo.*

VOLUME ["/data"]

WORKDIR /data

ENTRYPOINT ["/usr/local/cargo/bin/argon2cli"]

