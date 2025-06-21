# Dockerfile for evaluation on Singularity
FROM rust:1.87.0-bookworm
# or nightly
# FROM rustlang/rust:nightly

COPY ./challenge /challenge
COPY ./.cargo /challenge/.cargo
WORKDIR /challenge

RUN cargo build --release && \
    cp ./target/release/sustcsc-rs ./ && cargo clean && \
    chmod +x ./sustcsc-rs

ENTRYPOINT [ "./sustcsc-rs" ]
