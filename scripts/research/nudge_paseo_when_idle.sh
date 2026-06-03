#!/usr/bin/env bash
set -euo pipefail

AGENT_ID="${1:-8c4491e}"
shift || true

MESSAGE="${*:-${PASEO_NUDGE_MESSAGE:-keep going}}"
INTERVAL_SECONDS="${PASEO_NUDGE_INTERVAL_SECONDS:-30}"
SEND_ON_START="${PASEO_NUDGE_SEND_ON_START:-1}"
PASEO_BIN="${PASEO_BIN:-paseo}"

if ! command -v "$PASEO_BIN" >/dev/null 2>&1; then
  echo "paseo binary not found: $PASEO_BIN" >&2
  exit 127
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to parse paseo inspect --json output" >&2
  exit 127
fi

agent_status() {
  "$PASEO_BIN" inspect --json "$AGENT_ID" | jq -r '.Status // .status // empty'
}

send_prompt() {
  printf '%s sending to %s: %s\n' "$(date -Is)" "$AGENT_ID" "$MESSAGE"
  "$PASEO_BIN" send --no-wait "$AGENT_ID" --prompt "$MESSAGE" >/dev/null
}

last_status=""

printf '%s watching paseo agent %s every %ss\n' "$(date -Is)" "$AGENT_ID" "$INTERVAL_SECONDS"

while true; do
  if ! current_status="$(agent_status)"; then
    printf '%s failed to inspect agent %s\n' "$(date -Is)" "$AGENT_ID" >&2
    sleep "$INTERVAL_SECONDS"
    continue
  fi

  if [[ -z "$current_status" ]]; then
    printf '%s inspect returned no status for agent %s\n' "$(date -Is)" "$AGENT_ID" >&2
    sleep "$INTERVAL_SECONDS"
    continue
  fi

  if [[ "$current_status" != "$last_status" ]]; then
    printf '%s status: %s\n' "$(date -Is)" "$current_status"
  fi

  if [[ "$current_status" == "idle" && "$last_status" != "idle" ]]; then
    if [[ -n "$last_status" || "$SEND_ON_START" == "1" ]]; then
      send_prompt
    fi
  fi

  last_status="$current_status"
  sleep "$INTERVAL_SECONDS"
done
