use ed25519_dalek::Signature as Ed25519Signature;
use crate::error::{ChainCraftError, CryptoError, Result};

// Workaround for signature creation
pub fn create_ed25519_signature(bytes: &[u8; 64]) -> Result<Ed25519Signature> {
    // The documentation says this should return a Signature, but the compiler
    // thinks it's returning a Result<Signature, _>. This is likely a version mismatch
    // or API change in the library.
    
    // Use a more direct approach by importing the signature type
    match Ed25519Signature::from_bytes(bytes) {
        Ok(sig) => Ok(sig),
        Err(_) => Err(ChainCraftError::Crypto(CryptoError::InvalidSignature)),
    }
} 