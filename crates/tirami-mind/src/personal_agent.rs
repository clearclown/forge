//! Phase 18.5 — `PersonalAgent`.
//!
//! The user-facing AI agent that lives on the user's device,
//! manages its own wallet, and autonomously buys + sells compute
//! on the Tirami mesh. The user never manages TRM directly.
//!
//! # Relation to `TiramiMindAgent`
//!
//! `TiramiMindAgent` (Phase 7+) is a *self-improvement* harness:
//! it evolves a prompt-generation policy over many cycles,
//! optionally paying a frontier API via `TrmPaidOptimizer`.
//! That's an internal mechanism.
//!
//! `PersonalAgent` is the USER-facing wrapper. Its job is:
//! 1. Hold the user's Tirami identity + wallet.
//! 2. Serve inference to the mesh when the user's machine is
//!    idle (earn TRM passively).
//! 3. Spend TRM to rent compute from the mesh when the user
//!    asks for a task the local hardware cannot handle.
//! 4. Report to the user in natural language.
//!
//! `PersonalAgent` can *contain* a `TiramiMindAgent` for
//! self-improvement but it is not required.
//!
//! # Autonomy contract
//!
//! Everything the agent CAN do autonomously is listed in
//! [`AgentPreferences`]. Anything outside those bounds MUST
//! surface a [`AgentDecision::AskUser`] event.

use serde::{Deserialize, Serialize};
use tirami_core::NodeId;

use crate::budget::TrmBudget;

// ---------------------------------------------------------------------------
// Preferences
// ---------------------------------------------------------------------------

/// User-tunable guardrails on the personal agent. Every field has
/// a sensible default that matches "I bought a Mac and ran
/// `tirami start`" with no configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPreferences {
    /// Daily spending ceiling in TRM. Default 20. Agent asks
    /// before exceeding.
    pub daily_spend_limit_trm: u64,
    /// Per-task spending ceiling in TRM. Default 15. Agent asks
    /// before any single task above this.
    pub per_task_budget_trm: u64,
    /// Enable auto-earn (serve inference when idle). Default true.
    pub auto_earn_enabled: bool,
    /// Enable auto-spend (rent compute when local capacity is
    /// insufficient). Default true.
    pub auto_spend_enabled: bool,
    /// Fraction (0.0-1.0) of the current balance the agent is
    /// allowed to stake autonomously. Default 0.90 — keeps 10 %
    /// liquid for immediate spending.
    pub auto_stake_fraction: f64,
    /// CPU / GPU utilization threshold below which the machine
    /// counts as "idle" and auto-earn can trigger. Default 0.20.
    pub idle_utilization_threshold: f64,
    /// Seconds of continuous idle-state required before auto-earn
    /// starts a serving session. Default 60.
    pub idle_grace_seconds: u64,
    /// Agent serves requests from peers whose reputation is at
    /// least this. Default 0.3 (accepts most peers, rejects freshly
    /// slashed ones). 0.0 = serve anyone; 1.0 = only perfect rep.
    pub min_peer_reputation: f64,
    /// Content filter level applied to prompts the agent serves.
    /// Values: "none" (serve anything), "default" (block
    /// obvious-abuse patterns), "strict" (conservative).
    pub content_filter: String,
}

impl Default for AgentPreferences {
    fn default() -> Self {
        Self {
            daily_spend_limit_trm: 20,
            per_task_budget_trm: 15,
            auto_earn_enabled: true,
            auto_spend_enabled: true,
            auto_stake_fraction: 0.90,
            idle_utilization_threshold: 0.20,
            idle_grace_seconds: 60,
            min_peer_reputation: 0.3,
            content_filter: "default".to_string(),
        }
    }
}

impl AgentPreferences {
    /// Sanity-check the preferences. Returns `Err(message)` if any
    /// field is out of range. The default is always valid.
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.auto_stake_fraction) {
            return Err(format!(
                "auto_stake_fraction must be in [0.0, 1.0], got {}",
                self.auto_stake_fraction
            ));
        }
        if !(0.0..=1.0).contains(&self.idle_utilization_threshold) {
            return Err(format!(
                "idle_utilization_threshold must be in [0.0, 1.0], got {}",
                self.idle_utilization_threshold
            ));
        }
        if !(0.0..=1.0).contains(&self.min_peer_reputation) {
            return Err(format!(
                "min_peer_reputation must be in [0.0, 1.0], got {}",
                self.min_peer_reputation
            ));
        }
        if self.per_task_budget_trm > self.daily_spend_limit_trm {
            return Err(format!(
                "per_task_budget_trm ({}) exceeds daily_spend_limit_trm ({})",
                self.per_task_budget_trm, self.daily_spend_limit_trm
            ));
        }
        match self.content_filter.as_str() {
            "none" | "default" | "strict" => Ok(()),
            other => Err(format!(
                "content_filter must be one of: none | default | strict. Got: {other}"
            )),
        }
    }
}

