FROM rust:latest

WORKDIR /app

# Install cargo-watch for automatic rebuilding and SurrealDB CLI
RUN cargo install cargo-watch
RUN apt-get update && apt-get install -y curl
RUN curl -sSf https://install.surrealdb.com | sh

COPY . .

# Make scripts executable
RUN chmod +x run_all_migrations.sh start.sh

EXPOSE 6969 3000 8000

CMD ["./start.sh"]
