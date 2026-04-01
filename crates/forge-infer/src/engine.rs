use forge_core::LayerRange;
use std::path::Path;

/// The inference engine trait. Implemented by Candle backend.
pub trait InferenceEngine: Send + Sync {
    /// Load a GGUF model (or a slice of layers for distributed inference).
    fn load(
        &mut self,
        model_path: &Path,
        tokenizer_path: &Path,
        layer_range: Option<LayerRange>,
    ) -> Result<(), forge_core::ForgeError>;

    /// Check if a model is loaded and ready.
    fn is_loaded(&self) -> bool;

    /// Generate tokens from a prompt string.
    /// Returns a vector of generated text fragments.
    fn generate(
        &mut self,
        prompt: &str,
        max_tokens: u32,
        temperature: f32,
        top_p: Option<f64>,
    ) -> Result<Vec<String>, forge_core::ForgeError>;

    /// Tokenize a prompt and return token IDs.
    fn tokenize(&self, prompt: &str) -> Result<Vec<u32>, forge_core::ForgeError>;

    /// Decode token IDs back to text.
    fn decode(&self, tokens: &[u32]) -> Result<String, forge_core::ForgeError>;

    /// Run a forward pass on token IDs and return raw logits for the last position.
    /// This is used for split-inference coordination — the seed runs the full model
    /// but exposes the forward pass result for activation routing.
    fn forward_tokens(
        &mut self,
        tokens: &[u32],
        pos: usize,
    ) -> Result<Vec<f32>, forge_core::ForgeError>;

    /// Sample the next token from logits.
    fn sample_token(
        &mut self,
        logits: &[f32],
        temperature: f32,
        top_p: Option<f64>,
    ) -> Result<u32, forge_core::ForgeError>;
}