// ---------------------------------------------------------------------------
// Task intent + cost estimate
// ---------------------------------------------------------------------------

/// Size classification for a task the user asked the agent to
/// run. Determines local-vs-remote routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskSize {
    /// Fits comfortably on the local machine (e.g. summarizing
    /// a 1-page document with a 0.5B model).
    Local,
    /// Exceeds local capacity — needs a remote provider
    /// (e.g. a 7B+ model, multi-turn reasoning).
    Remote,
    /// Best-effort local; spill to remote only if latency budget
    /// is exceeded.
    Hybrid,
}

/// Estimated cost of a task. Returned by the agent's pre-flight
/// check so it knows whether to trigger an `AskUser` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCostEstimate {
    pub size: TaskSize,
    pub estimated_trm: u64,
    pub estimated_seconds: u64,
    pub preferred_provider: Option<NodeId>,
}

// ---------------------------------------------------------------------------
// Decision envelope
// ---------------------------------------------------------------------------

/// The outcome of the agent's decision for a single task. Either
/// the agent resolved it autonomously (allowed by the
/// preferences) or the decision bubbled up to the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentDecision {
    /// Agent ran the task locally within preferences.
    RanLocally {
        task_id: String,
        output_summary: String,
        tokens_processed: u64,
    },
    /// Agent paid the network for compute and got a result.
    RanRemote {
        task_id: String,
        output_summary: String,
        trm_spent: u64,
        provider: NodeId,
    },
    /// Agent earned TRM by serving a peer's request.
    ServedRequest {
        task_id: String,
        peer: NodeId,
        trm_earned: u64,
    },
    /// Agent needs user approval before proceeding.
    AskUser {
        task_id: String,
        reason: String,
        cost_estimate: TaskCostEstimate,
    },
    /// Agent refused a task (budget, content filter, preferences).
    Refused { task_id: String, reason: String },
}

// ---------------------------------------------------------------------------
// PersonalAgent
// ---------------------------------------------------------------------------

/// The user's agent. Owns a wallet (via `NodeId` pointing at the
/// user's on-disk identity), a budget, preferences, and a running
/// tally of today's activity.
///
/// Network I/O is NOT in this struct — the agent calls out to
/// `tirami-node` handlers via the `AgentEnv` trait. This keeps
/// the agent logic unit-testable without spinning up a full node.
#[derive(Debug, Clone)]
pub struct PersonalAgent {
    /// The Tirami node id this agent uses as its wallet.
    pub wallet: NodeId,
    /// Spend budget for the current session.
    pub budget: TrmBudget,
    /// User-tunable preferences.
    pub preferences: AgentPreferences,
    /// Running count of TRM spent today (resets at UTC midnight
    /// or on `reset_daily_tally`).
    pub spent_today_trm: u64,
    /// Running count of TRM earned today.
    pub earned_today_trm: u64,
    /// Timestamp (unix-ms) when the daily tally was last reset.
    pub day_started_at_ms: u64,
}

impl PersonalAgent {
    /// Construct a fresh agent with default preferences.
    pub fn new(wallet: NodeId, budget: TrmBudget, now_ms: u64) -> Self {
        Self {
            wallet,
            budget,
            preferences: AgentPreferences::default(),
            spent_today_trm: 0,
            earned_today_trm: 0,
            day_started_at_ms: now_ms,
        }
    }

    /// Construct with explicit preferences. Validates them first.
    pub fn with_preferences(
        wallet: NodeId,
        budget: TrmBudget,
        preferences: AgentPreferences,
        now_ms: u64,
    ) -> Result<Self, String> {
        preferences.validate()?;
        Ok(Self {
            wallet,
            budget,
            preferences,
            spent_today_trm: 0,
            earned_today_trm: 0,
            day_started_at_ms: now_ms,
        })
    }

    /// Update preferences. Returns an error (and leaves the
    /// current preferences intact) if validation fails.
    pub fn set_preferences(&mut self, new_prefs: AgentPreferences) -> Result<(), String> {
        new_prefs.validate()?;
        self.preferences = new_prefs;
        Ok(())
    }

    /// Reset the daily tally. Normally called when
    /// `now_ms - day_started_at_ms >= 24h`.
    pub fn reset_daily_tally(&mut self, now_ms: u64) {
        self.spent_today_trm = 0;
        self.earned_today_trm = 0;
        self.day_started_at_ms = now_ms;
    }

