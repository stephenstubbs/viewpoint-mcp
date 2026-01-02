//! Tests for `browser_navigate` tool

use crate::tools::Tool;
use crate::tools::browser_navigate::{BrowserNavigateInput, BrowserNavigateTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserNavigateTool::new();

    assert_eq!(tool.name(), "browser_navigate");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("url"))
    );
}

#[test]
fn test_input_parsing() {
    let input: BrowserNavigateInput = serde_json::from_value(json!({
        "url": "https://example.com"
    }))
    .unwrap();

    assert_eq!(input.url, "https://example.com");
}
