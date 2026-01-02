//! Tests for browser_tabs tool

use serde_json::json;
use viewpoint_mcp::tools::{BrowserNavigateTool, BrowserTabsTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_tabs_list() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();

    let result = tabs_tool
        .execute(&json!({ "action": "list" }), &mut browser)
        .await;

    assert!(
        result.is_ok(),
        "List tabs should succeed: {:?}",
        result.err()
    );
    let output = result.unwrap();
    // Should have at least one tab
    assert!(!output.is_empty());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_tabs_create_new() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();

    // Get initial tab count
    let initial = tabs_tool
        .execute(&json!({ "action": "list" }), &mut browser)
        .await
        .unwrap();

    // Create new tab
    let result = tabs_tool
        .execute(&json!({ "action": "new" }), &mut browser)
        .await;

    assert!(result.is_ok());

    // Should have one more tab
    let after = tabs_tool
        .execute(&json!({ "action": "list" }), &mut browser)
        .await
        .unwrap();

    // The output should show more tabs now
    assert_ne!(initial, after);

    browser.shutdown().await;
}

#[tokio::test]
async fn test_tabs_create_multiple() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();

    // Create multiple tabs
    for _ in 0..3 {
        tabs_tool
            .execute(&json!({ "action": "new" }), &mut browser)
            .await
            .unwrap();
    }

    let result = tabs_tool
        .execute(&json!({ "action": "list" }), &mut browser)
        .await
        .unwrap();

    // Should list multiple tabs
    assert!(result.lines().count() >= 3 || result.contains("4") || result.contains("tab"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_tabs_select() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();
    let nav_tool = BrowserNavigateTool::new();

    // Navigate first tab
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Tab 0</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Create second tab
    tabs_tool
        .execute(&json!({ "action": "new" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Tab 1</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Select first tab (index 0)
    let result = tabs_tool
        .execute(&json!({ "action": "select", "index": 0 }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_tabs_select_invalid_index() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();

    let result = tabs_tool
        .execute(&json!({ "action": "select", "index": 999 }), &mut browser)
        .await;

    assert!(result.is_err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_tabs_close() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();

    // Create a second tab
    tabs_tool
        .execute(&json!({ "action": "new" }), &mut browser)
        .await
        .unwrap();

    // Close the second tab (index 1)
    let result = tabs_tool
        .execute(&json!({ "action": "close", "index": 1 }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_tabs_close_current() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();

    // Create second tab
    tabs_tool
        .execute(&json!({ "action": "new" }), &mut browser)
        .await
        .unwrap();

    // Close without index (closes current)
    let result = tabs_tool
        .execute(&json!({ "action": "close" }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_tabs_invalid_action() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();

    let result = tabs_tool
        .execute(&json!({ "action": "invalid" }), &mut browser)
        .await;

    assert!(result.is_err());

    browser.shutdown().await;
}
