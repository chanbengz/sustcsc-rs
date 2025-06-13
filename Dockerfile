# Dockerfile for evaluation on Singularity
FROM rust:1.87.0-bookworm
# or nightly

WORKDIR /root/challenge

ENTRYPOINT [ "cargo", "run", "--release" ]