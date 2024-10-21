#!/bin/sh
set -e

# Run database migrations
migrate -path=./migrations -database=${DB_ADDR} up
