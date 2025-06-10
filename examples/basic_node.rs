//! Basic ChainCraft Node Example
//!
//! This example demonstrates how to create and run a basic ChainCraft node
//! with default configuration.

use chaincraft_rust::{
    ChainCraftNode, 
    error::Result, 
    network::PeerId, 
    storage::MemoryStorage
};
use serde_json::json;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, Level};
use tracing_subscriber::fmt::format;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Starting basic ChainCraft node example");

    // Create a node with default configuration
    let mut node = ChainCraftNode::builder()
        .port(21000)
        .max_peers(10)
        .build()?;

    info!("Node {} created on port {}", node.id(), node.port());

    // Start the node
    node.start().await?;
    info!("Node started successfully");

    // Let the node run for 30 seconds
    info!("Node will run for 30 seconds...");
    sleep(Duration::from_secs(30)).await;

    // Stop the node
    info!("Stopping node...");
    node.stop().await?;
    info!("Node stopped successfully");

    Ok(())
}
