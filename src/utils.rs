//! Utility functions and helpers

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Get current timestamp in milliseconds
pub fn current_time_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis() as u64
}

/// Convert bytes to a hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Convert a hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

/// Check if a string is a valid hex string
pub fn is_valid_hex(hex: &str) -> bool {
    hex.chars().all(|c| c.is_ascii_hexdigit())
}
