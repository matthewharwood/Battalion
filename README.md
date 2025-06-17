# Local development

## 1. Start SurrealDB
surreal start --user root --pass root memory

## 2. Run migrations
surreal migrate event/migrations job/migrations applicant/migrations

## 3. Run the site
cargo run -p website

## 4. Smoke test
open http://localhost:3000/events
