//! Adapter from forge-ledger trade log to forge-agora marketplace observations.

use forge_agora::{Marketplace, ModelTier, TradeObservation};
use forge_ledger::{ComputeLedger, TradeRecord};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Convert a single TradeRecord to a TradeObservation.
/// Tier is inferred from the model_id; defaults to Small if unknown.
pub fn observation_from_trade(trade: &TradeRecord) -> Option<TradeObservation> {
    let tier = infer_tier(&trade.model_id);
    let provider_hex = hex::encode(trade.provider.0);
    let consumer_hex = hex::encode(trade.consumer.0);

    // TradeObservation requires provider != consumer and both must be 64-char hex
    if provider_hex == consumer_hex {
        return None;
    }

    let trade_id = format!("{:x}-{:x}", trade.timestamp, trade.cu_amount);

    TradeObservation::new(
        trade_id,
        provider_hex,
        consumer_hex,
        trade.cu_amount,
        trade.tokens_processed,
        trade.model_id.clone(),
        tier,
        trade.timestamp,
    )
    .ok()
}

fn infer_tier(model_id: &str) -> ModelTier {
    let s = model_id.to_lowercase();
    if s.contains("frontier")
        || s.contains("opus")
        || s.contains("gpt-4")
        || s.contains("claude-3")
        || s.contains("claude-opus")
    {
        ModelTier::Frontier
    } else if s.contains("70b") || s.contains("large") {
        ModelTier::Large
    } else if s.contains("13b") || s.contains("medium") || s.contains("8b") {
        ModelTier::Medium
    } else {
        ModelTier::Small
    }
}

/// Lazily refresh marketplace from the ledger trade log.
/// Drains trades after `last_seen_idx` and feeds them to marketplace.observe_trade.
/// Updates `last_seen_idx` to the new tail.
pub async fn refresh_marketplace_from_ledger(
    ledger: &Arc<Mutex<ComputeLedger>>,
    marketplace: &Arc<Mutex<Marketplace>>,
    last_seen_idx: &Arc<Mutex<usize>>,
) {
    let trades_to_observe: Vec<TradeRecord> = {
        let l = ledger.lock().await;
        // recent_trades(usize::MAX) returns all trades newest-first; we need all for slicing
        let all_trades = l.recent_trades(usize::MAX);
        let total = all_trades.len();
        let mut idx = last_seen_idx.lock().await;
        if *idx >= total {
            return;
        }
        // all_trades is newest-first, so to get trades after last_seen_idx we need
        // the first (total - *idx) entries reversed (oldest to newest among new ones)
        let new_count = total - *idx;
        let new_trades: Vec<TradeRecord> = all_trades[..new_count].iter().rev().cloned().collect();
        *idx = total;
        new_trades
    };

    if !trades_to_observe.is_empty() {
        let mut mp = marketplace.lock().await;
        for trade in &trades_to_observe {
            if let Some(obs) = observation_from_trade(trade) {
                mp.observe_trade(obs);
            }
        }
    }
}
