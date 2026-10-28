#!/usr/bin/env sh
set -eu

BIN_PATH="${1:-target/release/vortex}"
if [ ! -f "$BIN_PATH" ]; then
    echo "Release binary not found at $BIN_PATH" >&2
    exit 1
fi

BIN_BYTES=$(wc -c < "$BIN_PATH" | tr -d ' ')
"$BIN_PATH" --hold-ms 600 &
PID=$!
sleep 0.15
RSS_KB=$(ps -o rss= -p "$PID" | tr -d ' ')
wait "$PID"
RSS_BYTES=$((RSS_KB * 1024))

for TIER in lite standard full; do
    "$BIN_PATH" --check-budgets "$TIER" --binary-bytes "$BIN_BYTES" --rss-bytes "$RSS_BYTES"
done
