//! Hash utilities and functions

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sha3::{Keccak256, Sha3_256};

/// Hash algorithms supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Sha256,
    Sha3_256,
    Keccak256,
    Blake3,
}

/// Compute SHA-256 hash
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Compute SHA-256 hash and return as hex string
pub fn sha256_hex(data: &[u8]) -> String {
    hex::encode(sha256(data))
}

/// Compute SHA3-256 hash
pub fn sha3_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Compute Keccak-256 hash (Ethereum style)
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Compute Blake3 hash
pub fn blake3_hash(data: &[u8]) -> [u8; 32] {
    blake3::hash(data).into()
}

/// Generic hash function
pub fn hash_with_algorithm(data: &[u8], algorithm: HashAlgorithm) -> [u8; 32] {
    match algorithm {
        HashAlgorithm::Sha256 => sha256(data),
        HashAlgorithm::Sha3_256 => sha3_256(data),
        HashAlgorithm::Keccak256 => keccak256(data),
        HashAlgorithm::Blake3 => blake3_hash(data),
    }
}

/// Hash multiple pieces of data together
pub fn hash_multiple(data_pieces: &[&[u8]], algorithm: HashAlgorithm) -> [u8; 32] {
    match algorithm {
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            for piece in data_pieces {
                hasher.update(piece);
            }
            hasher.finalize().into()
        },
        HashAlgorithm::Sha3_256 => {
            let mut hasher = Sha3_256::new();
            for piece in data_pieces {
                hasher.update(piece);
            }
            hasher.finalize().into()
        },
        HashAlgorithm::Keccak256 => {
            let mut hasher = Keccak256::new();
            for piece in data_pieces {
                hasher.update(piece);
            }
            hasher.finalize().into()
        },
        HashAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            for piece in data_pieces {
                hasher.update(piece);
            }
            hasher.finalize().into()
        },
    }
}
