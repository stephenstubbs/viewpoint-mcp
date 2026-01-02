//! Tests for `browser_evaluate` tool

use crate::tools::Tool;
use crate::tools::browser_evaluate::{BrowserEvaluateInput, BrowserEvaluateTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserEvaluateTool::new();

    assert_eq!(tool.name(), "browser_evaluate");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("function"))
    );
}

#[test]
fn test_input_parsing_page_level() {
    let input: BrowserEvaluateInput = serde_json::from_value(json!({
        "function": "() => document.title"
    }))
    .unwrap();

    assert_eq!(input.function, "() => document.title");
    assert!(input.element_ref.is_none());
    assert!(input.element.is_none());
}

#[test]
fn test_input_parsing_with_element() {
    let input: BrowserEvaluateInput = serde_json::from_value(json!({
        "function": "(el) => el.textContent",
        "ref": "e1a2b3c",
        "element": "Submit button"
    }))
    .unwrap();

    assert_eq!(input.function, "(el) => el.textContent");
    assert_eq!(input.element_ref, Some("e1a2b3c".to_string()));
    assert_eq!(input.element, Some("Submit button".to_string()));
}

#[test]
fn test_input_parsing_complex_function() {
    let input: BrowserEvaluateInput = serde_json::from_value(json!({
        "function": "() => { const items = document.querySelectorAll('li'); return items.length; }"
    }))
    .unwrap();

    assert!(input.function.contains("querySelectorAll"));
}
