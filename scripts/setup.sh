#!/bin/bash
# Forge — One-command setup for development and publishing
set -e

echo "=== Forge Setup ==="

# 1. Rust build
echo "Building Rust..."
cargo build --release

# 2. Python SDK
echo "Installing Python SDK..."
pip install -e sdk/python/ 2>/dev/null || pip install -e sdk/python/

# 3. MCP Server
echo "Installing MCP Server..."
pip install -e mcp/ 2>/dev/null || pip install -e mcp/

# 4. Build tools
echo "Installing build tools..."
pip install build twine 2>/dev/null || true

echo ""
echo "=== Setup Complete ==="
echo "Binaries:  target/release/forge, target/release/forged"
echo "Python:    forge_sdk, forge_mcp installed"
echo ""
echo "Quick start:"
echo "  forged node -m 'qwen2.5:0.5b' --ledger forge-ledger.json"
echo "  curl localhost:3000/v1/forge/balance"
echo ""
echo "To publish packages, fill in .env and run: ./scripts/publish.sh"
