//! Tests for browser_close tool

use serde_json::json;
use viewpoint_mcp::tools::{BrowserCloseTool, BrowserTabsTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_close_page() {
    let mut browser = create_browser().await;
    let close_tool = BrowserCloseTool::new();
    let tabs_tool = BrowserTabsTool::new();

    // Create a second tab first
    tabs_tool
        .execute(&json!({ "action": "new" }), &mut browser)
        .await
        .unwrap();

    // Close current page
    let result = close_tool.execute(&json!({}), &mut browser).await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_close_last_page() {
    let mut browser = create_browser().await;
    let close_tool = BrowserCloseTool::new();

    // Try to close the only page
    let result = close_tool.execute(&json!({}), &mut browser).await;

    // May succeed (closing browser) or fail (can't close last page)
    // Just ensure no panic
    let _ = result;

    browser.shutdown().await;
}
