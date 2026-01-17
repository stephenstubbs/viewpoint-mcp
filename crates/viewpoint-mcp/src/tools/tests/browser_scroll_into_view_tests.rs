//! Tests for `browser_scroll_into_view` tool

use crate::tools::Tool;
use crate::tools::browser_scroll_into_view::{
    BrowserScrollIntoViewInput, BrowserScrollIntoViewTool,
};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserScrollIntoViewTool::new();

    assert_eq!(tool.name(), "browser_scroll_into_view");
    assert!(!tool.description().is_empty());
    assert!(tool.description().contains("Scroll"));

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
            .contains(&json!("element"))
    );
}

#[test]
fn test_input_parsing() {
    let input: BrowserScrollIntoViewInput = serde_json::from_value(json!({
        "ref": "c0p0f0e1",
        "element": "Submit button"
    }))
    .unwrap();

    assert_eq!(input.element_ref, "c0p0f0e1");
    assert_eq!(input.element, "Submit button");
}

#[test]
fn test_input_missing_ref() {
    let result: Result<BrowserScrollIntoViewInput, _> = serde_json::from_value(json!({
        "element": "Some element"
    }));

    assert!(result.is_err());
}

#[test]
fn test_input_missing_element() {
    let result: Result<BrowserScrollIntoViewInput, _> = serde_json::from_value(json!({
        "ref": "c0p0f0e1"
    }));

    assert!(result.is_err());
}

#[test]
fn test_default_constructor() {
    let tool = BrowserScrollIntoViewTool::default();
    assert_eq!(tool.name(), "browser_scroll_into_view");
}
