ARG VERSION
FROM rust:1.69 as builder

WORKDIR /crabwalk
COPY . .
RUN cargo build --release

FROM debian:buster-slim
LABEL version=${VERSION}
COPY  --from=builder /crabwalk/target/release/crabwalk /usr/local/bin/crabwalk
