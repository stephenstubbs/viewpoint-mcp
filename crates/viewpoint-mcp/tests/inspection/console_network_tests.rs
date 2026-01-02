//! Console messages and network requests integration tests

use serde_json::json;
use viewpoint_mcp::tools::{
    BrowserConsoleMessagesTool, BrowserNavigateTool, BrowserNetworkRequestsTool, Tool,
};

use super::create_browser;

#[tokio::test]
async fn test_console_messages_after_log() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let console_tool = BrowserConsoleMessagesTool::new();

    // Page with console.log
    let html = r#"<script>console.log('Test message');</script>"#;

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
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
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
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

#[tokio::test]
async fn test_network_requests_basic() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let network_tool = BrowserNetworkRequestsTool::new();

    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Test</h1>" }),
            &mut browser,
        )
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
    let html =
        r#"<img src="data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7">"#;

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = network_tool
        .execute(&json!({ "includeStatic": true }), &mut browser)
        .await;
    assert!(result.is_ok());

    browser.shutdown().await;
}
