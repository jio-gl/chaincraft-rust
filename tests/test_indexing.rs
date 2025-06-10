use chaincraft_rust::{network::PeerId, storage::MemoryStorage, ChainCraftNode};
use serde_json::json;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

async fn create_indexed_node() -> ChainCraftNode {
    let id = PeerId::new();
    let storage = Arc::new(MemoryStorage::new());
    let mut node = ChainCraftNode::new(id, storage);
    node.start().await.unwrap();
    node
}

#[tokio::test]
async fn test_basic_indexing_setup() {
    let mut node = create_indexed_node().await;

    // Test that the node starts properly with indexing
    assert!(node.is_running());

    node.close().await.unwrap();
}

#[tokio::test]
async fn test_index_user_messages() {
    let mut node = create_indexed_node().await;

    // Create a user message
    let user_message = json!({
        "message_type": "User",
        "user_id": 1,
        "username": "alice",
        "email": "alice@example.com",
        "bio": "Hello, I'm Alice!"
    });

    node.create_shared_message_with_data(user_message)
        .await
        .unwrap();
    sleep(Duration::from_millis(100)).await;

    // Test passes if no errors occur during message creation
    // assert!(true);

    node.close().await.unwrap();
}

#[tokio::test]
async fn test_index_post_messages() {
    let mut node = create_indexed_node().await;

    // Create a post message with tags
    let post_message = json!({
        "message_type": "Post",
        "post_id": 1,
        "title": "My First Post",
        "content": "Hello, world!",
        "tags": ["introduction", "greeting"],
        "likes": [1, 2, 3]
    });

    node.create_shared_message_with_data(post_message)
        .await
        .unwrap();
    sleep(Duration::from_millis(100)).await;

    // Test passes if no errors occur during message creation
    // assert!(true);

    node.close().await.unwrap();
}

#[tokio::test]
async fn test_query_messages_by_type() {
    let mut node = create_indexed_node().await;

    // Create different message types
    let user_msg = json!({"message_type": "User", "username": "alice"});
    let post_msg = json!({"message_type": "Post", "title": "Hello World"});
    let comment_msg = json!({"message_type": "Comment", "text": "Nice post!"});

    node.create_shared_message_with_data(user_msg)
        .await
        .unwrap();
    node.create_shared_message_with_data(post_msg)
        .await
        .unwrap();
    node.create_shared_message_with_data(comment_msg)
        .await
        .unwrap();

    sleep(Duration::from_millis(100)).await;

    // Test passes if all messages are created without errors
    // assert!(true);

    node.close().await.unwrap();
}

#[tokio::test]
async fn test_complex_message_indexing() {
    let mut node = create_indexed_node().await;

    // Create a complex nested message
    let complex_msg = json!({
        "message_type": "ComplexData",
        "metadata": {
            "created_at": "2024-01-01T00:00:00Z",
            "author": "test_user",
            "version": 1
        },
        "data": {
            "items": [
                {"id": 1, "name": "Item 1"},
                {"id": 2, "name": "Item 2"}
            ],
            "total_count": 2
        }
    });

    node.create_shared_message_with_data(complex_msg)
        .await
        .unwrap();
    sleep(Duration::from_millis(100)).await;

    // Test passes if complex message is created without errors
    // assert!(true);

    node.close().await.unwrap();
}

#[tokio::test]
async fn test_timestamp_indexing() {
    let mut node = create_indexed_node().await;

    // Create messages with specific timestamps
    let msg1 = json!({"message_type": "Event", "event": "start"});
    let msg2 = json!({"message_type": "Event", "event": "middle"});
    let msg3 = json!({"message_type": "Event", "event": "end"});

    node.create_shared_message_with_data(msg1).await.unwrap();
    sleep(Duration::from_millis(10)).await;

    node.create_shared_message_with_data(msg2).await.unwrap();
    sleep(Duration::from_millis(10)).await;

    node.create_shared_message_with_data(msg3).await.unwrap();
    sleep(Duration::from_millis(100)).await;

    // Test passes if all timed messages are created without errors
    // assert!(true);

    node.close().await.unwrap();
}
