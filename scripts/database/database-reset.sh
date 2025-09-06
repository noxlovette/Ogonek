#!/bin/bash

# Database Reset Script for pg-ogonek-dev
# This script will reset the database, run migrations, and seed data

set -e  # Exit on any error

echo "🔄 Starting database reset process..."

# Step 1: Terminate existing connections and drop/recreate database
echo "📋 Terminating existing database connections..."
docker exec -it pg-ogonek-dev psql -U postgres -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = 'pg-ogonek-dev';"

echo "🗑️  Dropping existing database..."
docker exec -it pg-ogonek-dev psql -U postgres -c "DROP DATABASE IF EXISTS \"pg-ogonek-dev\";"

echo "🏗️  Creating new database..."
docker exec -it pg-ogonek-dev psql -U postgres -c "CREATE DATABASE \"pg-ogonek-dev\" OWNER postgres;"

# Step 2: Set environment variable
echo "🔧 Setting DATABASE_URL environment variable..."
export DATABASE_URL="postgres://postgres:H8QheSCRFCKejvDsbu@localhost:5433/pg-ogonek-dev"

# Step 3: Run migrations
cd axum
echo "🚀 Running database migrations..."
sqlx migrate run

# Step 4: Seed the database
echo "🌱 Seeding database with initial data..."
cargo run --bin seed_db

echo "✅ Database reset complete!"
echo "📊 Database URL: $DATABASE_URL"
