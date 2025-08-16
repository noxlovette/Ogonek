#!/bin/bash
# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DB_URL="postgres://postgres:H8QheSCRFCKejvDsbu@localhost:5433/pg-ogonek-dev"

echo -e "${BLUE}🚀 Starting development checks...${NC}"

# Function to handle errors
handle_error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

# Function to run command with error handling
run_cmd() {
    if ! "$@"; then
        handle_error "Command failed: $*"
    fi
}

echo -e "${YELLOW}🦀 Running Rust checks in axum folder...${NC}"

# Navigate to axum folder
cd axum || handle_error "axum folder not found"

# Set database URL for all operations
export DATABASE_URL="$DB_URL"

echo -e "${YELLOW}📦 Updating cargo dependencies...${NC}"
run_cmd cargo update

echo -e "${YELLOW}🎨 Checking Rust formatting...${NC}"
if ! cargo fmt --check; then
    echo -e "${YELLOW}⚠️  Formatting issues found. Auto-fixing...${NC}"
    run_cmd cargo fmt
fi

echo -e "${YELLOW}🔍 Running cargo clippy...${NC}"
run_cmd cargo clippy -- -D warnings

echo -e "${YELLOW}🧪 Running cargo tests...${NC}"
run_cmd cargo test

echo -e "${YELLOW}📋 Generating OpenAPI spec...${NC}"
run_cmd cargo run --bin generate_types

# Copy OpenAPI spec to Swift project
if [ -f openapi.yaml ]; then
    cp openapi.yaml ~/Development/ogonek-swift/ || echo -e "${YELLOW}⚠️  Could not copy OpenAPI spec to Swift project${NC}"
fi

echo -e "${YELLOW}🗄️  Preparing sqlx queries...${NC}"
run_cmd cargo sqlx prepare

echo -e "${GREEN}✅ Rust checks completed successfully${NC}"

# Navigate back to root and then to svelte folder
cd ..

echo -e "${YELLOW}⚡ Running Svelte checks in svelte folder...${NC}"
cd svelte || handle_error "svelte folder not found"

echo -e "${YELLOW}📦 Installing/updating pnpm dependencies...${NC}"
# Use install --frozen-lockfile for consistency with CI
run_cmd pnpm install --frozen-lockfile

echo -e "${YELLOW}🎨 Formatting Svelte code...${NC}"
run_cmd pnpm format

echo -e "${YELLOW}🔍 Type checking...${NC}"
run_cmd pnpm check

echo -e "${YELLOW}🏗️  Building Svelte app...${NC}"
run_cmd pnpm build

echo -e "${YELLOW}🧹 Cleaning build artifacts...${NC}"
rm -rf node_modules/.cache .svelte-kit/output

echo -e "${GREEN}✅ Svelte checks completed successfully${NC}"
echo -e "${GREEN}🎉 All development checks passed!${NC}"