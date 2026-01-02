//! Tests for `browser_take_screenshot` tool

use crate::tools::Tool;
use crate::tools::browser_take_screenshot::{
    BrowserTakeScreenshotInput, BrowserTakeScreenshotTool, ImageFormat,
};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserTakeScreenshotTool::new();

    assert_eq!(tool.name(), "browser_take_screenshot");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
}

#[test]
fn test_input_defaults() {
    let input: BrowserTakeScreenshotInput = serde_json::from_value(json!({})).unwrap();

    assert!(input.element_ref.is_none());
    assert!(!input.full_page);
    assert!(matches!(input.image_type, ImageFormat::Png));
}

#[test]
fn test_input_full_page() {
    let input: BrowserTakeScreenshotInput = serde_json::from_value(json!({
        "fullPage": true,
        "type": "jpeg"
    }))
    .unwrap();

    assert!(input.full_page);
    assert!(matches!(input.image_type, ImageFormat::Jpeg));
}

#[test]
fn test_input_element_screenshot() {
    let input: BrowserTakeScreenshotInput = serde_json::from_value(json!({
        "ref": "e1a2b3c",
        "element": "Login form"
    }))
    .unwrap();

    assert_eq!(input.element_ref, Some("e1a2b3c".to_string()));
    assert_eq!(input.element, Some("Login form".to_string()));
}
