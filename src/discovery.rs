//! Peer discovery system for Chaincraft

use crate::{
    error::{ChaincraftError, Result},
    network::{PeerId, PeerInfo},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

/// Discovery message types for peer-to-peer discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMessage {
    /// Announce this node to others
    Announce {
        node_id: PeerId,
        socket_addr: SocketAddr,
        timestamp: u64,
    },
    /// Request known peers from a node
    PeerRequest {
        requester_id: PeerId,
        max_peers: usize,
    },
    /// Response with known peers
    PeerResponse { peers: Vec<PeerAnnouncement> },
    /// Ping to check if peer is alive
    Ping { sender_id: PeerId, timestamp: u64 },
    /// Pong response to ping
    Pong {
        responder_id: PeerId,
        timestamp: u64,
    },
}

/// Peer announcement structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerAnnouncement {
    pub node_id: PeerId,
    pub socket_addr: SocketAddr,
    pub last_seen: u64,
    pub announced_at: u64,
}

/// Discovery configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Maximum number of peers to maintain
    pub max_peers: usize,
    /// How often to ping peers (seconds)
    pub ping_interval: u64,
    /// Peer timeout (seconds)
    pub peer_timeout: u64,
    /// Discovery announcement interval (seconds)
    pub announce_interval: u64,
    /// Enable discovery protocol
    pub enabled: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            max_peers: 50,
            ping_interval: 30,
            peer_timeout: 120,
            announce_interval: 60,
            enabled: true,
        }
    }
}

/// Peer discovery manager
pub struct DiscoveryManager {
    /// This node's ID
    node_id: PeerId,
    /// This node's socket address
    socket_addr: SocketAddr,
    /// Known peers
    peers: Arc<RwLock<HashMap<PeerId, PeerAnnouncement>>>,
    /// Connected peers
    connected_peers: Arc<RwLock<HashSet<PeerId>>>,
    /// Discovery configuration
    config: DiscoveryConfig,
    /// Last announcement time
    last_announce: Arc<RwLock<Option<Instant>>>,
}

impl DiscoveryManager {
    /// Create a new discovery manager
    pub fn new(node_id: PeerId, socket_addr: SocketAddr, config: DiscoveryConfig) -> Self {
        Self {
            node_id,
            socket_addr,
            peers: Arc::new(RwLock::new(HashMap::new())),
            connected_peers: Arc::new(RwLock::new(HashSet::new())),
            config,
            last_announce: Arc::new(RwLock::new(None)),
        }
    }

    /// Add a peer to the known peers list
    pub async fn add_peer(&self, peer_info: PeerInfo) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let announcement = PeerAnnouncement {
            node_id: peer_info.id.clone(),
            socket_addr: peer_info.address,
            last_seen: now,
            announced_at: now,
        };

        let mut peers = self.peers.write().await;
        peers.insert(peer_info.id, announcement);

        // If we have too many peers, remove the oldest ones
        if peers.len() > self.config.max_peers {
            let mut peer_last_seen: Vec<(PeerId, u64)> = peers
                .iter()
                .map(|(id, ann)| (id.clone(), ann.last_seen))
                .collect();

            peer_last_seen.sort_by_key(|(_, last_seen)| *last_seen);

            let oldest_peers: Vec<PeerId> = peer_last_seen
                .into_iter()
                .take(peers.len() - self.config.max_peers)
                .map(|(id, _)| id)
                .collect();

            for peer_id in oldest_peers {
                peers.remove(&peer_id);
            }
        }

