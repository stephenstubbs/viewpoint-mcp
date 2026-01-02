//! Tests for `browser_tabs` tool

use crate::tools::Tool;
use crate::tools::browser_tabs::{BrowserTabsInput, BrowserTabsTool, TabAction};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserTabsTool::new();

    assert_eq!(tool.name(), "browser_tabs");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("action"))
    );
}

#[test]
fn test_input_parsing_list() {
    let input: BrowserTabsInput = serde_json::from_value(json!({
        "action": "list"
    }))
    .unwrap();

    assert!(matches!(input.action, TabAction::List));
    assert!(input.index.is_none());
}

#[test]
fn test_input_parsing_new() {
    let input: BrowserTabsInput = serde_json::from_value(json!({
        "action": "new"
    }))
    .unwrap();

    assert!(matches!(input.action, TabAction::New));
}

#[test]
fn test_input_parsing_close_with_index() {
    let input: BrowserTabsInput = serde_json::from_value(json!({
        "action": "close",
        "index": 2
    }))
    .unwrap();

    assert!(matches!(input.action, TabAction::Close));
    assert_eq!(input.index, Some(2));
}

#[test]
fn test_input_parsing_close_without_index() {
    let input: BrowserTabsInput = serde_json::from_value(json!({
        "action": "close"
    }))
    .unwrap();

    assert!(matches!(input.action, TabAction::Close));
    assert!(input.index.is_none());
}

#[test]
fn test_input_parsing_select() {
    let input: BrowserTabsInput = serde_json::from_value(json!({
        "action": "select",
        "index": 1
    }))
    .unwrap();

    assert!(matches!(input.action, TabAction::Select));
    assert_eq!(input.index, Some(1));
}
