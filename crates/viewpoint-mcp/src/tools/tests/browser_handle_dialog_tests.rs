//! Tests for `browser_handle_dialog` tool

use crate::tools::Tool;
use crate::tools::browser_handle_dialog::{BrowserHandleDialogInput, BrowserHandleDialogTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserHandleDialogTool::new();

    assert_eq!(tool.name(), "browser_handle_dialog");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("accept"))
    );
}

#[test]
fn test_input_parsing_accept() {
    let input: BrowserHandleDialogInput = serde_json::from_value(json!({
        "accept": true
    }))
    .unwrap();

    assert!(input.accept);
    assert!(input.prompt_text.is_none());
}

#[test]
fn test_input_parsing_dismiss() {
    let input: BrowserHandleDialogInput = serde_json::from_value(json!({
        "accept": false
    }))
    .unwrap();

    assert!(!input.accept);
    assert!(input.prompt_text.is_none());
}

#[test]
fn test_input_parsing_with_prompt_text() {
    let input: BrowserHandleDialogInput = serde_json::from_value(json!({
        "accept": true,
        "promptText": "My answer"
    }))
    .unwrap();

    assert!(input.accept);
    assert_eq!(input.prompt_text, Some("My answer".to_string()));
}

#[test]
fn test_input_parsing_dismiss_with_prompt_text() {
    // promptText is ignored when dismissing, but should still parse
    let input: BrowserHandleDialogInput = serde_json::from_value(json!({
        "accept": false,
        "promptText": "Ignored text"
    }))
    .unwrap();

    assert!(!input.accept);
    assert_eq!(input.prompt_text, Some("Ignored text".to_string()));
}
