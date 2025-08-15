FROM rust:latest

WORKDIR /app

# Install SurrealDB CLI
RUN apt-get update && apt-get install -y curl ca-certificates && \
    curl -sSf https://install.surrealdb.com | sh && \
    rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build the application in release mode during image build
RUN cargo build --release --package website

# Make scripts executable
RUN chmod +x run_all_migrations.sh start.sh

EXPOSE 6969 8000

CMD ["./start.sh"]
