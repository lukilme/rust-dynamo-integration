# ---------- build ----------
FROM rust:1.75 as builder
WORKDIR /app

# evita rebuild desnecessário
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# código real
COPY . .
RUN cargo build --release

# ---------- runtime ----------
FROM debian:bookworm-slim
WORKDIR /app

# certificados TLS (necessários para AWS SDK)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /app/api

EXPOSE 3000
CMD ["./api"]
