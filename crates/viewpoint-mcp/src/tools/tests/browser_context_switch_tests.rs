//! Tests for `browser_context_switch` tool

use crate::tools::Tool;
use crate::tools::browser_context_switch::{BrowserContextSwitchInput, BrowserContextSwitchTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserContextSwitchTool::new();

    assert_eq!(tool.name(), "browser_context_switch");
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
    let input: BrowserContextSwitchInput = serde_json::from_value(json!({
        "name": "my-context"
    }))
    .unwrap();

    assert_eq!(input.name, "my-context");
}

#[test]
fn test_input_parsing_default_context() {
    let input: BrowserContextSwitchInput = serde_json::from_value(json!({
        "name": "default"
    }))
    .unwrap();

    assert_eq!(input.name, "default");
}
