//! Tests for `browser_hover` tool

use crate::tools::Tool;
use crate::tools::browser_hover::{BrowserHoverInput, BrowserHoverTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserHoverTool::new();

    assert_eq!(tool.name(), "browser_hover");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("ref"))
    );
}

#[test]
fn test_input_parsing() {
    let input: BrowserHoverInput = serde_json::from_value(json!({
        "ref": "e1a2b3c",
        "element": "Menu item"
    }))
    .unwrap();

    assert_eq!(input.element_ref, "e1a2b3c");
    assert_eq!(input.element, "Menu item");
}
