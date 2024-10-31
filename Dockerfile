# Stage 1 - Build
FROM rust:slim as builder
WORKDIR /usr/src/my_rust_cron
COPY . .
RUN cargo build --release

# Stage 2 - Run
FROM debian:bullseye-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/my_rust_cron/target/release/my_rust_cron .

# Install minimal dependencies for SQLite
RUN apt-get update && apt-get install -y libsqlite3-dev speedtest-cli && rm -rf /var/lib/apt/lists/*

CMD ["./my_rust_cron"]
