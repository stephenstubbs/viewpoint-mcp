//! Tests for `browser_mouse_drag_xy` tool

use crate::tools::Tool;
use crate::tools::browser_mouse_drag_xy::{BrowserMouseDragXyInput, BrowserMouseDragXyTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserMouseDragXyTool::new();

    assert_eq!(tool.name(), "browser_mouse_drag_xy");
    assert!(tool.description().contains("Drag"));
    assert!(tool.description().contains("vision"));

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    let required = schema["required"].as_array().unwrap();
    assert!(required.contains(&json!("startX")));
    assert!(required.contains(&json!("startY")));
    assert!(required.contains(&json!("endX")));
    assert!(required.contains(&json!("endY")));
}

#[test]
fn test_input_parsing_minimal() {
    let input: BrowserMouseDragXyInput = serde_json::from_value(json!({
        "startX": 100.0,
        "startY": 200.0,
        "endX": 300.0,
        "endY": 400.0
    }))
    .unwrap();

    assert!((input.start_x - 100.0).abs() < f64::EPSILON);
    assert!((input.start_y - 200.0).abs() < f64::EPSILON);
    assert!((input.end_x - 300.0).abs() < f64::EPSILON);
    assert!((input.end_y - 400.0).abs() < f64::EPSILON);
    assert_eq!(input.steps, 10); // default
}

#[test]
fn test_input_parsing_with_steps() {
    let input: BrowserMouseDragXyInput = serde_json::from_value(json!({
        "startX": 10.0,
        "startY": 20.0,
        "endX": 100.0,
        "endY": 200.0,
        "steps": 25
    }))
    .unwrap();

    assert!((input.start_x - 10.0).abs() < f64::EPSILON);
    assert!((input.start_y - 20.0).abs() < f64::EPSILON);
    assert!((input.end_x - 100.0).abs() < f64::EPSILON);
    assert!((input.end_y - 200.0).abs() < f64::EPSILON);
    assert_eq!(input.steps, 25);
}
