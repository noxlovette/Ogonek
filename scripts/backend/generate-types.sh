#!/bin/bash
# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

echo -e "${YELLOW}Generating types...${NC}"

# Navigate to axum folder
cd axum || handle_error "axum folder not found"

echo -e "${YELLOW}📋 Generating OpenAPI spec...${NC}"
run_cmd cargo run --bin generate_types

# Copy OpenAPI spec to Swift project
if [ -f openapi.yaml ]; then
    cp openapi.json ../svelte/ || echo -e "${YELLOW} Could not copy spec to svelte folder${NC}"

    cp openapi.yaml ~/Development/ogonek-swift/ || echo -e "${YELLOW}⚠️  Could not copy OpenAPI spec to Swift project${NC}"
fi

cd ../svelte

run_cmd pnpm run generate-types
run_cmd pnpm exec orval

echo -e "${GREEN}Types generated${NC}"