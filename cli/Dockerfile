# syntax=docker/dockerfile:1

# Builder.
FROM rust:1.69.0-slim-buster AS builder
RUN apt-get update && \
    apt-get install -y \
    --no-install-recommends\
    build-essential \
    cmake \
    && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY ./ .
RUN cargo build --release

# Final image.
FROM alpine:3.17.3
WORKDIR /app
COPY --from=builder /app/target/release/cli /app/
