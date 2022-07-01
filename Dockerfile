FROM rust as builder


RUN cargo new --bin saulbot-rust
WORKDIR /saulbot-rust
RUN pwd
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release && rm src/*.rs

COPY . /saulbot-rust
RUN cat ./src/main.rs
RUN ls -al ./target/release/deps
RUN rm ./target/release/deps/saulbot-rust* || true
RUN rm ./target/release/deps/saulbot_rust* || true
RUN ls -al
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /saulbot-rust/target/release/saulbot-rust /saulbot-rust
COPY --from=builder /saulbot-rust/messages.json /messages.json
COPY --from=builder /saulbot-rust/src/main.rs /main.rs
RUN cat /main.rs

CMD ["/saulbot-rust"]