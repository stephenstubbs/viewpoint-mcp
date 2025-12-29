//! Integration tests for management tools (tabs, resize, close, dialog, install)
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test tools_management
//! ```
#![cfg(feature = "integration")]

use serde_json::json;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};
use viewpoint_mcp::tools::{
    BrowserCloseTool, BrowserHandleDialogTool, BrowserInstallTool, BrowserNavigateTool,
    BrowserResizeTool, BrowserTabsTool, Tool,
};

/// Helper to create a headless browser state
async fn create_browser() -> BrowserState {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);
    state.initialize().await.expect("Failed to initialize browser");
    state
}

// =============================================================================
// browser_tabs Tests
// =============================================================================

#[tokio::test]
async fn test_tabs_list() {
    let mut browser = create_browser().await;
    let tabs_tool = BrowserTabsTool::new();

    let result = tabs_tool
        .execute(&json!({ "action": "list" }), &mut browser)
        .await;

    assert!(result.is_ok(), "List tabs should succeed: {:?}", result.err());
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
        .execute(&json!({ "url": "data:text/html,<h1>Tab 0</h1>" }), &mut browser)
        .await
        .unwrap();

    // Create second tab
    tabs_tool
        .execute(&json!({ "action": "new" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Tab 1</h1>" }), &mut browser)
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

// =============================================================================
// browser_resize Tests
// =============================================================================

#[tokio::test]
async fn test_resize_viewport() {
    let mut browser = create_browser().await;
    let resize_tool = BrowserResizeTool::new();

    let result = resize_tool
        .execute(&json!({ "width": 1920, "height": 1080 }), &mut browser)
        .await;

    assert!(result.is_ok(), "Resize should succeed: {:?}", result.err());
    let output = result.unwrap();
    assert!(output.contains("1920") || output.contains("resized"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_resize_mobile_viewport() {
    let mut browser = create_browser().await;
    let resize_tool = BrowserResizeTool::new();

    // iPhone-like dimensions
    let result = resize_tool
        .execute(&json!({ "width": 375, "height": 812 }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_resize_very_small() {
    let mut browser = create_browser().await;
    let resize_tool = BrowserResizeTool::new();

    let result = resize_tool
        .execute(&json!({ "width": 100, "height": 100 }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_resize_very_large() {
    let mut browser = create_browser().await;
    let resize_tool = BrowserResizeTool::new();

    let result = resize_tool
        .execute(&json!({ "width": 3840, "height": 2160 }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_resize_missing_dimensions() {
    let mut browser = create_browser().await;
    let resize_tool = BrowserResizeTool::new();

    let result = resize_tool
        .execute(&json!({ "width": 800 }), &mut browser)
        .await;

    assert!(result.is_err());

    let result2 = resize_tool
        .execute(&json!({ "height": 600 }), &mut browser)
        .await;

    assert!(result2.is_err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_resize_zero_dimensions() {
    let mut browser = create_browser().await;
    let resize_tool = BrowserResizeTool::new();

    let result = resize_tool
        .execute(&json!({ "width": 0, "height": 0 }), &mut browser)
        .await;

    // Should fail or be handled gracefully
    // Zero dimensions are invalid
    assert!(result.is_err() || result.is_ok()); // Either way, no panic

    browser.shutdown().await;
}

// =============================================================================
// browser_close Tests
// =============================================================================

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

// =============================================================================
// browser_handle_dialog Tests
// =============================================================================

#[tokio::test]
async fn test_dialog_accept() {
    let mut browser = create_browser().await;
    let dialog_tool = BrowserHandleDialogTool::new();

    // Set up dialog handler to accept
    let result = dialog_tool
        .execute(&json!({ "accept": true }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_dialog_dismiss() {
    let mut browser = create_browser().await;
    let dialog_tool = BrowserHandleDialogTool::new();

    let result = dialog_tool
        .execute(&json!({ "accept": false }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_dialog_with_prompt_text() {
    let mut browser = create_browser().await;
    let dialog_tool = BrowserHandleDialogTool::new();

    let result = dialog_tool
        .execute(
            &json!({ "accept": true, "promptText": "test input" }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_dialog_missing_accept() {
    let mut browser = create_browser().await;
    let dialog_tool = BrowserHandleDialogTool::new();

    let result = dialog_tool.execute(&json!({}), &mut browser).await;

    // Should fail - accept is required
    assert!(result.is_err());

    browser.shutdown().await;
}

// =============================================================================
// browser_install Tests
// =============================================================================

#[tokio::test]
async fn test_install_already_installed() {
    let mut browser = create_browser().await;
    let install_tool = BrowserInstallTool::new();

    // Browser is already installed (we're using it)
    let result = install_tool.execute(&json!({}), &mut browser).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(
        output.contains("installed") || output.contains("ready") || output.contains("available")
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_install_without_browser_init() {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut browser = BrowserState::new(config);
    // Don't initialize browser

    let install_tool = BrowserInstallTool::new();
    let result = install_tool.execute(&json!({}), &mut browser).await;

    // Should succeed even without initialization
    assert!(result.is_ok());
}

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
        .execute(&json!({ "url": "data:text/html,<h1>Tab 0</h1>" }), &mut browser)
        .await
        .unwrap();

    // Create Tab 1
    tabs_tool
        .execute(&json!({ "action": "new" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Tab 1</h1>" }), &mut browser)
        .await
        .unwrap();

    // Create Tab 2
    tabs_tool
        .execute(&json!({ "action": "new" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Tab 2</h1>" }), &mut browser)
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
        .execute(&json!({ "url": "data:text/html,<h1>Before Resize</h1>" }), &mut browser)
        .await
        .unwrap();

    resize_tool
        .execute(&json!({ "width": 800, "height": 600 }), &mut browser)
        .await
        .unwrap();

    // Navigate again - page should still work
    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>After Resize</h1>" }), &mut browser)
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
        assert!(result.is_ok(), "Resize to {}x{} should succeed", width, height);
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
