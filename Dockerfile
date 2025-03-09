FROM rust:latest as builder 

WORKDIR /app

COPY ..

RUN cargo build --release

FROM debian:latest

RUN apt-get update && apt-get install libssl-dev

COPY --from=builder /app/target/release/sync_https_server /app/sync_https_server

CMD ["/app/sync_https_server"]