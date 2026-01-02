//! Tests for `browser_context_close` tool

use crate::tools::Tool;
use crate::tools::browser_context_close::{BrowserContextCloseInput, BrowserContextCloseTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserContextCloseTool::new();

    assert_eq!(tool.name(), "browser_context_close");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("name"))
    );
}

#[test]
fn test_input_parsing() {
    let input: BrowserContextCloseInput = serde_json::from_value(json!({
        "name": "context-to-close"
    }))
    .unwrap();

    assert_eq!(input.name, "context-to-close");
}

#[test]
fn test_input_parsing_default_context() {
    let input: BrowserContextCloseInput = serde_json::from_value(json!({
        "name": "default"
    }))
    .unwrap();

    assert_eq!(input.name, "default");
}
