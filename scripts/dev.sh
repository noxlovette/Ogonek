#!/bin/bash
set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting Ogonek development environment...${NC}"


echo -e "${YELLOW}💾 Setting up environment...${NC}"
export DATABASE_URL="
postgres://postgres:H8QheSCRFCKejvDsbu@postgres:5432/pg-ogonek-dev"

echo -e "${GREEN}🐳 Starting Docker Compose...${NC}"

docker compose -f ./compose/compose.dev.yaml up -d
export DATABASE_URL="
postgres://postgres:H8QheSCRFCKejvDsbu@localhost:5432/pg-ogonek-dev"

./scripts/backend/generate-types.sh
cd axum/crates/ogonek-db
echo -e "${YELLOW}Creating sqlx queries...${NC}"
cargo sqlx prepare


echo -e "${GREEN}✅ Development environment started!${NC}"
echo -e "${BLUE}📊 Services available at:${NC}"
echo -e "  - Frontend: http://localhost:5173"
echo -e "  - Backend: http://localhost:3000"
echo -e "  - Database: localhost:5432"
echo -e "  - Redis: localhost:6379"
echo -e "  - Redis UI: http://localhost:8003"
