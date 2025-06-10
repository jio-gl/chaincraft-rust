//! Proof of Work implementation

use crate::crypto::KeylessCryptoPrimitive;
use crate::error::{ChaincraftError, CryptoError, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::task;

/// Proof of Work parameters and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfWorkConfig {
    /// Difficulty target (number of leading zeros required)
    pub difficulty: u32,
    /// Maximum nonce to try before giving up
    pub max_nonce: u64,
    /// Number of worker threads to use for mining
    pub threads: usize,
}

impl Default for ProofOfWorkConfig {
    fn default() -> Self {
        Self {
            difficulty: 4,
            max_nonce: u64::MAX,
            threads: num_cpus::get(),
        }
    }
}

/// Proof of Work challenge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoWChallenge {
    pub data: String,
}

impl PoWChallenge {
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }
}

/// Proof of Work proof/solution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoWProof {
    pub nonce: u64,
    pub hash: String,
}

impl PoWProof {
    pub fn new(nonce: u64, hash: String) -> Self {
        Self { nonce, hash }
    }
}

/// High-performance Proof of Work implementation
#[derive(Debug, Clone)]
pub struct ProofOfWork {
    config: ProofOfWorkConfig,
}

impl ProofOfWork {
    /// Create a new Proof of Work instance with default configuration
    pub fn new() -> Self {
        Self {
            config: ProofOfWorkConfig::default(),
        }
    }

    /// Create a new Proof of Work instance with custom configuration
    pub fn with_config(config: ProofOfWorkConfig) -> Self {
        Self { config }
    }

    /// Create a simple PoW with specified difficulty
    pub fn with_difficulty(difficulty: u32) -> Self {
        Self {
            config: ProofOfWorkConfig {
                difficulty,
                ..ProofOfWorkConfig::default()
            },
        }
    }

    /// Calculate hash for given data and nonce
    fn calculate_hash(data: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(nonce.to_le_bytes());
        hex::encode(hasher.finalize())
    }

    /// Check if hash meets difficulty requirement
    fn meets_difficulty(hash: &str, difficulty: u32) -> bool {
        hash.starts_with(&"0".repeat(difficulty as usize))
    }

    /// Mine a single block using CPU-bound work
    async fn mine_worker(
        data: String,
        difficulty: u32,
        start_nonce: u64,
        nonce_step: u64,
        max_nonce: u64,
        should_stop: Arc<AtomicBool>,
        best_nonce: Arc<AtomicU64>,
    ) -> Option<PoWProof> {
        task::spawn_blocking(move || {
            let mut nonce = start_nonce;

            while nonce < max_nonce && !should_stop.load(Ordering::Relaxed) {
                let hash = Self::calculate_hash(&data, nonce);

                if Self::meets_difficulty(&hash, difficulty) {
                    // Found a solution!
                    best_nonce.store(nonce, Ordering::Relaxed);
                    should_stop.store(true, Ordering::Relaxed);
                    return Some(PoWProof::new(nonce, hash));
                }

                nonce = nonce.saturating_add(nonce_step);

                // Yield occasionally to allow other tasks to run
                if nonce % 10000 == 0 && should_stop.load(Ordering::Relaxed) {
                    break;
                }
            }

            None
        })
        .await
        .unwrap_or(None)
    }

    /// Verify proof efficiently
    pub fn verify_sync(&self, challenge: &PoWChallenge, proof: &PoWProof) -> Result<bool> {
        // Verify the hash matches the nonce
        let calculated_hash = Self::calculate_hash(&challenge.data, proof.nonce);
        if calculated_hash != proof.hash {
            return Ok(false);
        }

        // Verify the hash meets difficulty
        Ok(Self::meets_difficulty(&proof.hash, self.config.difficulty))
    }

    /// Get the current difficulty
    pub fn difficulty(&self) -> u32 {
        self.config.difficulty
    }

    /// Set the difficulty
    pub fn set_difficulty(&mut self, difficulty: u32) {
        self.config.difficulty = difficulty;
    }

    /// Estimate time to mine based on hash rate
    pub fn estimate_time(&self, hash_rate: f64) -> std::time::Duration {
        let target = 2_u64.pow(self.config.difficulty);
        let expected_hashes = target as f64;
        let seconds = expected_hashes / hash_rate;
        std::time::Duration::from_secs_f64(seconds)
    }
}

impl Default for ProofOfWork {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl KeylessCryptoPrimitive for ProofOfWork {
    type Input = String;
    type Output = String;
    type Challenge = PoWChallenge;
    type Proof = PoWProof;

    async fn compute(&self, input: Self::Input) -> Result<Self::Output> {
        // Simple hash computation without proof of work
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        Ok(hex::encode(hasher.finalize()))
    }

    async fn create_proof(&self, challenge: Self::Challenge) -> Result<Self::Proof> {
        let should_stop = Arc::new(AtomicBool::new(false));
        let best_nonce = Arc::new(AtomicU64::new(0));
        let mut handles = Vec::new();

        let threads = self.config.threads;
        let nonce_step = threads as u64;

        // Spawn multiple worker tasks for parallel mining
        for i in 0..threads {
            let data = challenge.data.clone();
            let difficulty = self.config.difficulty;
            let start_nonce = i as u64;
            let max_nonce = self.config.max_nonce;
            let should_stop_clone = should_stop.clone();
            let best_nonce_clone = best_nonce.clone();

            let handle = task::spawn(Self::mine_worker(
                data,
                difficulty,
                start_nonce,
                nonce_step,
                max_nonce,
                should_stop_clone,
                best_nonce_clone,
            ));

            handles.push(handle);
        }

        // Wait for any worker to find a solution
        for handle in handles {
            if let Ok(Some(proof)) = handle.await {
                should_stop.store(true, Ordering::Relaxed);
                return Ok(proof);
            }
        }

        Err(ChaincraftError::Crypto(CryptoError::ProofOfWorkFailed))
    }

    async fn verify_proof(&self, challenge: Self::Challenge, proof: Self::Proof) -> Result<bool> {
        // Verification is fast, but we'll make it async for consistency
        let difficulty = self.config.difficulty;
        task::spawn_blocking(move || {
            let calculated_hash = Self::calculate_hash(&challenge.data, proof.nonce);
            if calculated_hash != proof.hash {
                return false;
            }
            Self::meets_difficulty(&proof.hash, difficulty)
        })
        .await
        .map_err(|_| ChaincraftError::Generic("Task join error".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pow_creation_and_verification() {
        let pow = ProofOfWork::with_difficulty(1); // Easy difficulty for testing
        let challenge = PoWChallenge::new("test data");

        let proof = pow.create_proof(challenge.clone()).await.unwrap();
        let is_valid = pow.verify_proof(challenge, proof).await.unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_sync_verification() {
        let pow = ProofOfWork::with_difficulty(2);
        let challenge = PoWChallenge::new("test");

        // This is a known valid proof for "test" with difficulty 2
        let proof = PoWProof::new(0, ProofOfWork::calculate_hash("test", 0));

        // Find a valid nonce manually for testing
        for nonce in 0..10000 {
            let hash = ProofOfWork::calculate_hash("test", nonce);
            if ProofOfWork::meets_difficulty(&hash, 2) {
                let proof = PoWProof::new(nonce, hash);
                assert!(pow.verify_sync(&challenge, &proof).unwrap());
                break;
            }
        }
    }

    #[test]
    fn test_difficulty_check() {
        assert!(ProofOfWork::meets_difficulty("00abc", 2));
        assert!(!ProofOfWork::meets_difficulty("0abc", 2));
        assert!(ProofOfWork::meets_difficulty("000abc", 3));
    }
}
