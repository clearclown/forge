//! Tirami tokenomics: supply cap, halving, mint rate, rarity scoring.
//!
//! Implements the economic growth model from spec/tokenomics.md.
//! Bitcoin-inspired: 21B TRM cap, yield halving, deflationary supply curve.
//!
//! # Key formulas (spec/parameters.md §14, §15, §18)
//!
//! ```text
//! supply_factor = 1 - (total_minted / TOTAL_TRM_SUPPLY)
//! current_epoch = floor(log2(TOTAL_TRM_SUPPLY / (TOTAL_TRM_SUPPLY - total_minted)))
//! yield_rate    = INITIAL_YIELD_RATE / 2^epoch
//! effective_trm = base_rate(tier) × rarity_multiplier(tier) × supply_factor
//! fee           = trm_amount × TRANSACTION_FEE_RATE  (only when supply_factor ≤ FEE_ACTIVATION_THRESHOLD)
//! ```

/// Total TRM that can ever be minted. Analogous to Bitcoin's 21M BTC.
pub const TOTAL_TRM_SUPPLY: u64 = 21_000_000_000;

/// Initial availability yield rate per hour (multiplied by reputation).
/// Halves with each epoch.  Spec §15: `initial_yield_rate = 0.001 (/hr × reputation)`.
pub const INITIAL_YIELD_RATE: f64 = 0.001;

/// Transaction fee rate (fraction of TRM transferred). Activates when
/// supply_factor drops below FEE_ACTIVATION_THRESHOLD.
/// Spec §14: `transaction_fee_rate = 0.01 (1%)`.
pub const TRANSACTION_FEE_RATE: f64 = 0.01;

/// supply_factor threshold below which transaction fees activate.
/// Spec §14: `fee_activation_threshold = 0.1`.
pub const FEE_ACTIVATION_THRESHOLD: f64 = 0.1;

/// Rarity multiplier for Small tier (< 3B params).
/// Spec §18: `rarity_common_multiplier = 1.0`.
pub const RARITY_COMMON: f64 = 1.0;

/// Rarity multiplier for Medium tier (3–14B params).
/// Spec §18: `rarity_uncommon_multiplier = 1.5`.
pub const RARITY_UNCOMMON: f64 = 1.5;

/// Rarity multiplier for Large tier (14–70B params).
/// Spec §18: `rarity_rare_multiplier = 3.0`.
pub const RARITY_RARE: f64 = 3.0;

/// Rarity multiplier for Frontier tier (70B+ params).
/// Spec §18: `rarity_legendary_multiplier = 10.0`.
pub const RARITY_LEGENDARY: f64 = 10.0;

// ---------------------------------------------------------------------------
// Core functions
// ---------------------------------------------------------------------------

/// Compute the supply factor: how much of the cap remains unminted.
///
/// Returns a value in `[0.0, 1.0]`. At `0.0` the supply is exhausted.
/// Spec §14: `supply_factor = 1 - (total_minted / TOTAL_TRM_SUPPLY)`.
pub fn supply_factor(total_minted: u64) -> f64 {
    if total_minted >= TOTAL_TRM_SUPPLY {
        return 0.0;
    }
    1.0 - (total_minted as f64 / TOTAL_TRM_SUPPLY as f64)
}

/// Compute the current halving epoch from total minted TRM.
///
/// Epoch 0: 0 → 50% minted. Epoch 1: 50 → 75%. Epoch 2: 75 → 87.5%, etc.
/// Spec §15: `current_epoch = floor(log2(cap / (cap - minted)))`.
pub fn current_epoch(total_minted: u64) -> u32 {
    if total_minted == 0 {
        return 0;
    }
    let remaining = TOTAL_TRM_SUPPLY.saturating_sub(total_minted);
    if remaining == 0 {
        return 32; // cap reached — conceptually infinite epoch
    }
    let ratio = TOTAL_TRM_SUPPLY as f64 / remaining as f64;
    (ratio.log2().floor() as u32).min(32)
}

/// Compute the yield rate for the current epoch.
///
/// Halves with each epoch: `yield = INITIAL_YIELD_RATE / 2^epoch`.
/// Spec §15: `yield_rate = initial_yield_rate / 2^current_epoch`.
pub fn epoch_yield_rate(total_minted: u64) -> f64 {
    let epoch = current_epoch(total_minted);
    INITIAL_YIELD_RATE / (1u64 << epoch.min(30)) as f64
}

/// Compute the effective TRM earned per inference token for a given model tier.
///
/// Applies: `base_rate(tier) × rarity_multiplier(tier) × supply_factor`.
/// Spec tokenomics.md §1.2, §5.1.
///
/// | tier      | base | rarity | effective (at genesis) |
/// |-----------|------|--------|------------------------|
/// | "small"   | 1.0  | 1.0    | 1.0                    |
/// | "medium"  | 3.0  | 1.5    | 4.5                    |
/// | "large"   | 8.0  | 3.0    | 24.0                   |
/// | "frontier"| 20.0 | 10.0   | 200.0                  |
pub fn effective_mint_rate(tier: &str, total_minted: u64) -> f64 {
    let base: f64 = match tier {
        "small" => 1.0,
        "medium" => 3.0,
        "large" => 8.0,
        "frontier" => 20.0,
        _ => 1.0,
    };
    let rarity: f64 = match tier {
        "small" => RARITY_COMMON,
        "medium" => RARITY_UNCOMMON,
        "large" => RARITY_RARE,
        "frontier" => RARITY_LEGENDARY,
        _ => RARITY_COMMON,
    };
    base * rarity * supply_factor(total_minted)
}

