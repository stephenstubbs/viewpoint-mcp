//! Tests for browser_handle_dialog tool

use serde_json::json;
use viewpoint_mcp::tools::{BrowserHandleDialogTool, Tool};

use super::create_browser;

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