    /// Can the agent autonomously spend `trm` on a single task?
    /// Factors in the per-task cap AND the daily cumulative cap.
    pub fn can_auto_spend(&self, trm: u64) -> bool {
        if trm > self.preferences.per_task_budget_trm {
            return false;
        }
        let new_total = self.spent_today_trm.saturating_add(trm);
        new_total <= self.preferences.daily_spend_limit_trm
    }

    /// Record a spend against the daily tally. Caller must have
    /// already verified `can_auto_spend(trm)` (or handled the
    /// AskUser flow).
    pub fn record_spend(&mut self, trm: u64) {
        self.spent_today_trm = self.spent_today_trm.saturating_add(trm);
    }

    /// Record an earn against the daily tally.
    pub fn record_earn(&mut self, trm: u64) {
        self.earned_today_trm = self.earned_today_trm.saturating_add(trm);
    }

    /// Net daily flow (earn - spend). Negative means the agent is
    /// net spending today; positive means net earning.
    pub fn net_today_trm(&self) -> i64 {
        self.earned_today_trm as i64 - self.spent_today_trm as i64
    }

    /// Should the agent trigger an `AskUser` flow for a task
    /// estimated at `estimate`? Returns a reason string if yes,
    /// `None` if the agent can proceed autonomously.
    pub fn needs_user_approval(&self, estimate: &TaskCostEstimate) -> Option<String> {
        if !self.preferences.auto_spend_enabled && estimate.size != TaskSize::Local {
            return Some("auto_spend is disabled; user must approve remote tasks".into());
        }
        if estimate.estimated_trm > self.preferences.per_task_budget_trm {
            return Some(format!(
                "task cost {} TRM exceeds per-task budget {} TRM",
                estimate.estimated_trm, self.preferences.per_task_budget_trm
            ));
        }
        let new_total = self.spent_today_trm.saturating_add(estimate.estimated_trm);
        if new_total > self.preferences.daily_spend_limit_trm {
            return Some(format!(
                "task would push today's spending to {} TRM, exceeding daily limit {} TRM",
                new_total, self.preferences.daily_spend_limit_trm
            ));
        }
        None
    }

