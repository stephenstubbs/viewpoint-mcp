//! Tests for `browser_mouse_click_xy` tool

use crate::tools::Tool;
use crate::tools::browser_mouse_click_xy::{
    BrowserMouseClickXyInput, BrowserMouseClickXyTool, MouseButton,
};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserMouseClickXyTool::new();

    assert_eq!(tool.name(), "browser_mouse_click_xy");
    assert!(tool.description().contains("coordinates"));
    assert!(tool.description().contains("vision"));

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(schema["required"].as_array().unwrap().contains(&json!("x")));
    assert!(schema["required"].as_array().unwrap().contains(&json!("y")));
}

#[test]
fn test_input_parsing_minimal() {
    let input: BrowserMouseClickXyInput = serde_json::from_value(json!({
        "x": 100.0,
        "y": 200.0
    }))
    .unwrap();

    assert!((input.x - 100.0).abs() < f64::EPSILON);
    assert!((input.y - 200.0).abs() < f64::EPSILON);
    assert!(matches!(input.button, MouseButton::Left));
    assert_eq!(input.click_count, 1);
    assert!(input.element.is_none());
}

#[test]
fn test_input_parsing_with_options() {
    let input: BrowserMouseClickXyInput = serde_json::from_value(json!({
        "x": 50.5,
        "y": 75.25,
        "button": "right",
        "clickCount": 2,
        "element": "Submit button"
    }))
    .unwrap();

    assert!((input.x - 50.5).abs() < f64::EPSILON);
    assert!((input.y - 75.25).abs() < f64::EPSILON);
    assert!(matches!(input.button, MouseButton::Right));
    assert_eq!(input.click_count, 2);
    assert_eq!(input.element, Some("Submit button".to_string()));
}

#[test]
fn test_mouse_button_conversion() {
    let left: viewpoint_core::MouseButton = MouseButton::Left.into();
    assert!(matches!(left, viewpoint_core::MouseButton::Left));

    let right: viewpoint_core::MouseButton = MouseButton::Right.into();
    assert!(matches!(right, viewpoint_core::MouseButton::Right));

    let middle: viewpoint_core::MouseButton = MouseButton::Middle.into();
    assert!(matches!(middle, viewpoint_core::MouseButton::Middle));
}
