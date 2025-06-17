# Local development

## 1. Start SurrealDB
surreal start --user root --pass root memory

## 2. Run migrations
chmod +x run_all_migrations.sh
./run_all_migrations.sh

## 3. Run the site
cargo run -p website

## 4. Smoke test
open http://localhost:3000/events

