#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 Starting Svelte testing process...${NC}"

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

# Navigate to svelte directory
cd svelte || handle_error "svelte folder not found"

echo -e "${YELLOW}📦 Installing/updating pnpm dependencies...${NC}"
run_cmd pnpm install --frozen-lockfile

echo -e "${YELLOW}🔍 Type checking...${NC}"
run_cmd pnpm check

echo -e "${YELLOW}🎨 Running linter...${NC}"
run_cmd pnpm lint

echo -e "${YELLOW}🏗️  Building Svelte app...${NC}"
run_cmd pnpm build

echo -e "${YELLOW}🧪 Running tests...${NC}"
run_cmd pnpm run test

echo -e "${GREEN}✅ All Svelte tests passed!${NC}"