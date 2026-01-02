//! Tests for `browser_console_messages` tool

use crate::tools::Tool;
use crate::tools::browser_console_messages::{
    BrowserConsoleMessagesInput, BrowserConsoleMessagesTool, ConsoleLevel,
};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserConsoleMessagesTool::new();

    assert_eq!(tool.name(), "browser_console_messages");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
}

#[test]
fn test_input_defaults() {
    let input: BrowserConsoleMessagesInput = serde_json::from_value(json!({})).unwrap();

    assert_eq!(input.level, ConsoleLevel::Info);
}

#[test]
fn test_input_error_level() {
    let input: BrowserConsoleMessagesInput = serde_json::from_value(json!({
        "level": "error"
    }))
    .unwrap();

    assert_eq!(input.level, ConsoleLevel::Error);
}

#[test]
fn test_input_debug_level() {
    let input: BrowserConsoleMessagesInput = serde_json::from_value(json!({
        "level": "debug"
    }))
    .unwrap();

    assert_eq!(input.level, ConsoleLevel::Debug);
}
