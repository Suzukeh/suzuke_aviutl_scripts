#!/bin/bash
set -e

PID_FILE="/tmp/opencode-web.pid"
LOG_FILE="/tmp/opencode-web.log"

if [ -f "$PID_FILE" ] && kill -0 "$(cat "$PID_FILE")" 2>/dev/null; then
    echo "[opencode-web] Already running (pid $(cat "$PID_FILE"))"
    exit 0
fi

if [ -z "$OPENCODE_SERVER_PASSWORD" ]; then
    echo "[opencode-web] WARNING: OPENCODE_SERVER_PASSWORD is not set."
    echo "[opencode-web] The web server will be UNSECURED."
    echo "[opencode-web] Set it via Codespaces secrets or export it manually."
fi

echo "[opencode-web] Starting opencode web on port 4096..."
nohup opencode web --port 4096 --hostname 0.0.0.0 > "$LOG_FILE" 2>&1 &
echo $! > "$PID_FILE"
echo "[opencode-web] Started (pid $(cat "$PID_FILE"))"
