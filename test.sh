#!/bin/bash
set -e

# Start relay
docker-compose down
docker-compose up -d

# Wait for relay to be ready
echo "Waiting for relay to start..."
sleep 5

# Run tests
cargo test -- --test-threads=4

# Cleanup
docker-compose down