/// Compute the transaction fee for a TRM transfer.
///
/// Returns `0` if `supply_factor > FEE_ACTIVATION_THRESHOLD` (early-epoch, fee-free).
/// Otherwise returns `floor(trm_amount × TRANSACTION_FEE_RATE)`.
/// Spec §14, tokenomics.md §7.
pub fn transaction_fee(trm_amount: u64, total_minted: u64) -> u64 {
    let sf = supply_factor(total_minted);
    if sf > FEE_ACTIVATION_THRESHOLD {
        return 0;
    }
    (trm_amount as f64 * TRANSACTION_FEE_RATE) as u64
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- supply_factor ---

    #[test]
    fn test_supply_factor_at_zero_minted() {
        assert_eq!(supply_factor(0), 1.0);
    }

    #[test]
    fn test_supply_factor_at_half_minted() {
        let half = TOTAL_TRM_SUPPLY / 2;
        let sf = supply_factor(half);
        assert!((sf - 0.5).abs() < 0.001, "expected ~0.5, got {sf}");
    }

    #[test]
    fn test_supply_factor_at_full_minted() {
        assert_eq!(supply_factor(TOTAL_TRM_SUPPLY), 0.0);
    }

    #[test]
    fn test_supply_factor_beyond_cap() {
        assert_eq!(supply_factor(TOTAL_TRM_SUPPLY + 1), 0.0);
    }

    #[test]
    fn test_supply_factor_at_90_percent() {
        let minted = (TOTAL_TRM_SUPPLY as f64 * 0.9) as u64;
        let sf = supply_factor(minted);
        assert!((sf - 0.1).abs() < 0.001, "expected ~0.1, got {sf}");
    }

    #[test]
    fn test_supply_factor_decreases_monotonically() {
        let steps = [0u64, 1_000_000, 5_000_000_000, 10_000_000_000, 20_000_000_000];
        for w in steps.windows(2) {
            assert!(
                supply_factor(w[0]) > supply_factor(w[1]),
                "supply_factor({}) should be > supply_factor({})",
                w[0],
                w[1]
            );
        }
    }

    // --- current_epoch ---

    #[test]
    fn test_epoch_zero_at_start() {
        assert_eq!(current_epoch(0), 0);
    }

    #[test]
    fn test_epoch_one_at_half() {
        // At exactly 50% minted remaining = 50% → ratio = 2.0 → log2 = 1.0
        assert_eq!(current_epoch(TOTAL_TRM_SUPPLY / 2), 1);
    }

    #[test]
    fn test_epoch_two_at_75_percent() {
        let minted = (TOTAL_TRM_SUPPLY as f64 * 0.75) as u64;
        assert_eq!(current_epoch(minted), 2);
    }

    #[test]
    fn test_epoch_three_at_875_percent() {
        let minted = (TOTAL_TRM_SUPPLY as f64 * 0.875) as u64;
        assert_eq!(current_epoch(minted), 3);
    }

    #[test]
    fn test_epoch_at_cap() {
        assert_eq!(current_epoch(TOTAL_TRM_SUPPLY), 32);
    }

    #[test]
    fn test_epoch_increases_with_minting() {
        let e0 = current_epoch(0);
        let e1 = current_epoch(TOTAL_TRM_SUPPLY / 2);
        let e2 = current_epoch((TOTAL_TRM_SUPPLY as f64 * 0.75) as u64);
        assert!(e0 < e1 && e1 < e2);
    }

    // --- epoch_yield_rate ---

    #[test]
    fn test_yield_rate_halves_each_epoch() {
        let y0 = epoch_yield_rate(0);
        let y1 = epoch_yield_rate(TOTAL_TRM_SUPPLY / 2);
        let y2 = epoch_yield_rate((TOTAL_TRM_SUPPLY as f64 * 0.75) as u64);
        assert!((y0 - 0.001).abs() < 1e-9, "epoch 0 yield should be 0.001, got {y0}");
        assert!((y1 - 0.0005).abs() < 1e-9, "epoch 1 yield should be 0.0005, got {y1}");
        assert!((y2 - 0.00025).abs() < 1e-9, "epoch 2 yield should be 0.00025, got {y2}");
    }

    #[test]
    fn test_yield_rate_decreases_monotonically() {
        let y0 = epoch_yield_rate(0);
        let y1 = epoch_yield_rate(TOTAL_TRM_SUPPLY / 2);
        let y2 = epoch_yield_rate((TOTAL_TRM_SUPPLY as f64 * 0.75) as u64);
        let y3 = epoch_yield_rate((TOTAL_TRM_SUPPLY as f64 * 0.875) as u64);
        assert!(y0 > y1 && y1 > y2 && y2 > y3);
    }

    // --- effective_mint_rate ---

    #[test]
    fn test_effective_mint_rate_small_at_start() {
        // base=1.0, rarity=1.0, sf=1.0 → 1.0
        let rate = effective_mint_rate("small", 0);
        assert!((rate - 1.0).abs() < 0.001, "expected 1.0, got {rate}");
    }

    #[test]
    fn test_effective_mint_rate_medium_at_start() {
        // base=3.0, rarity=1.5, sf=1.0 → 4.5
        let rate = effective_mint_rate("medium", 0);
        assert!((rate - 4.5).abs() < 0.001, "expected 4.5, got {rate}");
    }

    #[test]
    fn test_effective_mint_rate_large_at_start() {
        // base=8.0, rarity=3.0, sf=1.0 → 24.0
        let rate = effective_mint_rate("large", 0);
        assert!((rate - 24.0).abs() < 0.001, "expected 24.0, got {rate}");
    }

    #[test]
    fn test_effective_mint_rate_frontier_at_start() {
        // base=20.0, rarity=10.0, sf=1.0 → 200.0
        let rate = effective_mint_rate("frontier", 0);
        assert!((rate - 200.0).abs() < 0.001, "expected 200.0, got {rate}");
    }

    #[test]
    fn test_effective_mint_rate_decreases_with_supply() {
        let rate_early = effective_mint_rate("small", 0);
        let rate_late = effective_mint_rate("small", TOTAL_TRM_SUPPLY / 2);
        assert!(
            rate_early > rate_late,
            "mint rate should decrease as supply is consumed"
        );
    }

    #[test]
    fn test_effective_mint_rate_zero_at_cap() {
        let rate = effective_mint_rate("frontier", TOTAL_TRM_SUPPLY);
        assert_eq!(rate, 0.0);
    }

    #[test]
    fn test_effective_mint_rate_unknown_tier_falls_back_to_small() {
        let rate_unknown = effective_mint_rate("unknown", 0);
        let rate_small = effective_mint_rate("small", 0);
        assert!((rate_unknown - rate_small).abs() < 1e-9);
    }

    // --- transaction_fee ---

    #[test]
    fn test_transaction_fee_zero_when_supply_above_threshold() {
        // supply_factor = 1.0 > 0.1 → no fee
        assert_eq!(transaction_fee(1000, 0), 0);
    }

    #[test]
    fn test_transaction_fee_zero_exactly_at_threshold() {
        // supply_factor = 0.1 is NOT above threshold (condition: sf > threshold)
        // so fee activates at exactly 0.1
        let minted = (TOTAL_TRM_SUPPLY as f64 * 0.9) as u64;
        // supply_factor ≈ 0.1 → fee should activate
        let fee = transaction_fee(1000, minted);
        // 1% of 1000 = 10
        assert_eq!(fee, 10, "expected fee=10 at 90% minted, got {fee}");
    }

    #[test]
    fn test_transaction_fee_nonzero_when_supply_near_exhaustion() {
        // supply_factor = 0.05 (95% minted)
        let minted = (TOTAL_TRM_SUPPLY as f64 * 0.95) as u64;
        let fee = transaction_fee(1000, minted);
        assert_eq!(fee, 10, "expected fee=10 (1% of 1000), got {fee}");
    }

    #[test]
    fn test_transaction_fee_scales_with_amount() {
        let minted = (TOTAL_TRM_SUPPLY as f64 * 0.95) as u64;
        assert_eq!(transaction_fee(10_000, minted), 100);
        assert_eq!(transaction_fee(100_000, minted), 1000);
    }

    #[test]
    fn test_transaction_fee_zero_amount_is_zero() {
        let minted = (TOTAL_TRM_SUPPLY as f64 * 0.95) as u64;
        assert_eq!(transaction_fee(0, minted), 0);
    }

    // --- constants ---

    #[test]
    fn test_total_trm_supply_is_21_billion() {
        assert_eq!(TOTAL_TRM_SUPPLY, 21_000_000_000);
    }

    #[test]
    fn test_rarity_constants() {
        assert_eq!(RARITY_COMMON, 1.0);
        assert_eq!(RARITY_UNCOMMON, 1.5);
        assert_eq!(RARITY_RARE, 3.0);
        assert_eq!(RARITY_LEGENDARY, 10.0);
    }

    #[test]
    fn test_initial_yield_rate_constant() {
        assert!((INITIAL_YIELD_RATE - 0.001).abs() < 1e-12);
    }

    #[test]
    fn test_fee_rate_and_threshold_constants() {
        assert!((TRANSACTION_FEE_RATE - 0.01).abs() < 1e-12);
        assert!((FEE_ACTIVATION_THRESHOLD - 0.1).abs() < 1e-12);
    }
}
