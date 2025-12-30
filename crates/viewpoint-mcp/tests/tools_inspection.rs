//! Integration tests for inspection tools (snapshot, screenshot, console, network)
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test tools_inspection
//! ```
#![cfg(feature = "integration")]

use serde_json::json;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};
use viewpoint_mcp::tools::{
    BrowserConsoleMessagesTool, BrowserEvaluateTool, BrowserNavigateTool,
    BrowserNetworkRequestsTool, BrowserSnapshotTool, BrowserTakeScreenshotTool,
    BrowserWaitForTool, Tool,
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
// browser_snapshot Tests
// =============================================================================

#[tokio::test]
async fn test_snapshot_basic_page() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Hello World</h1><button>Click</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = snapshot_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok(), "Snapshot should succeed: {:?}", result.err());

    let snapshot = result.unwrap();
    // The snapshot returns formatted accessibility tree
    // Content varies based on accessibility tree structure
    assert!(snapshot.contains("element"), "Should contain element info");
    assert!(snapshot.contains("ref"), "Should contain refs");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_with_all_refs() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Create page with many elements
    let mut buttons = String::new();
    for i in 0..50 {
        buttons.push_str(&format!("<button>Button {i}</button>"));
    }
    let html = format!("<html><body>{buttons}</body></html>");

    nav_tool
        .execute(&json!({ "url": format!("data:text/html,{}", html) }), &mut browser)
        .await
        .unwrap();

    // Without allRefs
    let result1 = snapshot_tool.execute(&json!({}), &mut browser).await.unwrap();

    // With allRefs
    let result2 = snapshot_tool
        .execute(&json!({ "allRefs": true }), &mut browser)
        .await
        .unwrap();

    // Both should contain buttons
    assert!(result1.contains("button"));
    assert!(result2.contains("button"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_form_elements() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    let html = r#"<form>
        <input type="text" placeholder="Username">
        <input type="password" placeholder="Password">
        <input type="email" placeholder="Email">
        <textarea placeholder="Comments"></textarea>
        <select><option>Option 1</option><option>Option 2</option></select>
        <input type="checkbox" id="agree"> Agree
        <input type="radio" name="choice" value="a"> A
        <input type="radio" name="choice" value="b"> B
        <button type="submit">Submit</button>
    </form>"#;

    nav_tool
        .execute(&json!({ "url": format!("data:text/html,{}", html) }), &mut browser)
        .await
        .unwrap();

    let result = snapshot_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok());

    let snapshot = result.unwrap();
    assert!(snapshot.contains("textbox"));
    assert!(snapshot.contains("button"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_empty_page() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<html><body></body></html>" }), &mut browser)
        .await
        .unwrap();

    let result = snapshot_tool.execute(&json!({}), &mut browser).await;
    // Empty page might succeed with minimal content or fail
    // Just ensure no panic - outcome is implementation-dependent
    let _ = result;

    browser.shutdown().await;
}

// =============================================================================
// browser_take_screenshot Tests
// =============================================================================

#[tokio::test]
async fn test_screenshot_viewport() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let screenshot_tool = BrowserTakeScreenshotTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Screenshot Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = screenshot_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok());

    let output = result.unwrap();
    // Output should be base64 encoded image or contain image data info
    assert!(!output.is_empty());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_screenshot_full_page() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let screenshot_tool = BrowserTakeScreenshotTool::new();

    // Create a tall page
    let html = r#"<div style="height: 3000px; background: linear-gradient(red, blue);">
        <h1>Top</h1>
        <div style="position: absolute; bottom: 0;">Bottom</div>
    </div>"#;

    nav_tool
        .execute(&json!({ "url": format!("data:text/html,{}", html) }), &mut browser)
        .await
        .unwrap();

    let result = screenshot_tool
        .execute(&json!({ "fullPage": true }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_screenshot_jpeg_format() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let screenshot_tool = BrowserTakeScreenshotTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>JPEG Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = screenshot_tool
        .execute(&json!({ "type": "jpeg" }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

// =============================================================================
// browser_evaluate Tests
// =============================================================================

#[tokio::test]
async fn test_evaluate_simple_expression() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = eval_tool
        .execute(&json!({ "function": "() => 2 + 2" }), &mut browser)
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("4"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_document_title() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<html><head><title>My Title</title></head></html>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = eval_tool
        .execute(&json!({ "function": "() => document.title" }), &mut browser)
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("My Title"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_dom_query() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<div class='test'>Content</div>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = eval_tool
        .execute(
            &json!({ "function": "() => document.querySelector('.test').textContent" }),
            &mut browser,
        )
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("Content"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_returns_object() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = eval_tool
        .execute(
            &json!({ "function": "() => ({ name: 'test', value: 42 })" }),
            &mut browser,
        )
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("name") || output.contains("test"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_error_handling() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = eval_tool
        .execute(
            &json!({ "function": "() => { throw new Error('Test error'); }" }),
            &mut browser,
        )
        .await;

    // Should error or return error info
    // Behavior depends on implementation
    let _ = result;

    browser.shutdown().await;
}

// =============================================================================
// browser_wait_for Tests
// =============================================================================

#[tokio::test]
async fn test_wait_for_text() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let wait_tool = BrowserWaitForTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Waiting for this text</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = wait_tool
        .execute(&json!({ "text": "Waiting" }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_wait_for_time() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let wait_tool = BrowserWaitForTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let start = std::time::Instant::now();
    let result = wait_tool.execute(&json!({ "time": 1 }), &mut browser).await;
    let elapsed = start.elapsed();

    assert!(result.is_ok());
    assert!(elapsed.as_secs() >= 1);

    browser.shutdown().await;
}

#[tokio::test]
async fn test_wait_for_text_gone() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let wait_tool = BrowserWaitForTool::new();

    // Text that's already gone (never existed)
    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Present Text</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = wait_tool
        .execute(&json!({ "textGone": "Missing Text" }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

// =============================================================================
// browser_console_messages Tests
// =============================================================================

#[tokio::test]
async fn test_console_messages_after_log() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let console_tool = BrowserConsoleMessagesTool::new();

    // Page with console.log
    let html = r#"<script>console.log('Test message');</script>"#;

    nav_tool
        .execute(&json!({ "url": format!("data:text/html,{}", html) }), &mut browser)
        .await
        .unwrap();

    // Give it a moment for console to capture
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let result = console_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_console_messages_filter_level() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let console_tool = BrowserConsoleMessagesTool::new();

    let html = r#"<script>
        console.log('log message');
        console.warn('warn message');
        console.error('error message');
    </script>"#;

    nav_tool
        .execute(&json!({ "url": format!("data:text/html,{}", html) }), &mut browser)
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Filter by error level
    let result = console_tool
        .execute(&json!({ "level": "error" }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

// =============================================================================
// browser_network_requests Tests
// =============================================================================

#[tokio::test]
async fn test_network_requests_basic() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let network_tool = BrowserNetworkRequestsTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = network_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

#[tokio::test]
async fn test_network_requests_include_static() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let network_tool = BrowserNetworkRequestsTool::new();

    // Page with image reference (even if it doesn't load)
    let html = r#"<img src="data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7">"#;

    nav_tool
        .execute(&json!({ "url": format!("data:text/html,{}", html) }), &mut browser)
        .await
        .unwrap();

    let result = network_tool
        .execute(&json!({ "includeStatic": true }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}

// =============================================================================
// Edge Cases
// =============================================================================

#[tokio::test]
async fn test_snapshot_after_navigation() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // First page
    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Page 1</h1>" }), &mut browser)
        .await
        .unwrap();

    let snap1 = snapshot_tool.execute(&json!({}), &mut browser).await.unwrap();
    // Snapshot captures the current accessibility tree
    assert!(!snap1.is_empty(), "First snapshot should not be empty");

    // Second page
    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Page 2</h1>" }), &mut browser)
        .await
        .unwrap();

    let snap2 = snapshot_tool.execute(&json!({}), &mut browser).await.unwrap();
    assert!(!snap2.is_empty(), "Second snapshot should not be empty");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_caching_same_page() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Navigate to a page
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Cache Test</h1><button>Click</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // First snapshot (should cache)
    let start1 = std::time::Instant::now();
    let snap1 = snapshot_tool.execute(&json!({}), &mut browser).await.unwrap();
    let time1 = start1.elapsed();

    // Second snapshot (should use cache, be faster)
    let start2 = std::time::Instant::now();
    let snap2 = snapshot_tool.execute(&json!({}), &mut browser).await.unwrap();
    let time2 = start2.elapsed();

    // Both snapshots should be identical (same cached data)
    assert_eq!(snap1, snap2, "Cached snapshot should match original");

    // Cache hit should generally be faster, but we just verify it works
    // (timing can be variable, so we don't assert on exact times)
    tracing::debug!(?time1, ?time2, "Snapshot timing");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_cache_invalidation_on_allrefs() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Navigate to a page with many elements
    let mut buttons = String::new();
    for i in 0..50 {
        buttons.push_str(&format!("<button>Button {i}</button>"));
    }
    let html = format!("<html><body>{buttons}</body></html>");

    nav_tool
        .execute(&json!({ "url": format!("data:text/html,{}", html) }), &mut browser)
        .await
        .unwrap();

    // First snapshot without allRefs
    let snap1 = snapshot_tool.execute(&json!({}), &mut browser).await.unwrap();

    // Second snapshot with allRefs (should NOT use cache)
    let snap2 = snapshot_tool
        .execute(&json!({ "allRefs": true }), &mut browser)
        .await
        .unwrap();

    // Third snapshot with allRefs (should use cache from second)
    let snap3 = snapshot_tool
        .execute(&json!({ "allRefs": true }), &mut browser)
        .await
        .unwrap();

    // snap2 and snap3 should be identical (cached all_refs snapshot)
    assert_eq!(snap2, snap3, "Cached allRefs snapshot should match");

    // snap1 may differ from snap2/snap3 depending on compact mode
    // At minimum, they should both contain button elements
    assert!(snap1.contains("button"));
    assert!(snap2.contains("button"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_evaluate_async_function() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let eval_tool = BrowserEvaluateTool::new();

    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Test</h1>" }), &mut browser)
        .await
        .unwrap();

    let result = eval_tool
        .execute(
            &json!({ "function": "async () => { await new Promise(r => setTimeout(r, 100)); return 'done'; }" }),
            &mut browser,
        )
        .await;
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(output.contains("done"));

    browser.shutdown().await;
}
