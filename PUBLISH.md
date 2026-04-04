# Publishing Forge Packages

## 1. crates.io (Rust)

```bash
# Login (get token from https://crates.io/settings/tokens)
cargo login

# Publish in dependency order
cargo publish -p forge-core
sleep 30  # wait for index update
cargo publish -p forge-ledger
sleep 30
cargo publish -p forge-lightning

# Users can then: cargo add forge-ledger
```

## 2. PyPI (Python SDK)

```bash
# Install build tools
pip install build twine

# Build and publish SDK
cd sdk/python
python -m build
twine upload dist/*

# Build and publish MCP server
cd ../../mcp
python -m build
twine upload dist/*

# Users can then:
# pip install forge-sdk
# pip install forge-mcp
```

## 3. GitHub Release

Go to https://github.com/clearclown/forge/releases/tag/v0.2.0-alpha and click "Create release from tag".

## Post-Publish Verification

```bash
# Rust
cargo add forge-ledger  # should resolve from crates.io

# Python
pip install forge-sdk
python -c "from forge_sdk import ForgeClient; print('SDK OK')"

pip install forge-mcp
forge-mcp --help
```
