//! Address generation and validation

use crate::crypto::{hash::keccak256, PublicKey};
use serde::{Deserialize, Serialize};

/// Blockchain address
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(String);

impl Address {
    /// Generate Ethereum-style address from public key
    pub fn from_public_key(public_key: &PublicKey) -> Self {
        let hash = keccak256(&public_key.as_bytes());
        let address = format!("0x{}", hex::encode(&hash[12..]));
        Self(address)
    }

    /// Create from hex string
    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() == 42 && hex.starts_with("0x") {
            Some(Self(hex.to_string()))
        } else {
            None
        }
    }

    /// Get as string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get as bytes (without 0x prefix)
    pub fn as_bytes(&self) -> Vec<u8> {
        hex::decode(&self.0[2..]).unwrap_or_default()
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Address {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Address {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
