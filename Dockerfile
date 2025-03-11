FROM rust:latest as builder 

WORKDIR /app

COPY . .

RUN cargo test --release && cargo build --release

FROM debian:latest

RUN apt-get update -y && apt-get install libssl-dev -y

COPY --from=builder /app/target/release/sync_https_server /app/sync_https_server

CMD ["/app/sync_https_server"]