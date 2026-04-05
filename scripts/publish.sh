#!/bin/bash
# Forge — Publish all packages (requires .env credentials)
set -e

if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

echo "=== Publishing Forge Packages ==="

# 1. crates.io
if [ -n "$CARGO_REGISTRY_TOKEN" ]; then
    echo "Publishing to crates.io..."
    cargo publish -p forge-core --token "$CARGO_REGISTRY_TOKEN"
    echo "Waiting for index update..."
    sleep 30
    cargo publish -p forge-ledger --token "$CARGO_REGISTRY_TOKEN"
    sleep 30
    cargo publish -p forge-lightning --token "$CARGO_REGISTRY_TOKEN"
    echo "crates.io: DONE"
else
    echo "SKIP crates.io (no CARGO_REGISTRY_TOKEN in .env)"
    echo "  Get token: https://crates.io/settings/tokens"
fi

echo ""

# 2. PyPI — SDK
if [ -n "$TWINE_PASSWORD" ]; then
    echo "Publishing forge-sdk to PyPI..."
    cd sdk/python
    python -m build
    twine upload dist/* --username "$TWINE_USERNAME" --password "$TWINE_PASSWORD"
    cd ../..
    echo "forge-sdk: DONE"

    echo "Publishing forge-mcp to PyPI..."
    cd mcp
    python -m build
    twine upload dist/* --username "$TWINE_USERNAME" --password "$TWINE_PASSWORD"
    cd ..
    echo "forge-mcp: DONE"
else
    echo "SKIP PyPI (no TWINE_PASSWORD in .env)"
    echo "  Get token: https://pypi.org/manage/account/token/"
fi

echo ""
echo "=== Publishing Complete ==="
echo ""
echo "Next steps:"
echo "  1. Submit to MCP Market: https://mcpmarket.com"
echo "  2. Submit to Claude Marketplaces: https://claudemarketplaces.com"
echo "  3. Post to r/LocalLLaMA (see marketing/posts/reddit-localllama.md)"
echo "  4. Post Show HN (see marketing/posts/hackernews.md)"
echo "  5. Publish Dev.to article (see marketing/posts/devto-article.md)"
