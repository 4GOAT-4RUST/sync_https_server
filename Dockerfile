FROM rust:alpine as builder 

WORKDIR /app

COPY . .

RUN cargo test --release &&\
    cargo build --release

FROM alpine:latest

# RUN apt-get update -y && apt-get install libssl-dev -y
WORKDIR /app

RUN apk add --no-cache libgcc
# RUN apt-get update && apt-get install libssl-dev

COPY --from=builder /app/target/release/sync_https_server /app/sync_https_server

CMD ["/app/sync_https_server"]