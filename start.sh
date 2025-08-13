#!/bin/bash

echo "🚀 Starting Battalion application..."

# Start SurrealDB in the background
echo "🗄️ Starting SurrealDB on localhost:8000..."
surreal start --bind 0.0.0.0:8000 --user root --pass root memory &

# Wait for SurrealDB to start
echo "⏳ Waiting for SurrealDB to start..."
sleep 5

# Run database migrations
echo "📊 Running database migrations..."
./run_all_migrations.sh

# Start the website
echo "🌐 Starting website on port 6969..."
./target/release/website

