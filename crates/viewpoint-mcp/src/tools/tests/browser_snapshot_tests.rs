//! Tests for `browser_snapshot` tool

use crate::tools::Tool;
use crate::tools::browser_snapshot::{BrowserSnapshotInput, BrowserSnapshotTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserSnapshotTool::new();

    assert_eq!(tool.name(), "browser_snapshot");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(schema["properties"]["allRefs"].is_object());
}

#[test]
fn test_input_parsing() {
    let input: BrowserSnapshotInput = serde_json::from_value(json!({})).unwrap();
    assert!(!input.all_refs);

    let input: BrowserSnapshotInput = serde_json::from_value(json!({
        "allRefs": true
    }))
    .unwrap();
    assert!(input.all_refs);
}
