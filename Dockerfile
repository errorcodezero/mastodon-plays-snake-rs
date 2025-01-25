FROM rust:1.84.0

WORKDIR /usr/src/mastodon-plays-snake-rs

COPY ./Cargo.lock ./Cargo.lock

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build

COPY ./src/ ./src/

RUN cargo install --path .

CMD ["mastodon-plays-snake-rs"]
