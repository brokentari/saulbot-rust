FROM rust as builder

WORKDIR /usr/src/
RUN USER=root cargo new --bin saulbot-rust
WORKDIR /usr/src/saulbot-rust
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY . ./
RUN rm ./target/release/deps/saulbot-rust* || true
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/saulbot-rust/target/release/saulbot-rust /usr/src/app/saulbot-rust
COPY --from=builder /usr/src/saulbot-rust/messages.json /usr/src/app/messages.json

CMD ["/usr/src/app/saulbot-rust"]