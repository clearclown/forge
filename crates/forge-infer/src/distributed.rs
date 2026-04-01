//! Distributed inference orchestrator.
//!
//! Wraps llama.cpp's `llama-cli` with `--rpc` flag for split inference
//! across multiple machines. Forge provides the P2P discovery and
//! orchestration; llama.cpp handles the tensor computation.
//!
//! ## Architecture
//!
//! ```text
//! forge-infer (this crate)
//!   │
//!   ├── LlamaCppEngine       — local inference via llama-cpp-2 library
//!   └── DistributedEngine    — distributed inference via llama-cli subprocess
//!         │
//!         ├── llama-cli --rpc peer1:port,peer2:port -m model.gguf
//!         └── Peers run rpc-server via RpcServer::spawn()
//! ```

use forge_core::ForgeError;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Configuration for a distributed inference session.
#[derive(Debug, Clone)]
pub struct DistributedConfig {
    /// Path to the GGUF model file.
    pub model_path: PathBuf,
    /// RPC server endpoints (host:port pairs).
    pub rpc_endpoints: Vec<String>,
    /// Number of GPU layers to offload (0 = CPU only).
    pub n_gpu_layers: u32,
    /// Path to llama-cli binary.
    pub llama_cli_path: PathBuf,
}

/// Find the llama-cli binary.
pub fn find_llama_cli() -> Option<PathBuf> {
    // Check env var
    if let Ok(path) = std::env::var("FORGE_LLAMA_CLI_PATH") {
        let p = PathBuf::from(&path);
        if p.exists() {
            return Some(p);
        }
    }

    // Check PATH
    if let Ok(output) = Command::new("which").arg("llama-cli").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(PathBuf::from(path));
            }
        }
    }

    // Common locations
    for candidate in &[
        "/tmp/llama.cpp/build/bin/llama-cli",
        "/usr/local/bin/llama-cli",
        "/opt/homebrew/bin/llama-cli",
        "./llama-cli",
    ] {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return Some(p);
        }
    }

    None
}

/// Run distributed inference using llama-cli with --rpc.
///
/// Returns the generated text and token count.
pub fn run_distributed_inference(
    config: &DistributedConfig,
    prompt: &str,
    max_tokens: u32,
    temperature: f32,
) -> Result<(String, usize), ForgeError> {
    if config.rpc_endpoints.is_empty() {
        return Err(ForgeError::InferenceError(
            "no RPC endpoints configured".to_string(),
        ));
    }

    let rpc_arg = config.rpc_endpoints.join(",");

    tracing::info!(
        "Distributed inference: model={:?}, rpc={}, max_tokens={}, temp={}",
        config.model_path,
        rpc_arg,
        max_tokens,
        temperature
    );

    let mut cmd = Command::new(&config.llama_cli_path);
    cmd.arg("--rpc")
        .arg(&rpc_arg)
        .arg("-m")
        .arg(&config.model_path)
        .arg("-p")
        .arg(prompt)
        .arg("-n")
        .arg(max_tokens.to_string())
        .arg("--temp")
        .arg(format!("{:.2}", temperature))
        .arg("-ngl")
        .arg(config.n_gpu_layers.to_string())
        .arg("--no-display-prompt")
        .arg("--log-disable")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let child = cmd.spawn().map_err(|e| {
        ForgeError::InferenceError(format!("spawn llama-cli: {e}"))
    })?;

    let output = child.wait_with_output().map_err(|e| {
        ForgeError::InferenceError(format!("llama-cli wait: {e}"))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ForgeError::InferenceError(format!(
            "llama-cli failed (exit {}): {}",
            output.status,
            stderr.lines().last().unwrap_or("unknown error")
        )));
    }

    let text = String::from_utf8_lossy(&output.stdout).to_string();
    let text = text.trim().to_string();

    // Estimate token count from output length (rough approximation)
    let token_count = text.split_whitespace().count().max(1);

    Ok((text, token_count))
}

/// Check if distributed inference is available (llama-cli + rpc-server binaries exist).
pub fn is_distributed_available() -> bool {
    find_llama_cli().is_some() && super::rpc_manager::is_rpc_available()
}

/// Get a status summary of distributed inference capabilities.
pub fn distributed_status() -> DistributedStatus {
    DistributedStatus {
        llama_cli_available: find_llama_cli().is_some(),
        llama_cli_path: find_llama_cli(),
        rpc_server_available: super::rpc_manager::is_rpc_available(),
        rpc_server_path: super::rpc_manager::RpcServer::find_binary(),
    }
}

#[derive(Debug, Clone)]
pub struct DistributedStatus {
    pub llama_cli_available: bool,
    pub llama_cli_path: Option<PathBuf>,
    pub rpc_server_available: bool,
    pub rpc_server_path: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_llama_cli_does_not_panic() {
        let _ = find_llama_cli();
    }

    #[test]
    fn distributed_status_reports_availability() {
        let status = distributed_status();
        println!("llama-cli: {:?}", status.llama_cli_path);
        println!("rpc-server: {:?}", status.rpc_server_path);
    }
}
