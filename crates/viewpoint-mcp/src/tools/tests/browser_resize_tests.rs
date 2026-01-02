//! Tests for `browser_resize` tool

use crate::tools::Tool;
use crate::tools::browser_resize::{BrowserResizeInput, BrowserResizeTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserResizeTool::new();

    assert_eq!(tool.name(), "browser_resize");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("width"))
    );
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("height"))
    );
}

#[test]
fn test_input_parsing() {
    let input: BrowserResizeInput = serde_json::from_value(json!({
        "width": 1920,
        "height": 1080
    }))
    .unwrap();

    assert_eq!(input.width, 1920);
    assert_eq!(input.height, 1080);
}

#[test]
fn test_input_parsing_mobile_dimensions() {
    let input: BrowserResizeInput = serde_json::from_value(json!({
        "width": 375,
        "height": 812
    }))
    .unwrap();

    assert_eq!(input.width, 375);
    assert_eq!(input.height, 812);
}

#[test]
fn test_input_parsing_large_dimensions() {
    let input: BrowserResizeInput = serde_json::from_value(json!({
        "width": 3840,
        "height": 2160
    }))
    .unwrap();

    assert_eq!(input.width, 3840);
    assert_eq!(input.height, 2160);
}
