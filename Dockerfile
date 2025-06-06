FROM rust:1.87.0-bookworm
# or nightly

COPY sustcsc-rs-${TEAM_ID} /root/challenge
WORKDIR /root/challenge

RUN cargo build --release