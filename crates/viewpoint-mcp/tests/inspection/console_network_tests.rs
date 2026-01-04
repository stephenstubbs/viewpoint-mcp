//! Console messages and network requests integration tests

use serde_json::json;
use viewpoint_mcp::tools::{
    BrowserConsoleMessagesTool, BrowserNavigateTool, BrowserNetworkRequestsTool, Tool,
};

use super::create_browser;

#[tokio::test]
async fn test_console_messages_captures_log() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let console_tool = BrowserConsoleMessagesTool::new();

    // Page with console.log
    let html = r#"<script>console.log('Test message from console.log');</script>"#;

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    // Give it a moment for console to capture
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let result = console_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok());
    let output = result.unwrap();

    // Verify the message was captured
    assert!(
        output.contains("Test message from console.log"),
        "Should contain the logged message. Got: {output}"
    );
    assert!(
        output.contains("log"),
        "Should contain the message type. Got: {output}"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_console_messages_filter_by_error_level() {
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

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Filter by error level - should only include error
    let result = console_tool
        .execute(&json!({ "level": "error" }), &mut browser)
        .await;
    assert!(result.is_ok());
    let output = result.unwrap();

    assert!(
        output.contains("error message"),
        "Should contain error message. Got: {output}"
    );
    // With error filter, log and warn should not be included
    assert!(
        !output.contains("log message"),
        "Should NOT contain log message with error filter. Got: {output}"
    );
    assert!(
        !output.contains("warn message"),
        "Should NOT contain warn message with error filter. Got: {output}"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_console_messages_filter_by_warning_level() {
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

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Filter by warning level - should include warn and error
    let result = console_tool
        .execute(&json!({ "level": "warning" }), &mut browser)
        .await;
    assert!(result.is_ok());
    let output = result.unwrap();

    assert!(
        output.contains("warn message"),
        "Should contain warn message. Got: {output}"
    );
    assert!(
        output.contains("error message"),
        "Should contain error message. Got: {output}"
    );
    // log should not be included
    assert!(
        !output.contains("log message"),
        "Should NOT contain log message with warning filter. Got: {output}"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_console_messages_debug_includes_all() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let console_tool = BrowserConsoleMessagesTool::new();

    let html = r#"<script>
        console.debug('debug message');
        console.log('log message');
        console.info('info message');
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

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Debug level should include all messages
    let result = console_tool
        .execute(&json!({ "level": "debug" }), &mut browser)
        .await;
    assert!(result.is_ok());
    let output = result.unwrap();

    assert!(
        output.contains("debug message"),
        "Should contain debug. Got: {output}"
    );
    assert!(
        output.contains("log message"),
        "Should contain log. Got: {output}"
    );
    assert!(
        output.contains("info message"),
        "Should contain info. Got: {output}"
    );
    assert!(
        output.contains("warn message"),
        "Should contain warn. Got: {output}"
    );
    assert!(
        output.contains("error message"),
        "Should contain error. Got: {output}"
    );

    browser.shutdown().await;
}

#[tokio::test]
async fn test_console_messages_empty_page() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let console_tool = BrowserConsoleMessagesTool::new();

    // Page with no console output
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>No Console</h1>" }),
            &mut browser,
        )
        .await
        .unwrap();

    let result = console_tool.execute(&json!({}), &mut browser).await;
    assert!(result.is_ok());
    let output = result.unwrap();

    assert!(
        output.contains("No messages"),
        "Should indicate no messages. Got: {output}"
    );

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
    let html = r#"<img src="data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7">"#;

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
