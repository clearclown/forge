//! Phase 17 Wave 1.6 — hybrid post-quantum signatures (scaffold).
//!
//! # What this module is
//!
//! A scaffold for a "Ed25519 + ML-DSA" hybrid signature scheme. The goal:
//! once CRQC (cryptographically-relevant quantum computers) can break
//! Ed25519 (mid-2030s per NIST modeling), Tirami's economic record must
//! already have an intact PQ signature on every trade. Ratcheting after
//! a break is too late — the trade log is append-only and signatures
//! retroactively fail.
//!
//! # Design
//!
//! [`HybridSignature`] = `(ed25519_sig: [u8; 64], pq_sig: Option<Vec<u8>>)`.
//! The PQ half is optional so that pre-Phase-17 peers keep working
//! (their signatures will just have `pq_sig = None`), but the verify
//! path is both-or-fail whenever `pq_sig` is present. A signer cannot
//! produce a valid hybrid where the PQ half alone is bogus without
//! triggering a verify failure.
//!
//! [`HybridKey`] pairs an `ed25519_dalek::SigningKey` with an opaque
//! PQ key handle. The PQ half is produced by a [`PqSigner`] trait
//! implementation — [`MockPqSigner`] is used for tests; a future
//! `MlDsaSigner` will wrap the `ml-dsa` crate once its digest/sha3
//! version pins stabilize (it currently conflicts with iroh 0.97).
//!
//! # Wire format
//!
//! Canonical serialization: the caller computes a deterministic
//! `canonical_bytes()` over the payload, passes it to [`HybridKey::sign`],
//! and transmits the resulting `HybridSignature`. Verification
//! recomputes the same canonical bytes and calls
//! [`HybridSignature::verify`]. The hybrid shape is serde-friendly so
//! it can ride on top of the existing `SignedTradeRecord` / `SignedLoanRecord`
//! flow with zero new wire messages — the plumbing change is purely
//! the sig type.
//!
//! # Why scaffold-only right now
//!
//! The production dependency pull is blocked on a downstream version
//! conflict (`digest 0.11` vs iroh's `digest 0.11.0-rc.10`). Rather
//! than wait, Wave 1.6 ships the full type lattice + tests with a
//! deterministic mock verifier, so when ml-dsa stabilizes the swap is
//! one file, no API change to callers.

use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ---------------------------------------------------------------------------
// PQ trait surface
// ---------------------------------------------------------------------------

/// Pluggable post-quantum signer. Implementors wrap a specific PQ
/// scheme (ML-DSA, Falcon, etc.) and expose the four basic operations
/// used by [`HybridKey`].
pub trait PqSigner {
    /// Produce a PQ signature over `msg`. The returned bytes form the
    /// `pq_sig` field of [`HybridSignature`]. Must be deterministic OR
    /// carry its own entropy internally.
    fn sign_pq(&self, msg: &[u8]) -> Vec<u8>;

    /// Public key bytes for this signer. Used for verify.
    fn pq_verifying_key(&self) -> Vec<u8>;
}

/// Verification side of a [`PqSigner`]. Given the PQ public key, the
/// message, and the claimed signature, return `Ok(())` on success.
/// Implementations SHOULD be constant-time where the underlying
/// library allows.
pub trait PqVerifier {
    fn verify_pq(&self, pq_vk: &[u8], msg: &[u8], pq_sig: &[u8]) -> Result<(), PqError>;
}

/// Error codes from PQ verify. Kept small and `Copy` so they can be
/// returned inside `Result` hot paths without extra allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PqError {
    /// Key bytes were the wrong length or otherwise rejected by the
    /// underlying scheme.
    InvalidKey,
    /// Signature failed mathematical verification.
    InvalidSignature,
    /// Scheme-specific rejection (malformed, wrong algorithm id, …).
    Rejected,
}

// ---------------------------------------------------------------------------
// HybridSignature — on-the-wire format
// ---------------------------------------------------------------------------

/// Ed25519 + optional PQ signature, paired.
///
/// Serde-friendly; designed to drop in where a plain Ed25519 signature
/// currently lives. When `pq_sig` is `None`, the hybrid degrades to
/// pure Ed25519 (pre-Phase-17 compatibility). When `pq_sig` is `Some`,
/// both halves MUST verify.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HybridSignature {
    pub ed25519_sig: Vec<u8>,
    #[serde(default)]
    pub pq_sig: Option<Vec<u8>>,
    /// PQ verification key, serialized in the scheme's canonical
    /// encoding. Ships alongside the signature so verifiers don't
    /// need out-of-band key distribution for v1-of-v2 transitions.
    /// Required iff `pq_sig.is_some()`.
    #[serde(default)]
    pub pq_vk: Option<Vec<u8>>,
}

