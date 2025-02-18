FROM rust:1.84 AS builder
WORKDIR /jukebox
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src/ src/
RUN apt update
RUN apt install -y libasound2-dev

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /jukebox

RUN apt update
RUN apt install -y libasound2 libpq5

COPY --from=builder /jukebox/target/release/jukebox/ /jukebox

ENTRYPOINT [ "/jukebox/jukebox" ]