#!/bin/bash

echo "🚀 Starting Battalion application..."

# Run database migrations
echo "📊 Running database migrations..."
./run_all_migrations.sh

# Start the website
echo "🌐 Starting website..."
cargo watch -x "run --package website"

