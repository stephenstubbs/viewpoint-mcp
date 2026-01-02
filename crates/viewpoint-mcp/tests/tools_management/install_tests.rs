//! Tests for browser_install tool

use serde_json::json;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};
use viewpoint_mcp::tools::{BrowserInstallTool, Tool};

use super::create_browser;

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
