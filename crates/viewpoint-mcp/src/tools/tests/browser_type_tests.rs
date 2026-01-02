//! Tests for `browser_type` tool

use crate::tools::Tool;
use crate::tools::browser_type::{BrowserTypeInput, BrowserTypeTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserTypeTool::new();

    assert_eq!(tool.name(), "browser_type");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("ref"))
    );
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("text"))
    );
}

#[test]
fn test_input_parsing() {
    let input: BrowserTypeInput = serde_json::from_value(json!({
        "ref": "e1a2b3c",
        "element": "Email input",
        "text": "user@example.com"
    }))
    .unwrap();

    assert_eq!(input.element_ref, "e1a2b3c");
    assert_eq!(input.element, "Email input");
    assert_eq!(input.text, "user@example.com");
    assert!(!input.slowly);
    assert!(!input.submit);
}

#[test]
fn test_input_with_options() {
    let input: BrowserTypeInput = serde_json::from_value(json!({
        "ref": "e1a2b3c",
        "element": "Search box",
        "text": "hello world",
        "slowly": true,
        "submit": true
    }))
    .unwrap();

    assert!(input.slowly);
    assert!(input.submit);
}
