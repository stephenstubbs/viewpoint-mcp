//! Tests for browser_context_save_storage tool

use serde_json::json;
use tempfile::TempDir;
use viewpoint_mcp::tools::{BrowserContextCreateTool, BrowserContextSaveStorageTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_context_save_storage_basic() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().join("saved_storage.json");

    let mut browser = create_browser().await;
    let save_tool = BrowserContextSaveStorageTool::new();

    let result = save_tool
        .execute(
            &json!({
                "path": storage_path.to_str().unwrap()
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Save storage should succeed: {:?}",
        result.err()
    );

    // Verify the file was created
    assert!(storage_path.exists(), "Storage file should be created");

    // Verify the file contains valid JSON
    let content = std::fs::read_to_string(&storage_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(parsed.get("cookies").is_some(), "Should have cookies field");
    assert!(parsed.get("origins").is_some(), "Should have origins field");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_save_storage_named_context() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().join("named_storage.json");

    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let save_tool = BrowserContextSaveStorageTool::new();

    // Create a named context
    create_tool
        .execute(&json!({ "name": "test_storage_ctx" }), &mut browser)
        .await
        .unwrap();

    // Save storage for the named context
    let result = save_tool
        .execute(
            &json!({
                "name": "test_storage_ctx",
                "path": storage_path.to_str().unwrap()
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_ok(),
        "Save storage should succeed: {:?}",
        result.err()
    );

    let output = result.unwrap();
    assert!(
        output.contains("test_storage_ctx"),
        "Output should mention context name"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_save_storage_nonexistent_context() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().join("nonexistent_storage.json");

    let mut browser = create_browser().await;
    let save_tool = BrowserContextSaveStorageTool::new();

    let result = save_tool
        .execute(
            &json!({
                "name": "nonexistent_context",
                "path": storage_path.to_str().unwrap()
            }),
            &mut browser,
        )
        .await;

    assert!(
        result.is_err(),
        "Save storage should fail for nonexistent context"
    );
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("not found"),
        "Error should indicate context not found: {}",
        err
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_save_storage_empty_path() {
    let mut browser = create_browser().await;
    let save_tool = BrowserContextSaveStorageTool::new();

    let result = save_tool
        .execute(
            &json!({
                "path": ""
            }),
            &mut browser,
        )
        .await;

    assert!(result.is_err(), "Save storage should fail for empty path");
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("empty") || err.contains("Path"),
        "Error should mention path issue: {}",
        err
    );

    browser.shutdown().await;
}
