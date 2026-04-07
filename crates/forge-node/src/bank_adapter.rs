//! Adapter from forge-ledger state to forge-bank types.
//!
//! This is the single place where ComputeLedger lending state is reified
//! into forge-bank types. All `/v1/forge/bank/*` handlers use it.

use forge_bank::{
    BalancedStrategy, FuturesContract, Portfolio, PortfolioManager, RiskTolerance,
};
use forge_core::NodeId;
use forge_ledger::ComputeLedger;
use forge_ledger::lending::{max_borrowable, offered_interest_rate};

/// Wrapper combining PortfolioManager with a futures book.
/// PortfolioManager does not store futures, so we maintain them here.
pub struct BankServices {
    pub portfolio: PortfolioManager,
    pub futures: Vec<FuturesContract>,
}

impl BankServices {
    /// Default initial state: 10,000 CU cash, balanced strategy, balanced risk.
    pub fn new_default() -> Self {
        let portfolio = Portfolio::new(10_000);
        let strategy = Box::new(BalancedStrategy::default());
        let mgr = PortfolioManager::new(portfolio, strategy, RiskTolerance::Balanced);
        Self {
            portfolio: mgr,
            futures: Vec::new(),
        }
    }
}

/// Build a fresh forge-bank PoolSnapshot from the current ledger state for a given local node.
///
/// Mirrors the existing `pool_handler` in api.rs: same data sources,
/// packaged into the forge-bank type.
pub fn pool_snapshot_from_ledger(
    ledger: &ComputeLedger,
    local_node_id: &NodeId,
) -> forge_bank::PoolSnapshot {
    let status = ledger.lending_pool_status();
    let credit = ledger.compute_credit_score(local_node_id);
    let your_max_borrow = max_borrowable(credit, status.available_cu);
    let your_offered = offered_interest_rate(credit);

    forge_bank::PoolSnapshot::new(
        status.total_pool_cu,
        status.lent_cu,
        status.available_cu,
        status.reserve_ratio,
        status.active_loan_count as u64,
        status.avg_interest_rate,
        your_max_borrow,
        your_offered,
    )
    .expect("ledger state must produce valid PoolSnapshot")
}
