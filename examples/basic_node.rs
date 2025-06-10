//! This example demonstrates how to create and run a basic ChainCraft node
//! with default configuration.

use chaincraft_rust::{error::Result, ChainCraftNode};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for better debugging
    tracing_subscriber::fmt::init();
    
    // Create a new node with default configuration
    let mut node = ChainCraftNode::new_with_defaults().await?;
    
    // Start the node
    node.start().await?;
    println!("Node started with peer ID: {}", node.peer_id());
    
    // Keep the node running for a while
    println!("Node running for 5 seconds...");
    sleep(Duration::from_secs(5)).await;
    
    // Gracefully shut down the node
    println!("Shutting down node...");
    node.close().await?;
    println!("Node shut down successfully");
    
    Ok(())
}
