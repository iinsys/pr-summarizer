FROM rust:1.77-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:12.9-slim
COPY --from=builder /usr/src/app/target/release/pr-summarizer /usr/local/bin/pr-summarizer
ENTRYPOINT ["pr-summarizer"]