#!/bin/bash

set -e  # Exit on any error

echo "🗑️  Removing Battalion deployment from Fly.io..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

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
    print_error "flyctl is not installed."
    exit 1
fi

print_status "Destroying battalion-website app..."
flyctl apps destroy battalion-website --yes || print_warning "App battalion-website may not exist"

print_status "Destroying battalion-surrealdb app..."
flyctl apps destroy battalion-surrealdb --yes || print_warning "App battalion-surrealdb may not exist"

print_success "All Battalion apps have been removed from Fly.io!"

echo ""
echo "🧹 Cleanup completed!"
echo "   • All machines stopped and destroyed"
echo "   • All IP addresses released"
echo "   • No ongoing charges"