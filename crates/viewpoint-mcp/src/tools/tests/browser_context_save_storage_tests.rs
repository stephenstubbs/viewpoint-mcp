//! Tests for `browser_context_save_storage` tool

use crate::tools::Tool;
use crate::tools::browser_context_save_storage::{
    BrowserContextSaveStorageInput, BrowserContextSaveStorageTool,
};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserContextSaveStorageTool::new();

    assert_eq!(tool.name(), "browser_context_save_storage");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("path"))
    );
    // name is not required
    assert!(
        !schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("name"))
    );
}

#[test]
fn test_input_parsing_minimal() {
    let input: BrowserContextSaveStorageInput = serde_json::from_value(json!({
        "path": "/tmp/storage.json"
    }))
    .unwrap();

    assert!(input.name.is_none());
    assert_eq!(input.path, "/tmp/storage.json");
}

#[test]
fn test_input_parsing_with_name() {
    let input: BrowserContextSaveStorageInput = serde_json::from_value(json!({
        "name": "auth-context",
        "path": "/path/to/auth-storage.json"
    }))
    .unwrap();

    assert_eq!(input.name, Some("auth-context".to_string()));
    assert_eq!(input.path, "/path/to/auth-storage.json");
}

#[test]
fn test_input_parsing_default_context() {
    let input: BrowserContextSaveStorageInput = serde_json::from_value(json!({
        "name": "default",
        "path": "./storage.json"
    }))
    .unwrap();

    assert_eq!(input.name, Some("default".to_string()));
    assert_eq!(input.path, "./storage.json");
}
