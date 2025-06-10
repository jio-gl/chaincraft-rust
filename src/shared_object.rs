//! Enhanced shared object implementation with application-specific logic

pub use crate::shared::SharedObjectId;
use crate::{
    error::{ChaincraftError, Result},
    shared::{MessageType, SharedMessage, SharedObject},
};
use async_trait::async_trait;
use chrono;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Enhanced shared object trait with application-specific functionality
#[async_trait]
pub trait ApplicationObject: Send + Sync + std::fmt::Debug {
    /// Get the object's unique identifier
    fn id(&self) -> &SharedObjectId;

    /// Get the object's type name
    fn type_name(&self) -> &'static str;

    /// Validate if a message is valid for this object
    async fn is_valid(&self, message: &SharedMessage) -> Result<bool>;

    /// Add a validated message to the object
    async fn add_message(&mut self, message: SharedMessage) -> Result<()>;

    /// Check if this object supports merkleized synchronization
    fn is_merkleized(&self) -> bool;

    /// Get the latest state digest
    async fn get_latest_digest(&self) -> Result<String>;

    /// Check if object has a specific digest
    async fn has_digest(&self, digest: &str) -> Result<bool>;

    /// Validate if a digest is valid
    async fn is_valid_digest(&self, digest: &str) -> Result<bool>;

    /// Add a digest to the object
    async fn add_digest(&mut self, digest: String) -> Result<bool>;

    /// Get messages for gossip protocol
    async fn gossip_messages(&self, digest: Option<&str>) -> Result<Vec<SharedMessage>>;

    /// Get messages since a specific digest
    async fn get_messages_since_digest(&self, digest: &str) -> Result<Vec<SharedMessage>>;

    /// Get the current state as JSON
    async fn get_state(&self) -> Result<Value>;

    /// Reset the object to initial state
    async fn reset(&mut self) -> Result<()>;

    /// Clone the object
    fn clone_box(&self) -> Box<dyn ApplicationObject>;

    /// Get reference as Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Get mutable reference as Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Simple shared number object for testing (equivalent to Python SimpleSharedNumber)
#[derive(Debug, Clone)]
pub struct SimpleSharedNumber {
    id: SharedObjectId,
    number: i64,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    locked: bool,
    messages: Vec<SharedMessage>,
    seen_hashes: HashSet<String>,
    digests: Vec<String>,
}

impl SimpleSharedNumber {
    pub fn new() -> Self {
        Self {
            id: SharedObjectId::new(),
            number: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            locked: false,
            messages: Vec::new(),
            seen_hashes: HashSet::new(),
            digests: Vec::new(),
        }
    }

    pub fn get_number(&self) -> i64 {
        self.number
    }

    pub fn get_messages(&self) -> &[SharedMessage] {
        &self.messages
    }

    fn calculate_message_hash(data: &Value) -> String {
        let data_str = serde_json::to_string(&serde_json::json!({
            "content": data
        }))
        .unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(data_str.as_bytes());
        hex::encode(hasher.finalize())
    }
}

impl Default for SimpleSharedNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApplicationObject for SimpleSharedNumber {
    fn id(&self) -> &SharedObjectId {
        &self.id
    }

    fn type_name(&self) -> &'static str {
        "SimpleSharedNumber"
    }

    async fn is_valid(&self, message: &SharedMessage) -> Result<bool> {
        // We only accept integer data
        Ok(message.data.is_i64())
    }

    async fn add_message(&mut self, message: SharedMessage) -> Result<()> {
        // Deduplicate by hashing the message's data field
        let msg_hash = Self::calculate_message_hash(&message.data);

        if self.seen_hashes.contains(&msg_hash) {
            // Already processed this data
            return Ok(());
        }

        self.seen_hashes.insert(msg_hash);

        // Extract the integer value and add to our number
        if let Some(value) = message.data.as_i64() {
            self.number += value;
            self.messages.push(message);
            tracing::info!("SimpleSharedNumber: Added message with data: {}", value);
        }

        Ok(())
    }

    fn is_merkleized(&self) -> bool {
        false
    }

