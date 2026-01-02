//! Tests for `browser_mouse_move_xy` tool

use crate::tools::Tool;
use crate::tools::browser_mouse_move_xy::{BrowserMouseMoveXyInput, BrowserMouseMoveXyTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserMouseMoveXyTool::new();

    assert_eq!(tool.name(), "browser_mouse_move_xy");
    assert!(tool.description().contains("coordinates"));
    assert!(tool.description().contains("vision"));

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(schema["required"].as_array().unwrap().contains(&json!("x")));
    assert!(schema["required"].as_array().unwrap().contains(&json!("y")));
}

#[test]
fn test_input_parsing_minimal() {
    let input: BrowserMouseMoveXyInput = serde_json::from_value(json!({
        "x": 100.0,
        "y": 200.0
    }))
    .unwrap();

    assert!((input.x - 100.0).abs() < f64::EPSILON);
    assert!((input.y - 200.0).abs() < f64::EPSILON);
    assert_eq!(input.steps, 1);
}

#[test]
fn test_input_parsing_with_steps() {
    let input: BrowserMouseMoveXyInput = serde_json::from_value(json!({
        "x": 50.5,
        "y": 75.25,
        "steps": 10
    }))
    .unwrap();

    assert!((input.x - 50.5).abs() < f64::EPSILON);
    assert!((input.y - 75.25).abs() < f64::EPSILON);
    assert_eq!(input.steps, 10);
}
