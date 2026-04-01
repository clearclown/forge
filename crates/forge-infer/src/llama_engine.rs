use crate::engine::InferenceEngine;
use crate::token_stream::TokenOutputStream;
use forge_core::{ForgeError, LayerRange};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::token::data_array::LlamaTokenDataArray;
use std::num::NonZeroU32;
use std::path::Path;
use tokenizers::Tokenizer;

/// llama.cpp-based inference engine for GGUF quantized models.
///
/// Uses the llama-cpp-2 crate for inference and a HuggingFace tokenizer
/// for streaming text decode (compatible with the forge protocol).
pub struct LlamaCppEngine {
    backend: Option<LlamaBackend>,
    model: Option<LlamaModel>,
    tokenizer: Option<Tokenizer>,
    eos_token_id: i32,
}

// LlamaModel is not Send. We guarantee exclusive access via Arc<Mutex<>>.
unsafe impl Send for LlamaCppEngine {}
unsafe impl Sync for LlamaCppEngine {}

impl LlamaCppEngine {
    pub fn new() -> Self {
        tracing::info!("Inference engine: llama.cpp");
        Self {
            backend: None,
            model: None,
            tokenizer: None,
            eos_token_id: -1,
        }
    }
}

impl Default for LlamaCppEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl InferenceEngine for LlamaCppEngine {
    fn load(
        &mut self,
        model_path: &Path,
        tokenizer_path: &Path,
        _layer_range: Option<LayerRange>,
    ) -> Result<(), ForgeError> {
        tracing::info!("Loading GGUF model from {:?}", model_path);

        let backend = LlamaBackend::init()
            .map_err(|e| ForgeError::ModelLoadError(format!("backend init: {e}")))?;

        let model_params = LlamaModelParams::default();

        let model = LlamaModel::load_from_file(&backend, model_path, &model_params)
            .map_err(|e| ForgeError::ModelLoadError(format!("load model: {e}")))?;

        let eos = model.token_eos().0;
        tracing::info!("Model loaded (llama.cpp), EOS token={}", eos);

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| ForgeError::ModelLoadError(format!("load tokenizer: {e}")))?;
        tracing::info!("HF tokenizer loaded");

        self.backend = Some(backend);
        self.model = Some(model);
        self.tokenizer = Some(tokenizer);
        self.eos_token_id = eos;

        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.model.is_some() && self.tokenizer.is_some()
    }

    fn generate(
        &mut self,
        prompt: &str,
        max_tokens: u32,
        temperature: f32,
        _top_p: Option<f64>,
    ) -> Result<Vec<String>, ForgeError> {
        let backend = self.backend.as_ref()
            .ok_or_else(|| ForgeError::InferenceError("backend not initialized".to_string()))?;
        let model = self.model.as_ref()
            .ok_or_else(|| ForgeError::InferenceError("model not loaded".to_string()))?;
        let tokenizer = self.tokenizer.as_ref()
            .ok_or_else(|| ForgeError::InferenceError("tokenizer not loaded".to_string()))?;

        // Tokenize using llama.cpp's tokenizer
        let tokens = model
            .str_to_token(prompt, AddBos::Always)
            .map_err(|e| ForgeError::InferenceError(format!("tokenize: {e}")))?;

        if tokens.is_empty() {
            return Err(ForgeError::InferenceError("empty prompt".to_string()));
        }

        tracing::debug!("Prompt: {} tokens", tokens.len());

        // Create context for this generation
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(4096));

        let mut ctx = model
            .new_context(backend, ctx_params)
            .map_err(|e| ForgeError::InferenceError(format!("context: {e}")))?;

        // Evaluate prompt
        let mut batch = LlamaBatch::new(4096, 1);
        let last_idx = tokens.len() as i32 - 1;
        for (i, &token) in tokens.iter().enumerate() {
            batch
                .add(token, i as i32, &[0], i as i32 == last_idx)
                .map_err(|e| ForgeError::InferenceError(format!("batch add: {e}")))?;
        }

        ctx.decode(&mut batch)
            .map_err(|e| ForgeError::InferenceError(format!("decode prompt: {e}")))?;

        // Generation loop
        let mut token_stream = TokenOutputStream::new(tokenizer.clone());
        let mut generated: Vec<String> = Vec::new();
        let mut n_decoded = tokens.len();

        for _ in 0..max_tokens {
            let candidates = ctx.candidates_ith(batch.n_tokens() - 1);
            let mut candidates_p = LlamaTokenDataArray::from_iter(candidates, false);

            let new_token = if temperature <= 0.0 {
                candidates_p.sample_token_greedy()
            } else {
                candidates_p.sample_token(42) // seed=42
            };

            if new_token.0 == self.eos_token_id {
                break;
            }

            // Decode via HF tokenizer for streaming text
            if let Ok(Some(text)) = token_stream.next_token(new_token.0 as u32) {
                generated.push(text);
            }

            // Next token
            batch.clear();
            batch
                .add(new_token, n_decoded as i32, &[0], true)
                .map_err(|e| ForgeError::InferenceError(format!("batch add: {e}")))?;

            ctx.decode(&mut batch)
                .map_err(|e| ForgeError::InferenceError(format!("decode: {e}")))?;

            n_decoded += 1;
        }

        if let Ok(Some(text)) = token_stream.flush() {
            generated.push(text);
        }

        Ok(generated)
    }

    fn tokenize(&self, prompt: &str) -> Result<Vec<u32>, ForgeError> {
        let tokenizer = self.tokenizer.as_ref()
            .ok_or_else(|| ForgeError::InferenceError("tokenizer not loaded".to_string()))?;
        let encoding = tokenizer.encode(prompt, true)
            .map_err(|e| ForgeError::InferenceError(format!("tokenize: {e}")))?;
        Ok(encoding.get_ids().to_vec())
    }

    fn decode(&self, tokens: &[u32]) -> Result<String, ForgeError> {
        let tokenizer = self.tokenizer.as_ref()
            .ok_or_else(|| ForgeError::InferenceError("tokenizer not loaded".to_string()))?;
        tokenizer.decode(tokens, true)
            .map_err(|e| ForgeError::InferenceError(format!("decode: {e}")))
    }

    fn forward_tokens(
        &mut self,
        _tokens: &[u32],
        _pos: usize,
    ) -> Result<Vec<f32>, ForgeError> {
        Err(ForgeError::InferenceError(
            "forward_tokens not yet implemented for llama.cpp backend".to_string(),
        ))
    }

    fn sample_token(
        &mut self,
        _logits: &[f32],
        _temperature: f32,
        _top_p: Option<f64>,
    ) -> Result<u32, ForgeError> {
        Err(ForgeError::InferenceError(
            "sample_token not yet implemented for llama.cpp backend".to_string(),
        ))
    }
}
