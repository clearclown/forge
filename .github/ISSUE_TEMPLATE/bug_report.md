---
name: Bug report
about: Report a bug in the Forge protocol or tooling
title: "[BUG] "
labels: bug
assignees: ''
---

## Summary

One-sentence description of the problem.

## Environment

- Forge version: (run `forge --version` or paste `git rev-parse HEAD`)
- OS + arch: (e.g., macOS 14.5 / arm64, Ubuntu 22.04 / x86_64)
- Rust version: (run `rustc --version`)
- Model: (e.g., `smollm2:135m`, `qwen2.5:0.5b`, or a local GGUF path)
- Backend: (Metal / CUDA / ROCm / CPU)

## Steps to reproduce

```bash
# Exact commands you ran
```

## Expected behavior

What should have happened.

## Actual behavior

What actually happened. Paste the full error message and any relevant
log output.

```text
(paste error / log output here)
```

## Additional context

- Is this a regression from a previous version?
- Does it reproduce on a fresh clone?
- Does `bash scripts/demo-e2e.sh` succeed on this machine?
- Any relevant configuration in `forge-ledger.json` or env vars?
