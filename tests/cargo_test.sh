#!/bin/sh
set -e

psql -U auth -d postgres -c "DROP DATABASE auth;"
psql -U auth -d postgres -c "CREATE DATABASE auth;"

export AUTH_DATABASE_URL=postgres://auth:Welc0meAuth1@localhost:5432/auth
RUST_BACKTRACE=1 \
  cargo test --workspace --no-fail-fast
