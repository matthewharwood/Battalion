#!/bin/bash

# List of module directories
MODULES=("event" "job" "applicant" "review" "vote")

for module in "${MODULES[@]}"; do
  echo "📂 Entering $module directory..."
  cd "$module" || { echo "❌ Failed to enter $module"; exit 1; }

  for file in migrations/*.surql; do
    echo "🚀 Running migration: $module/$file"
    surreal sql --conn http://127.0.0.1:8000 \
                --user root \
                --pass root \
                --ns test \
                --db test < "$file"
  done

  cd - > /dev/null
done

echo "✅ All migrations completed successfully."
