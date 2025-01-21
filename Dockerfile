FROM rust:1.84.0

WORKDIR /usr/src/mastodon-plays-snake-rs

COPY . .

RUN cargo install --path .

CMD ["mastodon-plays-snake-rs"]
