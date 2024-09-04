FROM rust:1.71-slim AS builder
WORKDIR /usr/src/llm_validator
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . .
RUN cargo build --release
FROM debian:buster-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/llm_validator
COPY --from=builder /usr/src/llm_validator/target/release/llm_validator .
ENV RUN_LIVE=False
CMD ["./llm_validator"]

