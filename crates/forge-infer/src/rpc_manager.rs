//! RPC server subprocess manager for distributed inference.
//!
//! Manages llama.cpp `rpc-server` processes on peer nodes.
//! Forge provides the P2P discovery and encrypted transport layer;
//! llama.cpp's RPC protocol handles the actual tensor operations.

use forge_core::ForgeError;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

/// Manages a local llama.cpp rpc-server subprocess.
pub struct RpcServer {
    child: Option<Child>,
    port: u16,
    host: String,
}

impl RpcServer {
    /// Find the rpc-server binary. Checks:
    /// 1. FORGE_RPC_SERVER_PATH env var
    /// 2. `rpc-server` on PATH
    /// 3. Common llama.cpp build locations
    pub fn find_binary() -> Option<PathBuf> {
        // Check env var first
        if let Ok(path) = std::env::var("FORGE_RPC_SERVER_PATH") {
            let p = PathBuf::from(&path);
            if p.exists() {
                return Some(p);
            }
        }

        // Check PATH
        if let Ok(output) = Command::new("which").arg("rpc-server").output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(PathBuf::from(path));
                }
            }
        }

        // Common locations
        for candidate in &[
            "/usr/local/bin/rpc-server",
            "/opt/homebrew/bin/rpc-server",
            "./rpc-server",
            "./build/bin/rpc-server",
        ] {
            let p = PathBuf::from(candidate);
            if p.exists() {
                return Some(p);
            }
        }

        None
    }

    /// Spawn a local rpc-server on the given port.
    pub fn spawn(port: u16) -> Result<Self, ForgeError> {
        let binary = Self::find_binary().ok_or_else(|| {
            ForgeError::InferenceError(
                "rpc-server binary not found. Set FORGE_RPC_SERVER_PATH or install llama.cpp"
                    .to_string(),
            )
        })?;

        tracing::info!("Starting rpc-server on port {} ({:?})", port, binary);

        let child = Command::new(&binary)
            .arg("-p")
            .arg(port.to_string())
            .spawn()
            .map_err(|e| {
                ForgeError::InferenceError(format!("spawn rpc-server: {e}"))
            })?;

        let mut server = Self {
            child: Some(child),
            port,
            host: "127.0.0.1".to_string(),
        };

        // Wait for the server to be ready
        server.wait_ready(Duration::from_secs(10))?;

        tracing::info!("rpc-server ready on {}:{}", server.host, server.port);

        Ok(server)
    }

    /// Wait until the RPC server is accepting TCP connections.
    fn wait_ready(&self, timeout: Duration) -> Result<(), ForgeError> {
        let start = Instant::now();
        let addr = format!("{}:{}", self.host, self.port);

        loop {
            if TcpStream::connect(&addr).is_ok() {
                return Ok(());
            }

            if start.elapsed() > timeout {
                return Err(ForgeError::InferenceError(format!(
                    "rpc-server failed to start within {}s on {}",
                    timeout.as_secs(),
                    addr
                )));
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// Get the endpoint address string (host:port) for llama.cpp --rpc flag.
    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Get the port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Check if the server is still running.
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.child {
            match child.try_wait() {
                Ok(None) => true,  // still running
                Ok(Some(_)) => false,  // exited
                Err(_) => false,
            }
        } else {
            false
        }
    }

    /// Stop the server.
    pub fn stop(&mut self) {
        if let Some(ref mut child) = self.child {
            let _ = child.kill();
            let _ = child.wait();
            tracing::info!("rpc-server on port {} stopped", self.port);
        }
        self.child = None;
    }
}

impl Drop for RpcServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Check if rpc-server is available on this system.
pub fn is_rpc_available() -> bool {
    RpcServer::find_binary().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_binary_returns_none_if_not_installed() {
        // This test verifies the function doesn't panic.
        // It may return Some or None depending on the system.
        let _ = RpcServer::find_binary();
    }
}
