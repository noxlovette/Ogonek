#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🦀 Running Rust checks in axum folder...${NC}"

# Navigate to axum folder and run Rust commands
cd axum || { echo -e "${RED}❌ axum folder not found${NC}"; exit 1; }

echo -e "${YELLOW}Updating cargo index...${NC}"
cargo update
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Updating Rust failed${NC}"
    cargo fmt
fi


echo -e "${YELLOW}Running cargo fmt...${NC}"
cargo fmt --check
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Formatting issues found. Running cargo fmt to fix...${NC}"
    cargo fmt
fi

echo -e "${YELLOW}Running cargo clippy...${NC}"
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Clippy found issues${NC}"
    exit 1
fi

echo -e "${YELLOW}Running cargo test...${NC}"
cargo test
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Tests failed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Rust checks completed successfully${NC}"

# Navigate back to root and then to svelte folder
cd ..
echo -e "${YELLOW}⚡ Running Svelte checks in svelte folder...${NC}"

cd svelte || { echo -e "${RED}❌ svelte folder not found${NC}"; exit 1; }
echo -e "${YELLOW}Running pnpm update${NC}"
pnpm format
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Svelte update failed${NC}"
    exit 1
fi


echo -e "${YELLOW}Running pnpm format...${NC}"
pnpm format
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Formatting failed${NC}"
    exit 1
fi

echo -e "${YELLOW}Running pnpm lint...${NC}"
pnpm lint
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Linting failed${NC}"
    exit 1
fi

echo -e "${YELLOW}Running pnpm test...${NC}"
pnpm test
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Tests failed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Svelte checks completed successfully${NC}"
echo -e "${GREEN}🎉 All checks passed!${NC}"
