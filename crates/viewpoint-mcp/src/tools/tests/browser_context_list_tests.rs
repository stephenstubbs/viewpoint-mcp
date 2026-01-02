//! Tests for `browser_context_list` tool

use crate::tools::Tool;
use crate::tools::browser_context_list::{BrowserContextListInput, BrowserContextListTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserContextListTool::new();

    assert_eq!(tool.name(), "browser_context_list");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    // No required fields
    assert!(schema.get("required").is_none());
}

#[test]
fn test_input_parsing_empty() {
    let input: BrowserContextListInput = serde_json::from_value(json!({})).unwrap();
    // Just verify it parses without error
    let _ = input;
}

#[test]
fn test_input_parsing_with_extra_fields() {
    // Should ignore extra fields
    let result: Result<BrowserContextListInput, _> = serde_json::from_value(json!({
        "extraField": "ignored"
    }));
    assert!(result.is_ok());
}