        Ok(())
    }

    /// Remove a peer from known peers
    pub async fn remove_peer(&self, peer_id: &PeerId) -> Result<()> {
        let mut peers = self.peers.write().await;
        peers.remove(peer_id);

        let mut connected = self.connected_peers.write().await;
        connected.remove(peer_id);

        Ok(())
    }

    /// Mark a peer as connected
    pub async fn mark_connected(&self, peer_id: &PeerId) -> Result<()> {
        let mut connected = self.connected_peers.write().await;
        connected.insert(peer_id.clone());

        // Update last seen time
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.last_seen = now;
        }

        Ok(())
    }

    /// Mark a peer as disconnected
    pub async fn mark_disconnected(&self, peer_id: &PeerId) -> Result<()> {
        let mut connected = self.connected_peers.write().await;
        connected.remove(peer_id);
        Ok(())
    }

    /// Get all known peers
    pub async fn get_peers(&self) -> Vec<PeerAnnouncement> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    /// Get connected peers
    pub async fn get_connected_peers(&self) -> Vec<PeerId> {
        let connected = self.connected_peers.read().await;
        connected.iter().cloned().collect()
    }

    /// Get peers for discovery response (excluding requester and already connected)
    pub async fn get_peers_for_discovery(
        &self,
        requester_id: &PeerId,
        max_peers: usize,
    ) -> Vec<PeerAnnouncement> {
        let peers = self.peers.read().await;
        let connected = self.connected_peers.read().await;

        peers
            .values()
            .filter(|peer| &peer.node_id != requester_id && !connected.contains(&peer.node_id))
            .take(max_peers)
            .cloned()
            .collect()
    }

    /// Handle discovery message
    pub async fn handle_message(
        &self,
        message: DiscoveryMessage,
        sender_addr: SocketAddr,
    ) -> Result<Option<DiscoveryMessage>> {
        match message {
            DiscoveryMessage::Announce {
                node_id,
                socket_addr,
                timestamp: _,
            } => {
                // Add the announcing peer to our known peers
                let peer_info = PeerInfo::new(node_id, socket_addr);
                self.add_peer(peer_info).await?;
                Ok(None)
            },

            DiscoveryMessage::PeerRequest {
                requester_id,
                max_peers,
            } => {
                // Respond with known peers
                let peers = self.get_peers_for_discovery(&requester_id, max_peers).await;
                Ok(Some(DiscoveryMessage::PeerResponse { peers }))
            },

            DiscoveryMessage::PeerResponse { peers } => {
                // Add all peers from the response
                for peer_announcement in peers {
                    let peer_info =
                        PeerInfo::new(peer_announcement.node_id, peer_announcement.socket_addr);
                    self.add_peer(peer_info).await?;
                }
                Ok(None)
            },

            DiscoveryMessage::Ping {
                sender_id,
                timestamp: _,
            } => {
                // Respond with pong
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                Ok(Some(DiscoveryMessage::Pong {
                    responder_id: self.node_id.clone(),
                    timestamp: now,
                }))
            },

            DiscoveryMessage::Pong {
                responder_id,
                timestamp: _,
            } => {
                // Update last seen time for the responder
                self.mark_connected(&responder_id).await?;
                Ok(None)
            },
        }
    }

    /// Create an announcement message
    pub fn create_announcement(&self) -> DiscoveryMessage {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        DiscoveryMessage::Announce {
            node_id: self.node_id.clone(),
            socket_addr: self.socket_addr,
            timestamp: now,
        }
    }

    /// Create a peer request message
    pub fn create_peer_request(&self, max_peers: usize) -> DiscoveryMessage {
        DiscoveryMessage::PeerRequest {
            requester_id: self.node_id.clone(),
            max_peers,
        }
    }

    /// Create a ping message
    pub fn create_ping(&self) -> DiscoveryMessage {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        DiscoveryMessage::Ping {
            sender_id: self.node_id.clone(),
            timestamp: now,
        }
    }

    /// Check if we should announce ourselves
    pub async fn should_announce(&self) -> bool {
        if !self.config.enabled {
            return false;
        }

        let last_announce = self.last_announce.read().await;
        match *last_announce {
            None => true,
            Some(last) => {
                let elapsed = last.elapsed();
                elapsed >= Duration::from_secs(self.config.announce_interval)
            },
        }
    }

    /// Update last announce time
    pub async fn update_last_announce(&self) {
        let mut last_announce = self.last_announce.write().await;
        *last_announce = Some(Instant::now());
    }

    /// Clean up old peers
    pub async fn cleanup_old_peers(&self) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut peers = self.peers.write().await;
        let mut connected = self.connected_peers.write().await;

        let timeout_threshold = now - self.config.peer_timeout;
        let old_peers: Vec<PeerId> = peers
            .iter()
            .filter(|(_, peer)| peer.last_seen < timeout_threshold)
            .map(|(id, _)| id.clone())
            .collect();

        for peer_id in old_peers {
            peers.remove(&peer_id);
            connected.remove(&peer_id);
        }

        Ok(())
    }

    /// Get discovery statistics
    pub async fn get_stats(&self) -> DiscoveryStats {
        let peers = self.peers.read().await;
        let connected = self.connected_peers.read().await;

        DiscoveryStats {
            total_known_peers: peers.len(),
            connected_peers: connected.len(),
            max_peers: self.config.max_peers,
        }
    }
}

/// Discovery statistics
#[derive(Debug, Clone)]
pub struct DiscoveryStats {
    pub total_known_peers: usize,
    pub connected_peers: usize,
    pub max_peers: usize,
}

// Helper trait for sorting (simplified)
trait SortedByKey<T> {
    fn sorted_by_key<K, F>(self, f: F) -> Vec<T>
    where
        F: FnMut(&T) -> K,
        K: Ord;
}

impl<T> SortedByKey<T> for Vec<T> {
    fn sorted_by_key<K, F>(mut self, f: F) -> Vec<T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.sort_by_key(f);
        self
    }
}
