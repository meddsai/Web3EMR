FROM rust:1.76 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend
COPY . .
EXPOSE 8080
CMD ["/app/backend"]
