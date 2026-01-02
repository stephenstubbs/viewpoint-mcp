//! Integration tests combining multiple management tools
//! Edge cases and workflow tests

use serde_json::json;
use viewpoint_mcp::tools::{BrowserNavigateTool, BrowserResizeTool, BrowserTabsTool, Tool};

use super::create_browser;

// =============================================================================
// Integration: Tabs + Navigation
// =============================================================================

#[tokio::test]
async fn test_tabs_navigation_workflow() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();
    let nav_tool = BrowserNavigateTool::new();

    // Tab 0: Navigate
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Tab 0</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Create Tab 1
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

    // Create Tab 2
    tabs_tool
        .execute(&json!({ "action": "new" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Tab 2</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // List should show 3 tabs
    let list = tabs_tool
        .execute(&json!({ "action": "list" }), &mut browser)
        .await
        .unwrap();

    assert!(list.contains("Tab") || list.lines().count() >= 3);

    // Switch to Tab 0
    tabs_tool
        .execute(&json!({ "action": "select", "index": 0 }), &mut browser)
        .await
        .unwrap();

    // Close Tab 2
    tabs_tool
        .execute(&json!({ "action": "close", "index": 2 }), &mut browser)
        .await
        .unwrap();

    browser.shutdown().await;
}

// =============================================================================
// Edge Cases
// =============================================================================

#[tokio::test]
async fn test_resize_after_navigation() {
    let mut browser = create_browser().await;
    let resize_tool = BrowserResizeTool::new();
    let nav_tool = BrowserNavigateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Before Resize</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    resize_tool
        .execute(&json!({ "width": 800, "height": 600 }), &mut browser)
        .await
        .unwrap();

    // Navigate again - page should still work
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>After Resize</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    browser.shutdown().await;
}

#[tokio::test]
async fn test_multiple_resizes() {
    let mut browser = create_browser().await;
    let resize_tool = BrowserResizeTool::new();

    let sizes = [
        (1920, 1080),
        (1280, 720),
        (800, 600),
        (375, 812),
        (1920, 1080),
    ];

    for (width, height) in sizes {
        let result = resize_tool
            .execute(&json!({ "width": width, "height": height }), &mut browser)
            .await;
        assert!(
            result.is_ok(),
            "Resize to {}x{} should succeed",
            width,
            height
        );
    }

    browser.shutdown().await;
}

#[tokio::test]
async fn test_rapid_tab_operations() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();

    // Rapidly create and close tabs
    for _ in 0..5 {
        tabs_tool
            .execute(&json!({ "action": "new" }), &mut browser)
            .await
            .unwrap();
    }

    for i in (1..=5).rev() {
        tabs_tool
            .execute(&json!({ "action": "close", "index": i }), &mut browser)
            .await
            .unwrap();
    }

    browser.shutdown().await;
}
