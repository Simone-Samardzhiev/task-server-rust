FROM rust:latest as builder

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --target=x86_64-unknown-linux-musl

COPY src/ src/

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /app/server

CMD ["/app/server"]