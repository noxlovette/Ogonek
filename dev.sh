#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting Ogonek development environment...${NC}"

# Generate OpenAPI spec from Rust
echo -e "${YELLOW}📋 Generating OpenAPI spec...${NC}"
cd axum
cargo run --bin generate_types

# Export DATABASE_URL
echo -e "${YELLOW}💾 Setting up environment...${NC}"
export DATABASE_URL="
postgres://postgres:H8QheSCRFCKejvDsbu@postgres:5432/pg-ogonek-dev"
# Start the development environment
echo -e "${GREEN}🐳 Starting Docker Compose...${NC}"
# Go back to project root
cd ..

docker compose -f compose.dev.yaml up -d

echo -e "${GREEN}✅ Development environment started!${NC}"
echo -e "${BLUE}📊 Services available at:${NC}"
echo -e "  - Frontend: http://localhost:5173"
echo -e "  - Backend: http://localhost:3000"
echo -e "  - Database: localhost:5433"
echo -e "  - Redis: localhost:6379"
echo -e "  - Redis UI: http://localhost:8003"
