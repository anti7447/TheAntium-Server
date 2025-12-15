FROM rust:1.90 AS builder

WORKDIR /app
# COPY Cargo.toml Cargo.lock ./
# COPY src/ ./src/
# COPY front/ front/
COPY . .
RUN cargo fetch
# COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/TheAntium-Server .
COPY ./front ./front
RUN ls -l .

EXPOSE 8080
ENTRYPOINT ["/app/TheAntium-Server"]
