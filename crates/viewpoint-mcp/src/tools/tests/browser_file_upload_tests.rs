//! Tests for `browser_file_upload` tool

use crate::tools::Tool;
use crate::tools::browser_file_upload::{BrowserFileUploadInput, BrowserFileUploadTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserFileUploadTool::new();

    assert_eq!(tool.name(), "browser_file_upload");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    // paths is optional, so no required fields
    assert!(schema.get("required").is_none());
}

#[test]
fn test_input_parsing_with_paths() {
    let input: BrowserFileUploadInput = serde_json::from_value(json!({
        "paths": ["/path/to/file1.txt", "/path/to/file2.pdf"]
    }))
    .unwrap();

    assert_eq!(input.paths.len(), 2);
    assert_eq!(input.paths[0], "/path/to/file1.txt");
    assert_eq!(input.paths[1], "/path/to/file2.pdf");
}

#[test]
fn test_input_parsing_empty_paths() {
    let input: BrowserFileUploadInput = serde_json::from_value(json!({
        "paths": []
    }))
    .unwrap();

    assert!(input.paths.is_empty());
}

#[test]
fn test_input_parsing_no_paths() {
    let input: BrowserFileUploadInput = serde_json::from_value(json!({})).unwrap();

    assert!(input.paths.is_empty());
}

#[test]
fn test_single_file() {
    let input: BrowserFileUploadInput = serde_json::from_value(json!({
        "paths": ["/home/user/documents/report.pdf"]
    }))
    .unwrap();

    assert_eq!(input.paths.len(), 1);
    assert_eq!(input.paths[0], "/home/user/documents/report.pdf");
}
