# Deprecated Python packages

These Python packages have been replaced by Rust crates:

- `forge-sdk` (Python) → `crates/forge-sdk/` (Rust)
- `forge-cu-mcp` (Python) → `crates/forge-mcp/` (Rust)

The Python code is kept here temporarily for reference.
It will be deleted in a future commit once the Rust replacements
are fully verified.

To install the Rust versions:

```bash
cargo install --path crates/forge-sdk    # library (use as dependency)
cargo install --path crates/forge-mcp    # MCP server binary
```

