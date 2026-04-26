# syntax=docker/dockerfile:1.7

FROM rust:1.95-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests
COPY Docs ./Docs

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN useradd -r -u 10001 -g nogroup appuser \
    && apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates python3 python3-pip python3-venv \
    && python3 -m venv /opt/venv \
    && /opt/venv/bin/pip install --no-cache-dir sentence-transformers onnxruntime \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/embedding-api-server /usr/local/bin/embedding-api-server

ENV RUST_LOG=info \
    BIND_ADDR=0.0.0.0:8000 \
    MODELS_DIR=/app/AI_Models \
    PATH=/opt/venv/bin:$PATH

EXPOSE 8000

USER appuser

CMD ["/usr/local/bin/embedding-api-server"]
