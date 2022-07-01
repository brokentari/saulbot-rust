FROM rust as builder


RUN cargo new --bin saulbot-rust
WORKDIR /saulbot-rust
RUN pwd
COPY ./Cargo.toml ./Cargo.toml
RUN ls -al
RUN cargo build --release && rm src/*.rs

COPY . ./
RUN rm ./target/release/deps/saulbot-rust* || true
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /saulbot-rust/target/release/saulbot-rust /saulbot-rust
COPY --from=builder /saulbot-rust/messages.json /messages.json
RUN ls -la /

CMD ["/saulbot-rust"]