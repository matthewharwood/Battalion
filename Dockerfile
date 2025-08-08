FROM rust:latest

WORKDIR /app

# Install cargo-watch for automatic rebuilding
RUN cargo install cargo-watch

COPY . .

EXPOSE 6969 3000 8000

CMD ["cargo", "watch", "-x", "run --package website"]
