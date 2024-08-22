FROM rust:1.60 AS build

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM alpine:latest

COPY --from=build /usr/src/app/target/release/backend /usr/local/bin/backend

EXPOSE 8001

CMD ["backend"]
