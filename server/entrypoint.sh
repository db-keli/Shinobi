#!/bin/sh
set -e

# Run database migrations
migrate -path=./cmd/migrate/migrations -database=${DB_ADDR} up

# Start the application
exec ./out
