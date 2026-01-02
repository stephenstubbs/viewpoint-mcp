//! Tests for browser_context_list tool

use serde_json::json;
use viewpoint_mcp::tools::{BrowserContextCreateTool, BrowserContextListTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_context_list_default_only() {
    let mut browser = create_browser().await;
    let list_tool = BrowserContextListTool::new();

    let result = list_tool.execute(&json!({}), &mut browser).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("default"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_list_multiple() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let list_tool = BrowserContextListTool::new();

    // Create multiple contexts
    for name in ["alpha", "beta", "gamma"] {
        create_tool
            .execute(&json!({ "name": name }), &mut browser)
            .await
            .unwrap();
    }

    let result = list_tool.execute(&json!({}), &mut browser).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("default"));
    assert!(output.contains("alpha"));
    assert!(output.contains("beta"));
    assert!(output.contains("gamma"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_list_shows_active() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let list_tool = BrowserContextListTool::new();

    create_tool
        .execute(&json!({ "name": "active_one" }), &mut browser)
        .await
        .unwrap();

    let result = list_tool.execute(&json!({}), &mut browser).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    // Should indicate which context is active
    assert!(output.contains("active") || output.contains("*") || output.contains("current"));

    browser.shutdown().await;
}
