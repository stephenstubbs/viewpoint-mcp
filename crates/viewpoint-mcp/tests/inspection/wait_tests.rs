//! Wait tool integration tests

use serde_json::json;
use viewpoint_mcp::tools::{BrowserNavigateTool, BrowserWaitForTool, Tool};

use super::create_browser;

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
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1>" }),
            &mut browser,
        )
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
        .execute(
            &json!({ "url": "data:text/html,<h1>Present Text</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = wait_tool
        .execute(&json!({ "textGone": "Missing Text" }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}
