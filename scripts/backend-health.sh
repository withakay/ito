#!/usr/bin/env bash
#
# Health check script for Ito backend Docker Compose runtime.
#
# Usage:
#   ./scripts/backend-health.sh
#
# Exits 0 if the backend is healthy, 1 otherwise.

set -euo pipefail

COMPOSE_FILE="${COMPOSE_FILE:-docker-compose.backend.yml}"
HEALTH_URL="${HEALTH_URL:-http://127.0.0.1:9010/api/v1/health}"

echo "Checking backend health at $HEALTH_URL..."

if curl -fsS "$HEALTH_URL" > /dev/null 2>&1; then
    echo "Backend is healthy."
    exit 0
else
    echo "Backend health check failed."
    echo "Tip: ensure the container is running:"
    echo "  docker compose -f $COMPOSE_FILE ps"
    exit 1
fi
