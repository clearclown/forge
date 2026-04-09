## What this PR does

One-sentence summary of the change.

## Why

What problem does this solve? Reference any related issue with `#N`.

## Changes

- Bullet list of concrete changes
- Ideally one bullet per commit

## Checklist

- [ ] Tests added or updated (`cargo test --workspace` is green)
- [ ] `bash scripts/verify-impl.sh` is 95/95 GREEN (or new assertions added)
- [ ] `bash scripts/demo-e2e.sh` still succeeds (if touching the runtime path)
- [ ] Docs updated (`docs/` or inline rustdoc comments)
- [ ] `CHANGELOG.md` entry under `[Unreleased]`
- [ ] No new `cargo clippy` warnings beyond the baseline
- [ ] If a numeric constant was added/changed, `forge-economics/spec/parameters.md` is also updated (in a separate PR or as part of this PR)
- [ ] Commit messages follow the conventional-commits style (`feat:`, `fix:`, `docs:`, etc.)

## Testing

Describe how you verified this change works. Paste test output if relevant.

## Screenshots / output

If this changes user-facing behavior, include before/after output.

## Notes for reviewer

Anything the reviewer should know up front. Design decisions, open
questions, known rough edges, follow-up work.
