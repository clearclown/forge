#!/bin/bash
# Forge Live Demo — run this after: cargo build --release
# Usage: ./examples/demo.sh
set -e

FORGE="http://127.0.0.1:3000"
BIN="target/release/forged"

if [ ! -f "$BIN" ]; then
    echo "Build first: cargo build --release"
    exit 1
fi

echo "Starting Forge node..."
$BIN node -m "qwen2.5:0.5b" --port 3000 --ledger /tmp/forge-demo.json 2>/dev/null &
PID=$!
trap "kill $PID 2>/dev/null" EXIT

for i in $(seq 1 30); do
    LOADED=$(curl -s $FORGE/health 2>/dev/null | python3 -c "import sys,json; print(json.load(sys.stdin).get('model_loaded',''))" 2>/dev/null)
    [ "$LOADED" = "True" ] && break
    sleep 1
done

echo ""
echo "=== FORGE: Computation is Currency ==="
echo ""

echo "Balance: $(curl -s $FORGE/v1/forge/balance | python3 -c 'import sys,json; print(json.load(sys.stdin)["effective_balance"])') CU"
echo ""

echo "Inference: 'Say hello in Japanese'"
RESULT=$(curl -s $FORGE/v1/chat/completions -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"Say hello in Japanese, one word"}],"max_tokens":8}')
echo "$RESULT" | python3 -c "
import sys,json; d=json.load(sys.stdin)
print(f'  Response: {d[\"choices\"][0][\"message\"][\"content\"]}')
print(f'  Cost: {d[\"x_forge\"][\"cu_cost\"]} CU')
" 2>/dev/null
echo ""

echo "Trades: $(curl -s $FORGE/v1/forge/trades | python3 -c 'import sys,json; print(json.load(sys.stdin)["count"])') recorded"
echo "Merkle: $(curl -s $FORGE/v1/forge/network | python3 -c 'import sys,json; print(json.load(sys.stdin)["merkle_root"][:24])')..."
echo ""
echo "Every watt produced intelligence. Every CU is accountable."