    async fn get_latest_digest(&self) -> Result<String> {
        Ok(self.number.to_string())
    }

    async fn has_digest(&self, digest: &str) -> Result<bool> {
        Ok(self.digests.contains(&digest.to_string()))
    }

    async fn is_valid_digest(&self, _digest: &str) -> Result<bool> {
        Ok(true)
    }

    async fn add_digest(&mut self, digest: String) -> Result<bool> {
        self.digests.push(digest);
        Ok(true)
    }

    async fn gossip_messages(&self, _digest: Option<&str>) -> Result<Vec<SharedMessage>> {
        Ok(Vec::new())
    }

    async fn get_messages_since_digest(&self, _digest: &str) -> Result<Vec<SharedMessage>> {
        Ok(Vec::new())
    }

    async fn get_state(&self) -> Result<Value> {
        Ok(serde_json::json!({
            "number": self.number,
            "message_count": self.messages.len(),
            "seen_hashes_count": self.seen_hashes.len()
        }))
    }

    async fn reset(&mut self) -> Result<()> {
        self.number = 0;
        self.messages.clear();
        self.seen_hashes.clear();
        self.digests.clear();
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn ApplicationObject> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Registry for managing application objects
#[derive(Debug)]
pub struct ApplicationObjectRegistry {
    objects: HashMap<SharedObjectId, Box<dyn ApplicationObject>>,
    objects_by_type: HashMap<String, Vec<SharedObjectId>>,
}

impl ApplicationObjectRegistry {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            objects_by_type: HashMap::new(),
        }
    }

    /// Register a new application object
    pub fn register(&mut self, object: Box<dyn ApplicationObject>) -> SharedObjectId {
        let id = object.id().clone();
        let type_name = object.type_name().to_string();

        self.objects_by_type
            .entry(type_name)
            .or_default()
            .push(id.clone());

        self.objects.insert(id.clone(), object);
        id
    }

    /// Get an object by ID
    pub fn get(&self, id: &SharedObjectId) -> Option<&dyn ApplicationObject> {
        self.objects.get(id).map(|obj| obj.as_ref())
    }

    /// Get all objects of a specific type (returning owned clones for safety)
    pub fn get_by_type(&self, type_name: &str) -> Vec<Box<dyn ApplicationObject>> {
        self.objects_by_type
            .get(type_name)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.objects.get(id))
                    .map(|obj| obj.clone_box())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove an object
    pub fn remove(&mut self, id: &SharedObjectId) -> Option<Box<dyn ApplicationObject>> {
        if let Some(object) = self.objects.remove(id) {
            let type_name = object.type_name().to_string();
            if let Some(type_list) = self.objects_by_type.get_mut(&type_name) {
                type_list.retain(|obj_id| obj_id != id);
                if type_list.is_empty() {
                    self.objects_by_type.remove(&type_name);
                }
            }
            Some(object)
        } else {
            None
        }
    }

    /// Get all object IDs
    pub fn ids(&self) -> Vec<SharedObjectId> {
        self.objects.keys().cloned().collect()
    }

    /// Get count of objects
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Clear all objects
    pub fn clear(&mut self) {
        self.objects.clear();
        self.objects_by_type.clear();
    }

    /// Process a message against all appropriate objects
    pub async fn process_message(&mut self, message: SharedMessage) -> Result<Vec<SharedObjectId>> {
        let mut processed_objects = Vec::new();

        // Get all object IDs first to avoid borrow checker issues
        let ids: Vec<SharedObjectId> = self.objects.keys().cloned().collect();

        // Process each object sequentially
        for id in ids {
            // Check validity first
            let is_valid = if let Some(object) = self.objects.get(&id) {
                object.is_valid(&message).await?
            } else {
                false
            };

            // If valid, add the message
            if is_valid {
                if let Some(object) = self.objects.get_mut(&id) {
                    object.add_message(message.clone()).await?;
                    processed_objects.push(id);
                }
            }
        }

        Ok(processed_objects)
    }
}

impl Default for ApplicationObjectRegistry {
    fn default() -> Self {
        Self::new()
    }
}
