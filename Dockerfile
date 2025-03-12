FROM rust:alpine as builder 

WORKDIR /app

COPY . .

RUN cargo test --release &&\
    cargo build --release

FROM alpine:latest

<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 41e4726 (fixed some merge conflicts)
# RUN apt-get update -y && apt-get install libssl-dev -y
WORKDIR /app

RUN apk add --no-cache libgcc

EXPOSE 80
# RUN apt-get update && apt-get install libssl-dev
<<<<<<< HEAD
=======
RUN apt-get update -y && apt-get install libssl-dev -y
>>>>>>> bb4ffee (Fix: Added -y flag for non-interactive updates)
=======
>>>>>>> 41e4726 (fixed some merge conflicts)

COPY --from=builder /app/target/release/sync_https_server /app/sync_https_server

CMD ["/app/sync_https_server"]