impl HybridSignature {
    /// True iff this signature carries a PQ half (i.e. was produced by
    /// a Phase-17 or later signer with the PQ half enabled).
    pub fn is_hybrid(&self) -> bool {
        self.pq_sig.is_some()
    }

    /// Verify the Ed25519 half. Always required.
    ///
    /// NOTE: this does NOT verify the PQ half — use [`Self::verify`]
    /// for the full both-or-fail check.
    pub fn verify_ed25519(
        &self,
        ed25519_vk: &VerifyingKey,
        msg: &[u8],
    ) -> Result<(), HybridError> {
        if self.ed25519_sig.len() != 64 {
            return Err(HybridError::MalformedEd25519);
        }
        let sig_bytes: [u8; 64] = self.ed25519_sig[..]
            .try_into()
            .map_err(|_| HybridError::MalformedEd25519)?;
        let sig = ed25519_dalek::Signature::from_bytes(&sig_bytes);
        ed25519_vk
            .verify_strict(msg, &sig)
            .map_err(|_| HybridError::InvalidEd25519Signature)
    }

    /// Verify both halves — Ed25519 always, PQ iff present. When the
    /// PQ half is present, a missing `pq_vk` is a protocol error
    /// ([`HybridError::MissingPqKey`]). Callers that want to force PQ
    /// presence (i.e. reject pure-Ed25519 peers) should pre-check
    /// [`Self::is_hybrid`] before calling verify.
    pub fn verify<V: PqVerifier>(
        &self,
        ed25519_vk: &VerifyingKey,
        msg: &[u8],
        pq_verifier: &V,
    ) -> Result<(), HybridError> {
        self.verify_ed25519(ed25519_vk, msg)?;
        if let Some(pq_sig) = &self.pq_sig {
            let pq_vk = self.pq_vk.as_ref().ok_or(HybridError::MissingPqKey)?;
            pq_verifier
                .verify_pq(pq_vk, msg, pq_sig)
                .map_err(HybridError::PqFailure)?;
        }
        Ok(())
    }
}

/// Errors returned from [`HybridSignature::verify`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HybridError {
    #[error("ed25519 signature has wrong length (expected 64 bytes)")]
    MalformedEd25519,
    #[error("ed25519 signature failed mathematical verification")]
    InvalidEd25519Signature,
    #[error("hybrid signature includes a PQ sig but no PQ verification key")]
    MissingPqKey,
    #[error("post-quantum signature verification failed: {0:?}")]
    PqFailure(PqError),
}

// ---------------------------------------------------------------------------
// HybridKey — the signer side
// ---------------------------------------------------------------------------

/// Ed25519 + PQ keypair for producing [`HybridSignature`] values.
///
/// The PQ half is held polymorphically as a boxed [`PqSigner`] so a
/// node can be configured at construction time with Mock (tests),
/// MlDsa (production, once the dep lands), Falcon (future), etc.
pub struct HybridKey {
    pub ed25519: SigningKey,
    pub pq: Option<Box<dyn PqSigner + Send + Sync>>,
}

impl std::fmt::Debug for HybridKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HybridKey")
            .field("ed25519_vk", &hex::encode(self.ed25519.verifying_key().to_bytes()))
            .field("pq", &self.pq.as_ref().map(|_| "<boxed PqSigner>"))
            .finish()
    }
}

impl HybridKey {
    /// Construct from an existing Ed25519 key with no PQ half.
    /// The resulting signatures degrade to pure Ed25519 (pre-Phase-17).
    pub fn ed25519_only(ed25519: SigningKey) -> Self {
        Self { ed25519, pq: None }
    }

    /// Construct a full hybrid key from an Ed25519 + PQ signer pair.
    pub fn new<P: PqSigner + Send + Sync + 'static>(ed25519: SigningKey, pq: P) -> Self {
        Self {
            ed25519,
            pq: Some(Box::new(pq)),
        }
    }

    /// Produce a hybrid signature over `msg`.
    pub fn sign(&self, msg: &[u8]) -> HybridSignature {
        let ed = self.ed25519.sign(msg);
        let ed_bytes = ed.to_bytes().to_vec();
        if let Some(pq) = self.pq.as_ref() {
            HybridSignature {
                ed25519_sig: ed_bytes,
                pq_sig: Some(pq.sign_pq(msg)),
                pq_vk: Some(pq.pq_verifying_key()),
            }
        } else {
            HybridSignature {
                ed25519_sig: ed_bytes,
                pq_sig: None,
                pq_vk: None,
            }
        }
    }

    /// Ed25519 public key for this hybrid — callers hand this to
    /// [`HybridSignature::verify`] on the far end.
    pub fn verifying_key(&self) -> VerifyingKey {
        self.ed25519.verifying_key()
    }
}

