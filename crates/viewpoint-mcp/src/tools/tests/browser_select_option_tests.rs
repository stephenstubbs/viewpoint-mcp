//! Tests for `browser_select_option` tool

use crate::tools::Tool;
use crate::tools::browser_select_option::{BrowserSelectOptionInput, BrowserSelectOptionTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserSelectOptionTool::new();

    assert_eq!(tool.name(), "browser_select_option");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("values"))
    );
}

#[test]
fn test_input_parsing() {
    let input: BrowserSelectOptionInput = serde_json::from_value(json!({
        "ref": "e1a2b3c",
        "element": "Country dropdown",
        "values": ["US"]
    }))
    .unwrap();

    assert_eq!(input.element_ref, "e1a2b3c");
    assert_eq!(input.values, vec!["US"]);
}

#[test]
fn test_multi_select() {
    let input: BrowserSelectOptionInput = serde_json::from_value(json!({
        "ref": "e1a2b3c",
        "element": "Colors select",
        "values": ["red", "blue", "green"]
    }))
    .unwrap();

    assert_eq!(input.values.len(), 3);
}
