//! Tests for `browser_drag` tool

use crate::tools::Tool;
use crate::tools::browser_drag::{BrowserDragInput, BrowserDragTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserDragTool::new();

    assert_eq!(tool.name(), "browser_drag");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    let required = schema["required"].as_array().unwrap();
    assert!(required.contains(&json!("startRef")));
    assert!(required.contains(&json!("endRef")));
}

#[test]
fn test_input_parsing() {
    let input: BrowserDragInput = serde_json::from_value(json!({
        "startRef": "e1a2b3c",
        "startElement": "Draggable item",
        "endRef": "e4d5e6f",
        "endElement": "Drop zone"
    }))
    .unwrap();

    assert_eq!(input.start_ref, "e1a2b3c");
    assert_eq!(input.end_ref, "e4d5e6f");
}
