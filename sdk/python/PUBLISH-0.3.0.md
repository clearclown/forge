# forge-sdk / forge-cu-mcp v0.3.0 Release Checklist

Built: 2026-04-09 (Phase 10 P1)

## Artifacts (ready to upload)

```
/Users/ablaze/Projects/forge/sdk/python/dist/forge_sdk-0.3.0-py3-none-any.whl    (8.6 KB)
/Users/ablaze/Projects/forge/sdk/python/dist/forge_sdk-0.3.0.tar.gz              (10.0 KB)
/Users/ablaze/Projects/forge/mcp/dist/forge_cu_mcp-0.3.0-py3-none-any.whl        (2.9 KB)
/Users/ablaze/Projects/forge/mcp/dist/forge_cu_mcp-0.3.0.tar.gz                  (2.8 KB)
```

`twine check`: **all 4 PASSED**.

## What's new in v0.3.0

- **20 new SDK methods** covering Phase 8 L2/L3/L4 endpoints:
  - L2 bank (8): `bank_portfolio`, `bank_tick`, `bank_set_strategy`,
    `bank_set_risk`, `bank_list_futures`, `bank_create_future`,
    `bank_risk_assessment`, `bank_optimize`
  - L4 agora (7): `agora_register`, `agora_list_agents`,
    `agora_reputation`, `agora_find`, `agora_stats`,
    `agora_snapshot`, `agora_restore`
  - L3 mind (5): `mind_init`, `mind_state`, `mind_improve`,
    `mind_budget`, `mind_stats`
- **20 matching MCP tools** with agent-oriented descriptions
- **27 new pytest tests** (all passing, mocked httpx)
- Zero new runtime dependencies

## Release commands (user must run — requires PyPI credentials)

```bash
# 1. Publish forge-sdk to PyPI
cd /Users/ablaze/Projects/forge/sdk/python
twine upload dist/forge_sdk-0.3.0*

# 2. Publish forge-cu-mcp to PyPI
cd /Users/ablaze/Projects/forge/mcp
twine upload dist/forge_cu_mcp-0.3.0*

# 3. Push tags to GitHub
cd /Users/ablaze/Projects/forge
git push origin forge-sdk-v0.3.0 forge-cu-mcp-v0.3.0

# 4. Create GitHub Release (manual via web UI or gh cli)
gh release create forge-sdk-v0.3.0 \
  sdk/python/dist/forge_sdk-0.3.0-py3-none-any.whl \
  sdk/python/dist/forge_sdk-0.3.0.tar.gz \
  --title "forge-sdk 0.3.0" \
  --notes "Phase 8 L2/L3/L4 coverage. See CHANGELOG.md."

gh release create forge-cu-mcp-v0.3.0 \
  mcp/dist/forge_cu_mcp-0.3.0-py3-none-any.whl \
  mcp/dist/forge_cu_mcp-0.3.0.tar.gz \
  --title "forge-cu-mcp 0.3.0" \
  --notes "Phase 8 L2/L3/L4 MCP tools. See CHANGELOG.md."
```

## Post-release verification

```bash
pip install forge-sdk==0.3.0
python -c "from forge_sdk import ForgeClient; c = ForgeClient(); dir_methods = [m for m in dir(c) if not m.startswith('_')]; print('total methods:', len(dir_methods)); print('has bank_portfolio:', hasattr(c, 'bank_portfolio'))"

pip install forge-cu-mcp==0.3.0
forge-mcp --help
```

Expected: `total methods: 38+`, `has bank_portfolio: True`.
