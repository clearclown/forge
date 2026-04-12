//! forge-sdk — Rust HTTP client for the Forge compute economy.
//!
//! Replaces the Python `forge_sdk` package. 100% Rust, async, type-safe.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use forge_sdk::ForgeClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), forge_sdk::SdkError> {
//!     let client = ForgeClient::new("http://127.0.0.1:3000", Some("my-token"));
//!     let balance = client.balance().await?;
//!     println!("Balance: {} CU", balance.effective_balance);
//!     Ok(())
//! }
//! ```
//!
//! # Autonomous agent
//!
//! ```rust,no_run
//! use forge_sdk::ForgeAgent;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut agent = ForgeAgent::new("http://127.0.0.1:3000", None, 500, 100);
//!     while agent.has_budget().await {
//!         match agent.think("What is 2+2?", 64).await {
//!             Ok(Some(resp)) => println!("{:?}", resp.choices),
//!             _ => break,
//!         }
//!     }
//! }
//! ```

mod client;
mod error;
mod types;

pub use client::{ForgeAgent, ForgeClient};
pub use error::SdkError;
pub use types::*;