// ---------------------------------------------------------------------------
// MockPqSigner — test-only, deterministic
// ---------------------------------------------------------------------------

/// Deterministic test signer. The "signature" is just SHA-256 of the
/// message concatenated with a static salt; useless in production, but
/// lets us exercise every branch of the hybrid flow without depending
/// on the real ML-DSA crate.
///
/// **Do not use in production.** The rollout plan is:
///   1. (this wave) Mock wired + tested; no actual PQ resistance.
///   2. When ml-dsa 0.1 stabilizes, add an `MlDsaSigner` wrapper and
///      flip the daemon config switch.
///   3. Eventually make PQ signatures mandatory once all peers are
///      known to be capable of producing them.
#[derive(Debug, Clone)]
pub struct MockPqSigner {
    pub vk: Vec<u8>,
    pub secret: Vec<u8>,
}

impl MockPqSigner {
    /// Construct with a fresh 32-byte random "secret" and a matching
    /// 32-byte public key derived as SHA-256(secret). Deterministic
    /// from the secret so tests can reproduce outcomes.
    pub fn generate(rng: &mut impl rand::RngCore) -> Self {
        let mut secret = vec![0u8; 32];
        rng.fill_bytes(&mut secret);
        let mut hasher = Sha256::new();
        hasher.update(&secret);
        let vk = hasher.finalize().to_vec();
        Self { vk, secret }
    }
}

impl PqSigner for MockPqSigner {
    fn sign_pq(&self, msg: &[u8]) -> Vec<u8> {
        // Scaffold contract: sig := SHA-256(pq_vk || msg). This is
        // cryptographically meaningless (the vk is public, so anyone
        // can forge), but it lets us exercise the FULL verify-fails-
        // on-tamper matrix in tests. The real ML-DSA binding replaces
        // this with proper lattice signing.
        let mut hasher = Sha256::new();
        hasher.update(&self.vk);
        hasher.update(msg);
        hasher.finalize().to_vec()
    }

    fn pq_verifying_key(&self) -> Vec<u8> {
        self.vk.clone()
    }
}

/// Matching verifier for [`MockPqSigner`]. Checks the shape (32-byte
/// vk, 32-byte sig) and recomputes `SHA-256(pq_vk || msg)` to compare.
/// Catches any tampering to either the key, the message, or the
/// signature — enough structural coverage for the hybrid-flow tests.
///
/// The production `MlDsaVerifier` will be rigorously cryptographic.
#[derive(Debug, Clone, Default)]
pub struct MockPqVerifier;

