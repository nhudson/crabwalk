ARG VERSION
FROM rust:1.69 as builder

ARG TARGET
WORKDIR /crabwalk
COPY . .
RUN cargo build --release --target ${TARGET}

FROM debian:buster-slim
LABEL org.opencontainers.image.source=https://github.com/nhudson/crabwalk
LABEL version=${VERSION}
COPY  --from=builder /crabwalk/target/release/crabwalk /usr/local/bin/crabwalk
