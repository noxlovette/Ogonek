#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

show_help() {
    echo -e "${BLUE}🚀 Ogonek Development Scripts${NC}"
    echo ""
    echo -e "${YELLOW}Usage:${NC} ./scripts/run.sh <command>"
    echo ""
    echo -e "${YELLOW}Available commands:${NC}"
    echo ""
    echo -e "${GREEN}Development:${NC}"
    echo "  dev                    - Start development environment"
    echo "  test                   - Run comprehensive tests and formatting"
    echo ""
    echo -e "${GREEN}Frontend:${NC}"
    echo "  svelte-test           - Run Svelte tests"
    echo "  svelte-debug          - Debug Svelte with detailed testing"
    echo ""
    echo -e "${GREEN}Backend:${NC}"
    echo "  generate-types        - Generate TypeScript types from OpenAPI"
    echo ""
    echo -e "${GREEN}Database:${NC}"
    echo "  db-reset              - Reset database with fresh data"
    echo ""
    echo -e "${GREEN}Utilities:${NC}"
    echo "  help                  - Show this help message"
    echo "  update                - Update everything"
    echo ""
}

case "$1" in
    "dev")
        ./scripts/dev.sh
        ;;
    "test")
        ./scripts/test-and-format.sh
        ;;
    "svelte-test")
        ./scripts/frontend/svelte-test.sh
        ;;
    "svelte-debug")
        ./scripts/frontend/svelte-debug.sh
        ;;
    "generate-types")
        ./scripts/backend/generate-types.sh
        ;;
    "db-reset")
        ./scripts/database/database-reset.sh
        ;;
    "update")
        ./scripts/utils/update.sh
        ;;
    "help" | "-h" | "--help" | "")
        show_help
        ;;
    *)
        echo -e "${RED}❌ Unknown command: $1${NC}"
        echo ""
        show_help
        exit 1
        ;;
esac