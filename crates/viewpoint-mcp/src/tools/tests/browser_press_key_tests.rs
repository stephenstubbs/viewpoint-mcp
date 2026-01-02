//! Tests for `browser_press_key` tool

use crate::tools::Tool;
use crate::tools::browser_press_key::{BrowserPressKeyInput, BrowserPressKeyTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserPressKeyTool::new();

    assert_eq!(tool.name(), "browser_press_key");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("key"))
    );
}

#[test]
fn test_input_parsing() {
    let input: BrowserPressKeyInput = serde_json::from_value(json!({
        "key": "Enter"
    }))
    .unwrap();

    assert_eq!(input.key, "Enter");
}

#[test]
fn test_input_with_modifier() {
    let input: BrowserPressKeyInput = serde_json::from_value(json!({
        "key": "Control+a"
    }))
    .unwrap();

    assert_eq!(input.key, "Control+a");
}

#[test]
fn test_input_arrow_key() {
    let input: BrowserPressKeyInput = serde_json::from_value(json!({
        "key": "ArrowLeft"
    }))
    .unwrap();

    assert_eq!(input.key, "ArrowLeft");
}
