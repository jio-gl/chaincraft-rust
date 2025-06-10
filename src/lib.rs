//! ChainCraft - A blockchain education and prototyping platform
//! Rust implementation

#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// Modules
pub mod consensus;
pub mod crypto;
pub mod discovery;
pub mod error;
pub mod examples;
pub mod network;
pub mod node;
pub mod shared;
pub mod shared_object;
pub mod storage;
pub mod types;
pub mod utils;

// Re-exports
pub use error::{ChainCraftError, Result};
pub use network::{PeerId, PeerInfo};
pub use node::ChainCraftNode;
pub use shared::{SharedMessage, SharedObject, SharedObjectId, SharedObjectRegistry};

// Application object re-exports
pub use shared_object::{ApplicationObject, ApplicationObjectRegistry, SimpleSharedNumber};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Default network port for ChainCraft nodes
pub const DEFAULT_PORT: u16 = 21000;

/// Maximum number of peers by default
pub const DEFAULT_MAX_PEERS: usize = 10;

/// Default gossip interval in milliseconds
pub const DEFAULT_GOSSIP_INTERVAL_MS: u64 = 500;