    /// Human-readable status line for `tirami agent status`.
    pub fn status_summary(&self) -> String {
        format!(
            "wallet {} · spent today {} TRM · earned today {} TRM · net {:+} TRM · auto-earn {} · auto-spend {}",
            self.wallet.to_hex(),
            self.spent_today_trm,
            self.earned_today_trm,
            self.net_today_trm(),
            if self.preferences.auto_earn_enabled { "on" } else { "off" },
            if self.preferences.auto_spend_enabled { "on" } else { "off" },
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn wallet() -> NodeId {
        NodeId([0xABu8; 32])
    }

    fn budget() -> TrmBudget {
        TrmBudget::default()
    }

    #[test]
    fn default_preferences_are_valid() {
        let p = AgentPreferences::default();
        assert!(p.validate().is_ok());
    }

    #[test]
    fn preferences_reject_out_of_range_stake_fraction() {
        let mut p = AgentPreferences::default();
        p.auto_stake_fraction = 1.5;
        assert!(p.validate().is_err());
        p.auto_stake_fraction = -0.1;
        assert!(p.validate().is_err());
    }

    #[test]
    fn preferences_reject_per_task_over_daily_limit() {
        let mut p = AgentPreferences::default();
        p.per_task_budget_trm = 100;
        p.daily_spend_limit_trm = 20;
        let err = p.validate().unwrap_err();
        assert!(err.contains("per_task_budget_trm"));
    }

    #[test]
    fn preferences_reject_unknown_content_filter() {
        let mut p = AgentPreferences::default();
        p.content_filter = "whatever".into();
        let err = p.validate().unwrap_err();
        assert!(err.contains("content_filter"));
    }

    #[test]
    fn new_agent_starts_with_zero_tally() {
        let a = PersonalAgent::new(wallet(), budget(), 1_000);
        assert_eq!(a.spent_today_trm, 0);
        assert_eq!(a.earned_today_trm, 0);
        assert_eq!(a.net_today_trm(), 0);
    }

    #[test]
    fn with_preferences_validates() {
        let mut bad = AgentPreferences::default();
        bad.content_filter = "bogus".into();
        let r = PersonalAgent::with_preferences(wallet(), budget(), bad, 0);
        assert!(r.is_err());
    }

    #[test]
    fn can_auto_spend_enforces_per_task_cap() {
        let mut a = PersonalAgent::new(wallet(), budget(), 0);
        // 15 TRM / task default
        assert!(a.can_auto_spend(10));
        assert!(a.can_auto_spend(15));
        assert!(!a.can_auto_spend(16));
        // Also false when daily would blow past 20.
        a.record_spend(10);
        assert!(a.can_auto_spend(10)); // 10 + 10 = 20, allowed
        assert!(!a.can_auto_spend(11)); // 10 + 11 = 21, denied
    }

    #[test]
    fn record_spend_and_earn_update_net() {
        let mut a = PersonalAgent::new(wallet(), budget(), 0);
        a.record_earn(8);
        a.record_spend(3);
        assert_eq!(a.net_today_trm(), 5);
        a.record_spend(10);
        assert_eq!(a.net_today_trm(), -5);
    }

    #[test]
    fn reset_daily_tally_clears_counters() {
        let mut a = PersonalAgent::new(wallet(), budget(), 0);
        a.record_spend(10);
        a.record_earn(5);
        a.reset_daily_tally(86_400_000);
        assert_eq!(a.spent_today_trm, 0);
        assert_eq!(a.earned_today_trm, 0);
        assert_eq!(a.day_started_at_ms, 86_400_000);
    }

    #[test]
    fn needs_user_approval_on_per_task_over_budget() {
        let a = PersonalAgent::new(wallet(), budget(), 0);
        let estimate = TaskCostEstimate {
            size: TaskSize::Remote,
            estimated_trm: 50,
            estimated_seconds: 60,
            preferred_provider: None,
        };
        let reason = a.needs_user_approval(&estimate).unwrap();
        assert!(reason.contains("per-task budget"));
    }

    #[test]
    fn needs_user_approval_on_daily_cumulative() {
        let mut a = PersonalAgent::new(wallet(), budget(), 0);
        a.record_spend(15);
        let estimate = TaskCostEstimate {
            size: TaskSize::Remote,
            estimated_trm: 10, // under per-task, over daily (15+10 > 20)
            estimated_seconds: 30,
            preferred_provider: None,
        };
        let reason = a.needs_user_approval(&estimate).unwrap();
        assert!(reason.contains("daily limit"));
    }

    #[test]
    fn needs_user_approval_when_auto_spend_disabled_and_remote() {
        let mut a = PersonalAgent::new(wallet(), budget(), 0);
        a.preferences.auto_spend_enabled = false;
        let estimate = TaskCostEstimate {
            size: TaskSize::Remote,
            estimated_trm: 5,
            estimated_seconds: 10,
            preferred_provider: None,
        };
        assert!(a.needs_user_approval(&estimate).is_some());
    }

    #[test]
    fn needs_user_approval_none_when_local_even_if_spend_disabled() {
        let mut a = PersonalAgent::new(wallet(), budget(), 0);
        a.preferences.auto_spend_enabled = false;
        let estimate = TaskCostEstimate {
            size: TaskSize::Local,
            estimated_trm: 1,
            estimated_seconds: 2,
            preferred_provider: None,
        };
        // Local task with tiny cost — no approval needed.
        assert!(a.needs_user_approval(&estimate).is_none());
    }

    #[test]
    fn set_preferences_rejects_invalid() {
        let mut a = PersonalAgent::new(wallet(), budget(), 0);
        let mut bad = AgentPreferences::default();
        bad.min_peer_reputation = 2.0;
        let err = a.set_preferences(bad).unwrap_err();
        assert!(err.contains("min_peer_reputation"));
        // Current preferences unchanged.
        assert_eq!(a.preferences.min_peer_reputation, 0.3);
    }

    #[test]
    fn status_summary_contains_expected_fields() {
        let mut a = PersonalAgent::new(wallet(), budget(), 0);
        a.record_spend(3);
        a.record_earn(8);
        let s = a.status_summary();
        assert!(s.contains("wallet"));
        assert!(s.contains("spent today 3 TRM"));
        assert!(s.contains("earned today 8 TRM"));
        assert!(s.contains("net +5 TRM"));
        assert!(s.contains("auto-earn on"));
    }

    #[test]
    fn agent_decision_serde_roundtrips() {
        let d = AgentDecision::RanLocally {
            task_id: "t1".into(),
            output_summary: "done".into(),
            tokens_processed: 42,
        };
        let s = serde_json::to_string(&d).unwrap();
        let back: AgentDecision = serde_json::from_str(&s).unwrap();
        match back {
            AgentDecision::RanLocally {
                task_id, tokens_processed, ..
            } => {
                assert_eq!(task_id, "t1");
                assert_eq!(tokens_processed, 42);
            }
            _ => panic!("wrong variant"),
        }
    }
}
