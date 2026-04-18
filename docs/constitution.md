# The Tirami Constitution

**Version:** 1.0 (Phase 18.1) · 2026-04-18.
**Status:** Ratified. Violations are build-breaking.
**Enforcement:** `crates/tirami-ledger/src/governance.rs` —
`MUTABLE_GOVERNANCE_PARAMETERS` + `IMMUTABLE_CONSTITUTIONAL_PARAMETERS`.

---

This document is the single source of truth for what governance
*can* and *cannot* change about Tirami. The accompanying code
enforces this at compile + test time: anything not on the mutable
whitelist cannot even be *recorded* as a governance proposal.

The goal is credible neutrality. A currency's value rests on the
belief that its rules will not be rewritten under pressure. Tirami
can never achieve Bitcoin-grade mathematical immutability because
its runtime is upgradeable, but we can lay down a written
Constitution that makes rule-change attempts visible, loud, and
socially expensive.

---

## Article I — The Economic Foundation (immutable)

The following parameters define what TRM *is*. Changing any of
them would produce a different currency. Governance cannot change
them; a hostile software fork that did so would be publicly
identifiable as "not Tirami".

| Parameter | Value | Why immutable |
|-----------|-------|---------------|
| `TOTAL_TRM_SUPPLY` | 21 × 10⁹ TRM | The scarcity claim. Raise this and every holder's position dilutes. The whole "compute-backed currency" narrative collapses. |
| `FLOPS_PER_CU` | 10⁹ FLOP | Principle 1: 1 TRM ↔ 10⁹ FLOP. This is the bridge between computational work and monetary issuance. |
| `HALVING_EPOCH_FUNCTION` | 50% / 75% / 87.5% / ... | The issuance curve. Predictable scarcity depends on this being unchangeable. |
| `INITIAL_YIELD_RATE` | 0.001 / hour | Base yield for idle capacity. Changing this retroactively would rewrite holders' expectations. |

## Article II — Slashing Floor (immutable)

Minimum slash penalties for specific offenses. These can be
*applied more strictly* by the running code, but the floor itself
is Constitutional — a chain that weakens these is weakening the
honesty guarantee, which is a protocol-breaking change.

| Parameter | Value | Offense |
|-----------|-------|---------|
| `SLASH_RATE_MINOR` | 5% | Minor collusion signal |
| `SLASH_RATE_MAJOR` | 20% | Major collusion or audit failure |
| `SLASH_RATE_CRITICAL` | 50% | Critical collusion + repeated audit failure |
| `AUDIT_FAIL_TRUST_PENALTY` | 0.3 (major tier) | Bridged from `AuditVerdict::Failed` |

## Article III — Cryptographic Invariants (immutable)

These are the cryptographic guarantees every Tirami node depends
on. Changing them retroactively invalidates every historical
signature and turns existing trade records into orphaned bytes.

| Parameter | Meaning |
|-----------|---------|
| `ED25519_SIGNATURE_REQUIRED` | Every `TradeRecord` + `LoanRecord` + audit challenge is Ed25519-signed. |
| `DUAL_SIGNATURE_REQUIRED` | Every paid trade requires BOTH provider and consumer signatures. |
| `NONCE_REPLAY_DEFENSE_ENABLED` | v2 trades carry a 128-bit nonce; replays are rejected. |
| `CANONICAL_BYTES_V1_FORMAT` | Legacy (zero-nonce) canonical byte layout. |
| `CANONICAL_BYTES_V2_FORMAT` | Phase-17 (non-zero-nonce) canonical byte layout. |

Breaking any of these is equivalent to forking the protocol.

## Article IV — Trust & Identity Invariants (immutable)

| Parameter | Value | Role |
|-----------|-------|------|
| `DEFAULT_REPUTATION` | 0.5 | Starting reputation for a new node. Shifting this changes how peer trust accrues. |
| `COLD_START_CREDIT` | 0.3 | Welcome credit factor. |
| `COLLATERAL_BURN_ON_DEFAULT` | 1.0 | Defaulters lose 100% of collateral, always. Diluting this would incentivize default. |

## Article V — Governance Meta (immutable)

Governance cannot weaken its own bootstrap protection. These
parameters are the gates through which any proposal must pass.

| Parameter | Value |
|-----------|-------|
| `GOVERNANCE_MIN_REPUTATION` | 0.7 |
| `GOVERNANCE_MIN_STAKE` | 1 000 TRM |
| `GOVERNANCE_WHITELIST_CONTENTS` | This Constitution itself |

## Article VI — What governance CAN change (whitelist)

The `MUTABLE_GOVERNANCE_PARAMETERS` array in
`crates/tirami-ledger/src/governance.rs` is the exhaustive list.
Every entry has a one-line justification here:

### Lending parameters (operational tuning)

| Parameter | Why mutable |
|-----------|-------------|
| `WELCOME_LOAN_AMOUNT` | Bootstrap incentive; operators must be able to tune as network matures. |
| `MAX_LTV_RATIO` | Risk management under changing market conditions. |
| `MIN_RESERVE_RATIO` | Circuit-breaker tightness. |
| `DEFAULT_RATE_THRESHOLD` | Default-cascade trigger. |
| `VELOCITY_LIMIT_LOANS_PER_MINUTE` | Throughput tuning. |
| `MIN_CREDIT_FOR_BORROWING` | Onboarding strictness. |

### Market pricing (supply/demand response)

