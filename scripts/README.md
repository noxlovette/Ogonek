# Scripts Directory

This directory contains organized development scripts for the Ogonek project.

## Directory Structure

- `database/` - Database-related scripts
- `frontend/` - Frontend (Svelte) scripts
- `backend/` - Backend (Axum/Rust) scripts
- `utils/` - Utility scripts

## Available Scripts

### Development Workflow

- `dev.sh` - Start development environment with Docker Compose
- `test-and-format.sh` - Run comprehensive tests and formatting for both frontend and backend

### Frontend Scripts

- `frontend/svelte-test.sh` - Run Svelte tests with pnpm
- `frontend/svelte-debug.sh` - Debug Svelte with detailed testing

### Backend Scripts

- `backend/generate-types.sh` - Generate TypeScript types from OpenAPI spec

### Database Scripts

- `database/database-reset.sh` - Reset database with fresh migrations and seed data

### Util Scripts

- `utils/update.sh` - Updates everything

## Usage

All scripts should be run from the project root directory:

```bash
# Development
./scripts/dev.sh

# Testing
./scripts/test-and-format.sh
./scripts/frontend/svelte-test.sh

# Database management
./scripts/database/database-reset.sh
```
