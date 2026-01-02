//! Tests for `browser_context_create` tool

use crate::tools::Tool;
use crate::tools::browser_context_create::{BrowserContextCreateInput, BrowserContextCreateTool};
use serde_json::json;

#[test]
fn test_tool_metadata() {
    let tool = BrowserContextCreateTool::new();

    assert_eq!(tool.name(), "browser_context_create");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("name"))
    );
}

#[test]
fn test_input_parsing_minimal() {
    let input: BrowserContextCreateInput = serde_json::from_value(json!({
        "name": "test-context"
    }))
    .unwrap();

    assert_eq!(input.name, "test-context");
    assert!(input.proxy.is_none());
    assert!(input.storage_state.is_none());
}

#[test]
fn test_input_parsing_with_proxy() {
    let input: BrowserContextCreateInput = serde_json::from_value(json!({
        "name": "proxy-context",
        "proxy": {
            "server": "socks5://proxy.example.com:1080",
            "username": "user",
            "password": "pass"
        }
    }))
    .unwrap();

    assert_eq!(input.name, "proxy-context");
    let proxy = input.proxy.unwrap();
    assert_eq!(proxy.server, "socks5://proxy.example.com:1080");
    assert_eq!(proxy.username, Some("user".to_string()));
    assert_eq!(proxy.password, Some("pass".to_string()));
}

#[test]
fn test_input_parsing_with_storage_state() {
    let input: BrowserContextCreateInput = serde_json::from_value(json!({
        "name": "auth-context",
        "storageState": "/path/to/storage.json"
    }))
    .unwrap();

    assert_eq!(input.name, "auth-context");
    assert_eq!(
        input.storage_state,
        Some("/path/to/storage.json".to_string())
    );
}

#[test]
fn test_input_parsing_full() {
    let input: BrowserContextCreateInput = serde_json::from_value(json!({
        "name": "full-context",
        "proxy": {
            "server": "http://proxy:8080"
        },
        "storageState": "/tmp/state.json"
    }))
    .unwrap();

    assert_eq!(input.name, "full-context");
    assert!(input.proxy.is_some());
    assert!(input.storage_state.is_some());
}
