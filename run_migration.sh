#!/usr/bin/env bash
set -Eeuo pipefail

# Load DATABASE_URL from environment or .env (without overwriting an existing env var)
if [[ -z "${DATABASE_URL:-}" ]]; then
  if [[ -f ".env" ]]; then
    # shellcheck disable=SC1091
    set -a
    source .env
    set +a
  fi
fi

if [[ -z "${DATABASE_URL:-}" ]]; then
  echo "ERROR: DATABASE_URL is not set. Set it in your environment or in .env" >&2
  exit 1
fi

DB_SCHEMA="dental"

# Modify DATABASE_URL to set search_path to dental schema
# This ensures seaql_migrations table is created in dental schema
if [[ "$DATABASE_URL" == *"?"* ]]; then
  MODIFIED_URL="${DATABASE_URL}&options=-c%20search_path%3D${DB_SCHEMA}%2Cpublic"
else
  MODIFIED_URL="${DATABASE_URL}?options=-c%20search_path%3D${DB_SCHEMA}%2Cpublic"
fi

echo "Running database migrations against schema: ${DB_SCHEMA} ..."
export DATABASE_URL="$MODIFIED_URL"
cargo run --manifest-path migration/Cargo.toml --bin run_church_migrations

echo "Migrations completed!"
