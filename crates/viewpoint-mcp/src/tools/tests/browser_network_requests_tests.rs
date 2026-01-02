//! Tests for `browser_network_requests` tool

use crate::tools::Tool;
use crate::tools::browser_network_requests::{
    BrowserNetworkRequestsInput, BrowserNetworkRequestsTool,
};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserNetworkRequestsTool::new();

    assert_eq!(tool.name(), "browser_network_requests");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
}

#[test]
fn test_input_defaults() {
    let input: BrowserNetworkRequestsInput = serde_json::from_value(json!({})).unwrap();

    assert!(!input.include_static);
}

#[test]
fn test_input_include_static() {
    let input: BrowserNetworkRequestsInput = serde_json::from_value(json!({
        "includeStatic": true
    }))
    .unwrap();

    assert!(input.include_static);
}
