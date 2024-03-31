FROM rust:1.74-bullseye AS builder

WORKDIR /usr/src/auth

COPY Cargo.toml Cargo.lock ./

COPY src ./src

COPY migration ./migration

COPY entity ./entity

RUN cargo build --locked --release

FROM debian:bullseye-slim

WORKDIR /usr/src/app

COPY --from=builder /usr/src/auth/target/release/auth /usr/src/app/auth

EXPOSE 80

CMD ["./auth"]