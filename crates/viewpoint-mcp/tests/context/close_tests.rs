//! Tests for browser_context_close tool

use serde_json::json;
use viewpoint_mcp::tools::{BrowserContextCloseTool, BrowserContextCreateTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_context_close_non_active() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let close_tool = BrowserContextCloseTool::new();

    // Create and switch to new context, then switch back
    create_tool
        .execute(&json!({ "name": "to_close" }), &mut browser)
        .await
        .unwrap();

    // Switch back to default
    browser.switch_context("default").unwrap();

    // Close the other context
    let result = close_tool
        .execute(&json!({ "name": "to_close" }), &mut browser)
        .await;

    assert!(result.is_ok());

    // Verify it's gone
    let contexts = browser.list_contexts();
    assert!(!contexts.iter().any(|c| c.name == "to_close"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_close_active_fallback() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let close_tool = BrowserContextCloseTool::new();

    // Create new context (becomes active)
    create_tool
        .execute(&json!({ "name": "active_ctx" }), &mut browser)
        .await
        .unwrap();

    assert_eq!(browser.active_context_name(), "active_ctx");

    // Close the active context
    let result = close_tool
        .execute(&json!({ "name": "active_ctx" }), &mut browser)
        .await;

    assert!(result.is_ok());
    // Should fall back to default
    assert_eq!(browser.active_context_name(), "default");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_close_last_fails() {
    let mut browser = create_browser().await;
    let close_tool = BrowserContextCloseTool::new();

    // Try to close the only context
    let result = close_tool
        .execute(&json!({ "name": "default" }), &mut browser)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("last") || err.contains("only") || err.contains("cannot"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_close_nonexistent() {
    let mut browser = create_browser().await;
    let close_tool = BrowserContextCloseTool::new();

    let result = close_tool
        .execute(&json!({ "name": "nonexistent" }), &mut browser)
        .await;

    assert!(result.is_err());

    browser.shutdown().await;
}
