#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🐛 Starting Svelte debug process...${NC}"

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

echo -e "${YELLOW}🔍 Type checking (watch mode)...${NC}"
echo -e "${BLUE}💡 Running type check in watch mode - press Ctrl+C to continue to tests${NC}"
timeout 10s pnpm check:watch || echo -e "${YELLOW}⏰ Type check timeout - continuing...${NC}"

echo -e "${YELLOW}🧪 Running tests in debug mode...${NC}"
echo -e "${BLUE}💡 Running tests with detailed output${NC}"
run_cmd pnpm run test --debug

echo -e "${YELLOW}🔧 Generating fresh types...${NC}"
run_cmd pnpm run generate-types

echo -e "${YELLOW}🎨 Checking code formatting...${NC}"
if ! pnpm lint; then
    echo -e "${YELLOW}⚠️  Formatting issues found. Auto-fixing...${NC}"
    run_cmd pnpm format
    echo -e "${GREEN}✅ Code formatted${NC}"
else
    echo -e "${GREEN}✅ Code formatting is correct${NC}"
fi

echo -e "${GREEN}✅ Svelte debug process completed!${NC}"