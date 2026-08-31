#!/usr/bin/env sh
set -eu

RPC_URL=${SOROBAN_RPC_URL:-}

if [ -z "$RPC_URL" ]; then
  printf 'error: SOROBAN_RPC_URL is required\n' >&2
  printf 'usage: SOROBAN_RPC_URL=https://... %s\n' "$0" >&2
  exit 2
fi

if ! command -v curl >/dev/null 2>&1; then
  printf 'error: curl is required\n' >&2
  exit 2
fi

probe() {
  method=$1
  response=$(
    curl -fsS \
      -H 'Content-Type: application/json' \
      --data "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"$method\"}" \
      "$RPC_URL"
  ) || {
    printf 'error: %s request failed\n' "$method" >&2
    return 1
  }

  if printf '%s' "$response" | grep -Eq '"error"[[:space:]]*:'; then
    printf 'error: %s returned a JSON-RPC error: %s\n' "$method" "$response" >&2
    return 1
  fi

  if ! printf '%s' "$response" | grep -Eq '"result"[[:space:]]*:'; then
    printf 'error: %s response has no result: %s\n' "$method" "$response" >&2
    return 1
  fi

  printf '%s: ok\n' "$method"
}

probe getHealth
probe getLatestLedger
printf 'RPC endpoint healthy: %s\n' "$RPC_URL"
