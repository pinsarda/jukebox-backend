FROM rust:1.85 AS builder
WORKDIR /jukebox
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src/ src/
COPY migrations/ migrations/
RUN apt update
RUN apt install -y libasound2-dev

RUN cargo build --release

RUN wget https://github.com/yt-dlp/yt-dlp/releases/download/2025.03.27/yt-dlp_linux

FROM debian:bookworm-slim

WORKDIR /jukebox

RUN apt update
RUN apt install -y libasound2 libpq5 ca-certificates alsa-utils
COPY --chmod=0755 --from=builder /jukebox/yt-dlp_linux /bin/yt-dlp

EXPOSE 8080
COPY --from=builder /jukebox/target/release/jukebox/ /jukebox

ENTRYPOINT [ "/jukebox/jukebox" ]