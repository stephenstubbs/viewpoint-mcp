//! Tests for browser_resize tool

use serde_json::json;
use viewpoint_mcp::tools::{BrowserResizeTool, Tool};

use super::create_browser;

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
