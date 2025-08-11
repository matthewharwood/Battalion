#!/bin/bash

set -e  # Exit on any error

echo "🚀 Starting Battalion deployment to Fly.io..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if flyctl is installed
if ! command -v flyctl &> /dev/null; then
    print_error "flyctl is not installed. Please install it first: https://fly.io/docs/getting-started/installing-flyctl/"
    exit 1
fi

# Check if user is logged in
if ! flyctl auth whoami &> /dev/null; then
    print_error "You are not logged in to Fly.io. Please run: flyctl auth login"
    exit 1
fi

print_status "Creating SurrealDB app..."

# Step 1: Create SurrealDB app
cd surrealdb-fly
flyctl apps create battalion-surrealdb || print_warning "App battalion-surrealdb may already exist"

# Create fly.toml for SurrealDB with internal-only access
cat > fly.toml << 'EOF'
app = 'battalion-surrealdb'
primary_region = 'sjc'

[build]
  dockerfile = 'Dockerfile'

# Internal-only service - no public HTTP access
[[services]]
  internal_port = 8000
  protocol = 'tcp'
  auto_stop_machines = true
  auto_start_machines = true
  min_machines_running = 1

  # No [[services.ports]] section = internal only
  
  [services.concurrency]
    type = 'connections'
    hard_limit = 25
    soft_limit = 20

[[vm]]
  memory = '256mb'
  cpu_kind = 'shared'
  cpus = 1
EOF

print_status "Deploying SurrealDB container..."
flyctl deploy

print_success "SurrealDB deployed successfully!"
cd ..

print_status "Creating Battalion website app..."

# Step 2: Create Battalion website app
flyctl apps create battalion-website || print_warning "App battalion-website may already exist"

# Create fly.toml for Battalion website with public access
cat > fly.toml << 'EOF'
app = 'battalion-website'
primary_region = 'sjc'

[build]
  dockerfile = 'Dockerfile'

[env]
  SURREALDB_URL = "battalion-surrealdb.internal:8000"

[http_service]
  internal_port = 6969
  force_https = true
  auto_stop_machines = true
  auto_start_machines = true
  min_machines_running = 1

[[vm]]
  memory = '1gb'
  cpu_kind = 'shared'
  cpus = 1
EOF

print_status "Setting environment variables for database connection..."
flyctl secrets set SURREALDB_URL=battalion-surrealdb.internal:8000 --app battalion-website

print_status "Deploying Battalion website..."
flyctl deploy --app battalion-website

print_success "Battalion website deployed successfully!"

print_status "Allocating dedicated IP addresses..."
flyctl ips allocate-v4 --yes --app battalion-website || print_warning "IPv4 allocation failed (may require billing)"
flyctl ips allocate-v6 --app battalion-website || print_warning "IPv6 allocation failed"

print_success "Deployment completed!"

echo ""
echo "🎉 Battalion is now deployed on Fly.io!"
echo ""
echo "📋 Deployment Summary:"
echo "  • SurrealDB: Internal service (battalion-surrealdb.internal:8000)"
echo "  • Website: https://battalion-website.fly.dev/"
echo ""
echo "🔍 Useful commands:"
echo "  • Check status: flyctl status --app battalion-website"
echo "  • View logs: flyctl logs --app battalion-website"
echo "  • View DB logs: flyctl logs --app battalion-surrealdb"
echo "  • SSH into app: flyctl ssh console --app battalion-website"
echo ""
echo "🌐 Your website will be available at: https://battalion-website.fly.dev/"
echo "   (Note: Initial startup may take 2-3 minutes for Rust compilation)"
EOF