//! Tests for browser_context_switch tool

use serde_json::json;
use viewpoint_mcp::tools::{
    BrowserContextCreateTool, BrowserContextSwitchTool, BrowserNavigateTool, Tool,
};

use super::create_browser;

#[tokio::test]
async fn test_context_switch_basic() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let switch_tool = BrowserContextSwitchTool::new();

    // Create additional context
    create_tool
        .execute(&json!({ "name": "other" }), &mut browser)
        .await
        .unwrap();

    // Switch back to default
    let result = switch_tool
        .execute(&json!({ "name": "default" }), &mut browser)
        .await;

    assert!(result.is_ok());
    assert_eq!(browser.active_context_name(), "default");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_switch_nonexistent() {
    let mut browser = create_browser().await;
    let switch_tool = BrowserContextSwitchTool::new();

    let result = switch_tool
        .execute(&json!({ "name": "nonexistent" }), &mut browser)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found") || err.contains("does not exist"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_switch_preserves_state() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let switch_tool = BrowserContextSwitchTool::new();
    let nav_tool = BrowserNavigateTool::new();

    // Create two contexts with different pages
    create_tool
        .execute(&json!({ "name": "ctx_a" }), &mut browser)
        .await
        .unwrap();
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Context A</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    create_tool
        .execute(&json!({ "name": "ctx_b" }), &mut browser)
        .await
        .unwrap();
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Context B</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Switch back to A
    switch_tool
        .execute(&json!({ "name": "ctx_a" }), &mut browser)
        .await
        .unwrap();

    // Context A should still have its URL
    let ctx = browser.active_context().unwrap();
    assert!(
        ctx.current_url
            .as_ref()
            .map(|u| u.contains("Context A"))
            .unwrap_or(false)
    );

    browser.shutdown().await;
}
