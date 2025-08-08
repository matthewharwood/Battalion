#!/bin/bash

# List of module directories
DB_URL=${SURREALDB_URL:-127.0.0.1:8000}

# MODULES=("job" "event" "applicant" "vote")
MODULES=("job" "event" "applicant")

for module in "${MODULES[@]}"; do
  echo "📂 Entering $module directory..."
  cd "$module" || { echo "❌ Failed to enter $module"; exit 1; }

  for file in migrations/*.surql; do
    echo "🚀 Running migration: $module/$file"
    surreal sql --conn http://$DB_URL \
                --user root \
                --pass root \
                --ns test \
                --db test < "$file"
  done

  cd - > /dev/null
done

echo "✅ All migrations completed successfully."
