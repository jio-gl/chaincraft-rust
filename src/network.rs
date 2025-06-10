//! Networking module for peer-to-peer communication

use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::SocketAddr;
use uuid::Uuid;

/// Unique identifier for a peer
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerId(Uuid);

impl PeerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for PeerId {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: PeerId,
    pub address: SocketAddr,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

impl PeerInfo {
    pub fn new(id: PeerId, address: SocketAddr) -> Self {
        Self {
            id,
            address,
            last_seen: chrono::Utc::now(),
        }
    }
}
