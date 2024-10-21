#!/bin/sh
set -e

# Run database migrations
migrate -path=./migrations -database=${DB_ADDR} up

# Start the application
exec ./out
