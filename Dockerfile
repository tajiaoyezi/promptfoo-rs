FROM rust:1-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --workspace --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/promptfoo-rs /usr/local/bin/promptfoo-rs
ENTRYPOINT ["promptfoo-rs"]
