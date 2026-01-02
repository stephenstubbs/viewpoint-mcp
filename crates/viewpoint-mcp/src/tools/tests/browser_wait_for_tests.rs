//! Tests for `browser_wait_for` tool

use crate::tools::Tool;
use crate::tools::browser_wait_for::{BrowserWaitForInput, BrowserWaitForTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserWaitForTool::new();

    assert_eq!(tool.name(), "browser_wait_for");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(schema["properties"]["text"].is_object());
    assert!(schema["properties"]["textGone"].is_object());
    assert!(schema["properties"]["time"].is_object());
}

#[test]
fn test_input_parsing_text() {
    let input: BrowserWaitForInput = serde_json::from_value(json!({
        "text": "Loading complete"
    }))
    .unwrap();

    assert_eq!(input.text, Some("Loading complete".to_string()));
    assert!(input.text_gone.is_none());
    assert!(input.time.is_none());
}

#[test]
fn test_input_parsing_text_gone() {
    let input: BrowserWaitForInput = serde_json::from_value(json!({
        "textGone": "Loading..."
    }))
    .unwrap();

    assert!(input.text.is_none());
    assert_eq!(input.text_gone, Some("Loading...".to_string()));
    assert!(input.time.is_none());
}

#[test]
fn test_input_parsing_time() {
    let input: BrowserWaitForInput = serde_json::from_value(json!({
        "time": 2.5
    }))
    .unwrap();

    assert!(input.text.is_none());
    assert!(input.text_gone.is_none());
    assert_eq!(input.time, Some(2.5));
}

#[test]
fn test_input_parsing_empty() {
    let input: BrowserWaitForInput = serde_json::from_value(json!({})).unwrap();

    assert!(input.text.is_none());
    assert!(input.text_gone.is_none());
    assert!(input.time.is_none());
}
