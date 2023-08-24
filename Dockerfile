ARG VERSION
FROM rust:1.72 as builder

WORKDIR /crabwalk
COPY . .
RUN cargo build --release

FROM debian:buster-slim
LABEL org.opencontainers.image.source=https://github.com/nhudson/crabwalk
LABEL version=${VERSION}
COPY  --from=builder /crabwalk/target/release/crabwalk /usr/local/bin/crabwalk

# Metadata
LABEL org.opencontainers.image.vendor="nhudson" \
    org.opencontainers.image.url="https://github.com/nhudson/crabwalk" \
    org.opencontainers.image.title="crabwalk" \
    org.opencontainers.image.description="A simple Github webhook router written in Rust" \
    org.opencontainers.image.version="${VERSION}" \
    org.opencontainers.image.documentation="https://github.com/nhudson/crabwalk"
