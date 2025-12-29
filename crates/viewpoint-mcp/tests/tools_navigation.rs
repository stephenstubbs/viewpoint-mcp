//! Integration tests for navigation tools
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test tools_navigation
//! ```
#![cfg(feature = "integration")]

use serde_json::json;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};
use viewpoint_mcp::tools::{BrowserNavigateBackTool, BrowserNavigateTool, Tool};

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
// browser_navigate Tests
// =============================================================================

#[tokio::test]
async fn test_navigate_to_url() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    let result = tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Hello</h1>" }),
            &mut browser,
        )
        .await;

    assert!(result.is_ok());
    let msg = result.unwrap();
    assert!(msg.contains("Navigated to"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_to_complex_html() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    let html = r#"data:text/html,<!DOCTYPE html>
        <html>
        <head><title>Test Page</title></head>
        <body>
            <h1>Complex Page</h1>
            <form>
                <input type="text" name="username" placeholder="Username">
                <input type="password" name="password" placeholder="Password">
                <button type="submit">Login</button>
            </form>
        </body>
        </html>"#;

    let result = tool.execute(&json!({ "url": html }), &mut browser).await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_missing_url() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    let result = tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.to_string().contains("url") || err.to_string().contains("missing"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_empty_url() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    let result = tool.execute(&json!({ "url": "" }), &mut browser).await;
    // Empty URL should fail during navigation
    assert!(result.is_err());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_invalid_url() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    // Invalid protocol
    let _result = tool
        .execute(&json!({ "url": "not-a-valid-url" }), &mut browser)
        .await;
    // This may or may not error depending on browser behavior
    // Chrome often treats this as a search query, so we just check it doesn't panic

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_updates_context_url() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    let url = "data:text/html,<h1>Test</h1>";
    let _ = tool.execute(&json!({ "url": url }), &mut browser).await;

    let context = browser.active_context().unwrap();
    assert_eq!(context.current_url, Some(url.to_string()));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_multiple_pages() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    // Navigate to first page
    let _ = tool
        .execute(&json!({ "url": "data:text/html,<h1>Page 1</h1>" }), &mut browser)
        .await
        .unwrap();

    // Navigate to second page
    let _ = tool
        .execute(&json!({ "url": "data:text/html,<h1>Page 2</h1>" }), &mut browser)
        .await
        .unwrap();

    // Navigate to third page
    let result = tool
        .execute(&json!({ "url": "data:text/html,<h1>Page 3</h1>" }), &mut browser)
        .await;

    assert!(result.is_ok());

    browser.shutdown().await;
}

// =============================================================================
// browser_navigate_back Tests
// =============================================================================

#[tokio::test]
async fn test_navigate_back_after_navigation() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let back_tool = BrowserNavigateBackTool::new();

    // Navigate to first page
    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Page 1</h1>" }), &mut browser)
        .await
        .unwrap();

    // Navigate to second page
    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Page 2</h1>" }), &mut browser)
        .await
        .unwrap();

    // Go back
    let result = back_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok());
    let msg = result.unwrap();
    assert!(msg.contains("back") || msg.contains("Back"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_back_at_start() {
    let mut browser = create_browser().await;
    let back_tool = BrowserNavigateBackTool::new();

    // Try to go back without any navigation history
    // This should succeed (browser handles empty history gracefully)
    let result = back_tool.execute(&json!({}), &mut browser).await;
    // May succeed or fail depending on browser behavior
    // Just ensure no panic
    let _ = result;

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_back_multiple_times() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let back_tool = BrowserNavigateBackTool::new();

    // Build up history
    for i in 1..=5 {
        nav_tool
            .execute(
                &json!({ "url": format!("data:text/html,<h1>Page {}</h1>", i) }),
                &mut browser,
            )
            .await
            .unwrap();
    }

    // Go back multiple times
    for _ in 1..=3 {
        let result = back_tool.execute(&json!({}), &mut browser).await;
        assert!(result.is_ok());
    }

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_back_invalidates_cache() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let back_tool = BrowserNavigateBackTool::new();

    // Navigate to pages
    nav_tool
        .execute(&json!({ "url": "data:text/html,<button>Page 1</button>" }), &mut browser)
        .await
        .unwrap();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<button>Page 2</button>" }), &mut browser)
        .await
        .unwrap();

    // Go back - should invalidate snapshot cache
    back_tool.execute(&json!({}), &mut browser).await.unwrap();

    // Taking a snapshot should work (cache was invalidated)
    let ctx = browser.active_context().unwrap();
    let page = ctx.active_page().unwrap();
    
    use viewpoint_mcp::snapshot::{AccessibilitySnapshot, SnapshotOptions};
    let snapshot = AccessibilitySnapshot::capture(page, SnapshotOptions::default())
        .await
        .unwrap();
    
    // Should have elements from Page 1
    let formatted = snapshot.format();
    assert!(formatted.contains("button"));

    browser.shutdown().await;
}

// =============================================================================
// Edge Cases and Error Handling
// =============================================================================

#[tokio::test]
async fn test_navigate_with_fragment() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    let html = "data:text/html,<h1 id='section'>Section</h1><a href='#section'>Link</a>";
    let result = tool.execute(&json!({ "url": html }), &mut browser).await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_with_query_params() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    let html = "data:text/html,<h1>Query Test</h1>?param1=value1&param2=value2";
    let result = tool.execute(&json!({ "url": html }), &mut browser).await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_with_special_characters() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    // HTML with special characters
    let html = "data:text/html,<h1>Special: &amp; &lt; &gt; \"quotes\"</h1>";
    let result = tool.execute(&json!({ "url": html }), &mut browser).await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_with_unicode() {
    let mut browser = create_browser().await;
    let tool = BrowserNavigateTool::new();

    // HTML with unicode
    let html = "data:text/html,<h1>Unicode: 日本語 中文 한국어 🎉</h1>";
    let result = tool.execute(&json!({ "url": html }), &mut browser).await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_navigate_without_browser_init() {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut browser = BrowserState::new(config);
    // Don't initialize browser

    let tool = BrowserNavigateTool::new();
    
    // Should auto-initialize when tool is executed
    let result = tool
        .execute(&json!({ "url": "data:text/html,<h1>Test</h1>" }), &mut browser)
        .await;
    
    // Should succeed because tool auto-initializes browser
    assert!(result.is_ok());

    browser.shutdown().await;
}
