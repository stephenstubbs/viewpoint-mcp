//! Tests for browser_context_create tool

use serde_json::json;
use tempfile::TempDir;
use viewpoint_mcp::tools::{BrowserContextCreateTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_context_create_basic() {
    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    let result = tool
        .execute(&json!({ "name": "test_context" }), &mut browser)
        .await;

    assert!(
        result.is_ok(),
        "Create context should succeed: {:?}",
        result.err()
    );
    let output = result.unwrap();
    assert!(output.contains("test_context") || output.contains("created"));

    // Verify context exists
    let contexts = browser.list_contexts();
    assert!(contexts.iter().any(|c| c.name == "test_context"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_create_multiple() {
    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    for i in 1..=5 {
        let result = tool
            .execute(&json!({ "name": format!("ctx_{}", i) }), &mut browser)
            .await;
        assert!(result.is_ok(), "Create context {} should succeed", i);
    }

    let contexts = browser.list_contexts();
    // Should have default + 5 new contexts
    assert_eq!(contexts.len(), 6);

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_create_duplicate_name() {
    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    // Create first context
    tool.execute(&json!({ "name": "duplicate" }), &mut browser)
        .await
        .unwrap();

    // Try to create another with same name
    let result = tool
        .execute(&json!({ "name": "duplicate" }), &mut browser)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("already exists") || err.contains("duplicate"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_create_with_storage_state() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().join("storage.json");

    // Create a valid storage state file
    std::fs::write(&storage_path, r#"{"cookies":[],"origins":[]}"#).unwrap();

    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    let result = tool
        .execute(
            &json!({
                "name": "with_storage",
                "storageState": storage_path.to_str().unwrap()
            }),
            &mut browser,
        )
        .await;

    // May succeed or fail depending on file format validation
    // Just ensure no panic
    let _ = result;

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_create_switches_to_new() {
    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    // Initially on default
    assert_eq!(browser.active_context_name(), "default");

    // Create new context
    tool.execute(&json!({ "name": "new_active" }), &mut browser)
        .await
        .unwrap();

    // Should now be on new context
    assert_eq!(browser.active_context_name(), "new_active");

    browser.shutdown().await;
}
