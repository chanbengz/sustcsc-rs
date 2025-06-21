# Dockerfile for evaluation on Singularity
FROM rust:1.87.0-bookworm
# or nightly
# FROM rustlang/rust:nightly

COPY ./challenge /root/challenge
COPY ./.cargo /root/challenge/.cargo
WORKDIR /root/challenge

RUN cargo build --release

ENTRYPOINT [ "cargo", "run", "--release" ]