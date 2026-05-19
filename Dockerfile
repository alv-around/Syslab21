FROM rust:1.95 as build

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release

FROM debian:bookworm-slim as runner

WORKDIR /app

COPY --from=build /build/target/release/systemlab21 .

ENTRYPOINT [ "/app/systemlab21" ]
