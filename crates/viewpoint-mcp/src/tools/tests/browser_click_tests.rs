//! Tests for `browser_click` tool

use crate::tools::Tool;
use crate::tools::browser_click::{BrowserClickInput, BrowserClickTool, ClickButton};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserClickTool::new();

    assert_eq!(tool.name(), "browser_click");
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
    let input: BrowserClickInput = serde_json::from_value(json!({
        "ref": "e1a2b3c",
        "element": "Submit button"
    }))
    .unwrap();

    assert_eq!(input.element_ref, "e1a2b3c");
    assert_eq!(input.element, Some("Submit button".to_string()));
    assert!(matches!(input.button, ClickButton::Left));
    assert!(!input.double_click);
}

#[test]
fn test_input_with_options() {
    let input: BrowserClickInput = serde_json::from_value(json!({
        "ref": "clean:e1a2b3c",
        "element": "Menu item",
        "button": "right",
        "doubleClick": true,
        "modifiers": ["Control", "Shift"]
    }))
    .unwrap();

    assert_eq!(input.element_ref, "clean:e1a2b3c");
    assert!(matches!(input.button, ClickButton::Right));
    assert!(input.double_click);
    assert_eq!(input.modifiers.len(), 2);
}