| Parameter | Why mutable |
|-----------|-------------|
| `BASE_TRM_PER_TOKEN` | Base pricing; must respond to hardware costs. |
| `TIER_SMALL_CU_PER_TOKEN` | Tier pricing calibration. |
| `TIER_FRONTIER_CU_PER_TOKEN` | Tier pricing calibration. |

### Sybil / rate-limit knobs (DDoS response)

| Parameter | Why mutable |
|-----------|-------------|
| `WELCOME_LOAN_SYBIL_THRESHOLD` | Attack-surface tuning. |
| `WELCOME_LOAN_PER_BUCKET_CAP` | ASN-level welcome loan limit. |
| `ASN_RATE_LIMIT_PER_SEC` | Per-ASN message rate limit. |
| `MAX_CONCURRENT_CONNECTIONS` | Transport-level connection cap. |

### Audit tuning (operational policy)

| Parameter | Why mutable |
|-----------|-------------|
| `AUDIT_SAMPLE_RATE` | Light-audit fire rate. |
| `AUDIT_VALIDATOR_COUNT` | Validators per challenge. |
| `HEAVY_AUDIT_SAMPLE_RATE` | Heavy audit (2-of-3 quorum) probability. |

### Staking bonus curves

| Parameter | Why mutable |
|-----------|-------------|
| `STAKE_DURATION_7D_MULTIPLIER` | Lock-duration incentive. |
| `STAKE_DURATION_30D_MULTIPLIER` | Lock-duration incentive. |
| `STAKE_DURATION_90D_MULTIPLIER` | Lock-duration incentive. |

### Anchor timing (infrastructure cost)

| Parameter | Why mutable |
|-----------|-------------|
| `ANCHOR_INTERVAL_SECS` | On-chain anchor frequency. |
| `CHECKPOINT_INTERVAL_SECS` | Trade-log seal frequency. |
| `CHECKPOINT_RETAIN_SECS` | In-memory retention window. |
| `SLASHING_INTERVAL_SECS` | Slashing-sweep frequency. |

## Article VII — Amendment

Adding or removing an entry on the mutable whitelist is itself
a Constitutional amendment. Amendments require:

1. A `ProposalKind::ProtocolUpgrade` passes the normal governance
   process (stake-weighted super-majority).
2. The PR that updates `MUTABLE_GOVERNANCE_PARAMETERS` and this
   document is tagged `constitutional-amendment` and held for a
   14-day public review period.
3. The amendment is recorded at the bottom of this file with
   the date, proposer, and rationale.

Removing an entry from `IMMUTABLE_CONSTITUTIONAL_PARAMETERS`
(as opposed to *moving* it to the mutable list) is NOT an
amendment — it's a protocol fork, and the resulting software
is no longer Tirami.

## Article VIII — Emergency Powers

The single emergency lever available to governance is
`ProposalKind::EmergencyPause`. It halts new trade execution
without changing any parameter. Once the emergency clears, normal
governance resumes. The pause itself has an automatic expiry
(currently 24 hours; tunable via `ANCHOR_INTERVAL_SECS`-class
parameters but bounded).

Note that pause does NOT rewrite history: all trades before the
pause remain valid; all signatures remain verifiable; all
slashing events remain recorded.

## Article IX — Auditor's Role

External security auditors reviewing Tirami should treat this
Constitution as a contract. If the audit finds a way to change
a Constitutional parameter through a side channel — e.g. through
a race condition, a deserialization quirk, a governance
coordination attack that bypasses the stake threshold — that is
a **Critical** finding by definition. The Constitution is
promises-made-to-users; any hole in it must be Critical.

## Article X — Relationship to Code

The authoritative enforcement lives in
`crates/tirami-ledger/src/governance.rs`:

```rust
pub const MUTABLE_GOVERNANCE_PARAMETERS: &[&str] = &[ ... ];
pub const IMMUTABLE_CONSTITUTIONAL_PARAMETERS: &[&str] = &[ ... ];

pub fn is_mutable_parameter(name: &str) -> bool { ... }
pub fn is_constitutional_parameter(name: &str) -> bool { ... }
```

The `create_proposal` method rejects any
`ProposalKind::ChangeParameter` whose `name` is not in the
mutable list with `GovernanceError::ConstitutionalParameter`.

The regression suite in the `tests` module of `governance.rs`
asserts:

- Core Constitutional parameters (`TOTAL_TRM_SUPPLY`,
  `FLOPS_PER_CU`, `SLASH_RATE_*`, `DUAL_SIGNATURE_REQUIRED`,
  `NONCE_REPLAY_DEFENSE_ENABLED`, `GOVERNANCE_WHITELIST_CONTENTS`)
  are rejected by `create_proposal`.
- Mutable whitelist members (`WELCOME_LOAN_AMOUNT`,
  `MAX_LTV_RATIO`, `ANCHOR_INTERVAL_SECS`) are accepted.
- `EmergencyPause` and `ProtocolUpgrade` are always accepted.
- The mutable and immutable lists are disjoint.
- An unknown parameter name is rejected (Constitutional by
  default — you can't bypass the Constitution by inventing a
  new parameter name).

Any commit that weakens these checks is a Constitutional
violation and must be rejected at code review.

## Amendment log

*(No amendments yet. When the first amendment ratifies, it
goes here with the form: "2026-MM-DD · Proposer · Rationale ·
Commit hash".)*
