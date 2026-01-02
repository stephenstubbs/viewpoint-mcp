//! Tests for `browser_install` tool

use crate::tools::Tool;
use crate::tools::browser_install::{BrowserInstallInput, BrowserInstallTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserInstallTool::new();

    assert_eq!(tool.name(), "browser_install");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    // No required properties
    assert!(schema.get("required").is_none());
}

#[test]
fn test_input_parsing_empty() {
    let input: BrowserInstallInput = serde_json::from_value(json!({})).unwrap();
    // Just verify it parses without error
    let _ = input;
}

#[test]
fn test_input_parsing_with_extra_fields() {
    // Extra fields should be ignored
    let input: BrowserInstallInput = serde_json::from_value(json!({
        "unknownField": "value"
    }))
    .unwrap();
    let _ = input;
}