impl PqVerifier for MockPqVerifier {
    fn verify_pq(&self, pq_vk: &[u8], msg: &[u8], pq_sig: &[u8]) -> Result<(), PqError> {
        if pq_vk.len() != 32 {
            return Err(PqError::InvalidKey);
        }
        if pq_sig.len() != 32 {
            return Err(PqError::InvalidSignature);
        }
        let mut hasher = Sha256::new();
        hasher.update(pq_vk);
        hasher.update(msg);
        let expected = hasher.finalize();
        // Constant-time byte comparison via subtle-style accumulation.
        let mut diff: u8 = 0;
        for (a, b) in expected.iter().zip(pq_sig.iter()) {
            diff |= a ^ b;
        }
        if diff != 0 {
            return Err(PqError::InvalidSignature);
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    fn ed_key() -> SigningKey {
        SigningKey::generate(&mut OsRng)
    }

    #[test]
    fn ed25519_only_degrades_to_non_hybrid() {
        let k = HybridKey::ed25519_only(ed_key());
        let sig = k.sign(b"hello");
        assert!(!sig.is_hybrid());
        assert!(sig.pq_sig.is_none());
        assert!(sig.pq_vk.is_none());
        assert!(sig.verify(&k.verifying_key(), b"hello", &MockPqVerifier).is_ok());
    }

    #[test]
    fn hybrid_signature_roundtrips_ed25519_and_pq() {
        let mut rng = OsRng;
        let k = HybridKey::new(ed_key(), MockPqSigner::generate(&mut rng));
        let sig = k.sign(b"payload");
        assert!(sig.is_hybrid());
        assert_eq!(sig.ed25519_sig.len(), 64);
        assert_eq!(sig.pq_sig.as_ref().unwrap().len(), 32);
        assert_eq!(sig.pq_vk.as_ref().unwrap().len(), 32);
        assert!(sig.verify(&k.verifying_key(), b"payload", &MockPqVerifier).is_ok());
    }

    #[test]
    fn verify_fails_on_tampered_message() {
        let mut rng = OsRng;
        let k = HybridKey::new(ed_key(), MockPqSigner::generate(&mut rng));
        let sig = k.sign(b"good-msg");
        // Either the Ed25519 half OR the PQ half must reject a tampered
        // message. Neither should silently pass.
        let err = sig
            .verify(&k.verifying_key(), b"evil-msg", &MockPqVerifier)
            .unwrap_err();
        assert!(matches!(
            err,
            HybridError::InvalidEd25519Signature | HybridError::PqFailure(_)
        ));
    }

    #[test]
    fn verify_fails_on_wrong_ed25519_key() {
        let mut rng = OsRng;
        let signer = HybridKey::new(ed_key(), MockPqSigner::generate(&mut rng));
        let sig = signer.sign(b"x");
        let attacker = ed_key();
        let err = sig
            .verify(&attacker.verifying_key(), b"x", &MockPqVerifier)
            .unwrap_err();
        assert_eq!(err, HybridError::InvalidEd25519Signature);
    }

    #[test]
    fn verify_fails_on_missing_pq_key_when_pq_sig_present() {
        // Construct a malformed hybrid that includes a pq_sig but no
        // pq_vk — catch at verify time.
        let k = HybridKey::ed25519_only(ed_key());
        let mut sig = k.sign(b"m");
        sig.pq_sig = Some(vec![0u8; 32]); // fake
        sig.pq_vk = None;
        let err = sig.verify(&k.verifying_key(), b"m", &MockPqVerifier).unwrap_err();
        assert_eq!(err, HybridError::MissingPqKey);
    }

    #[test]
    fn verify_fails_on_tampered_pq_sig() {
        let mut rng = OsRng;
        let k = HybridKey::new(ed_key(), MockPqSigner::generate(&mut rng));
        let mut sig = k.sign(b"m");
        // Flip one bit in the PQ signature.
        sig.pq_sig.as_mut().unwrap()[0] ^= 0xFF;
        let err = sig.verify(&k.verifying_key(), b"m", &MockPqVerifier).unwrap_err();
        assert!(matches!(err, HybridError::PqFailure(_)));
    }

    #[test]
    fn malformed_ed25519_length_is_rejected_early() {
        let sig = HybridSignature {
            ed25519_sig: vec![0u8; 63], // wrong length
            pq_sig: None,
            pq_vk: None,
        };
        let vk = ed_key().verifying_key();
        let err = sig.verify(&vk, b"x", &MockPqVerifier).unwrap_err();
        assert_eq!(err, HybridError::MalformedEd25519);
    }

    #[test]
    fn is_hybrid_distinguishes_by_pq_sig_presence() {
        let sig_plain = HybridSignature {
            ed25519_sig: vec![0u8; 64],
            pq_sig: None,
            pq_vk: None,
        };
        let sig_pq = HybridSignature {
            ed25519_sig: vec![0u8; 64],
            pq_sig: Some(vec![0u8; 32]),
            pq_vk: Some(vec![0u8; 32]),
        };
        assert!(!sig_plain.is_hybrid());
        assert!(sig_pq.is_hybrid());
    }

    #[test]
    fn hybrid_signature_serde_roundtrip() {
        // Wire shape must survive serde round-trip with no field loss.
        let mut rng = OsRng;
        let k = HybridKey::new(ed_key(), MockPqSigner::generate(&mut rng));
        let sig = k.sign(b"m");
        let json = serde_json::to_string(&sig).unwrap();
        let back: HybridSignature = serde_json::from_str(&json).unwrap();
        assert_eq!(back, sig);
        assert!(back.verify(&k.verifying_key(), b"m", &MockPqVerifier).is_ok());
    }

    #[test]
    fn legacy_wire_without_pq_fields_deserializes_via_default() {
        // Pre-Phase-17 peers send just `ed25519_sig`. `#[serde(default)]`
        // on pq_sig / pq_vk lets this still parse.
        let json = r#"{"ed25519_sig":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}"#;
        let parsed: HybridSignature = serde_json::from_str(json).unwrap();
        assert!(parsed.pq_sig.is_none());
        assert!(parsed.pq_vk.is_none());
        assert!(!parsed.is_hybrid());
    }

    #[test]
    fn mock_pq_verifier_rejects_wrong_vk_length() {
        let v = MockPqVerifier;
        assert_eq!(v.verify_pq(&[0u8; 10], b"m", &[0u8; 32]), Err(PqError::InvalidKey));
    }

    #[test]
    fn mock_pq_verifier_rejects_wrong_sig_length() {
        let v = MockPqVerifier;
        assert_eq!(
            v.verify_pq(&[0u8; 32], b"m", &[0u8; 10]),
            Err(PqError::InvalidSignature)
        );
    }
}

