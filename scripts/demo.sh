#!/bin/bash
# Forge — Run the full live demo (for screenshots/recordings)
set -e

BIN="/Users/ablaze/Projects/forge/target/release/forged"
MODEL="qwen2.5:0.5b"
PORT=3000
LEDGER="/tmp/forge-demo-ledger.json"

if [ ! -f "$BIN" ]; then
    echo "Build first: cargo build --release"
    exit 1
fi

kill $(pgrep forged) 2>/dev/null || true
sleep 1
rm -f "$LEDGER"

echo "Starting Forge node..."
$BIN node -m "$MODEL" --port $PORT --ledger "$LEDGER" 2>/dev/null &
FPID=$!
trap "kill $FPID 2>/dev/null" EXIT

until curl -sf http://127.0.0.1:$PORT/health > /dev/null 2>&1; do sleep 0.5; done
echo "Ready."
echo ""

echo "=== FORGE: Computation is Currency ==="
echo ""

echo "Balance:"
curl -s http://127.0.0.1:$PORT/v1/forge/balance | python3 -m json.tool
echo ""

echo "Inference #1:"
curl -s http://127.0.0.1:$PORT/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"Say hello in Japanese, one word"}],"max_tokens":8}' \
  | python3 -m json.tool
echo ""

echo "Inference #2:"
curl -s http://127.0.0.1:$PORT/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"What is 2+2? Just the number"}],"max_tokens":4}' \
  | python3 -m json.tool
echo ""

echo "Trades:"
curl -s http://127.0.0.1:$PORT/v1/forge/trades | python3 -m json.tool
echo ""

echo "Network + Merkle Root:"
curl -s http://127.0.0.1:$PORT/v1/forge/network | python3 -m json.tool
echo ""

echo "Safety:"
curl -s http://127.0.0.1:$PORT/v1/forge/safety | python3 -m json.tool
echo ""

echo "=== Every watt produced intelligence. Every CU is accountable. ==="
