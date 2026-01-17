//! Tests for MCP protocol handling

use serde_json::{Value, json};

use crate::browser::BrowserConfig;
use crate::server::ServerConfig;
use crate::server::protocol::{
    ContentItem, JsonRpcRequest, JsonRpcResponse, McpServer, ToolCallResult, ToolOutput,
};

fn create_test_server() -> McpServer {
    let config = ServerConfig {
        browser: BrowserConfig::default(),
        ..Default::default()
    };
    McpServer::new(config)
}

fn create_request(method: &str, params: Value) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: method.to_string(),
        params,
    }
}

#[tokio::test]
async fn test_initialize() {
    let mut server = create_test_server();
    let request = create_request("initialize", json!({}));

    let result = server.handle_request(&request).await.unwrap();

    assert!(result.get("protocolVersion").is_some());
    assert!(result.get("capabilities").is_some());
    assert!(result.get("serverInfo").is_some());

    let server_info = &result["serverInfo"];
    assert_eq!(server_info["name"], "viewpoint-mcp");
}

#[tokio::test]
async fn test_initialized_notification() {
    let mut server = create_test_server();

    // First initialize
    let init_request = create_request("initialize", json!({}));
    server.handle_request(&init_request).await.unwrap();

    // Then send initialized notification
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: None, // Notification - no id
        method: "initialized".to_string(),
        params: json!({}),
    };

    let result = server.handle_request(&request).await.unwrap();
    assert_eq!(result, Value::Null);
}

#[tokio::test]
async fn test_tools_list_returns_core_tools() {
    let mut server = create_test_server();

    // Initialize first
    let init_request = create_request("initialize", json!({}));
    server.handle_request(&init_request).await.unwrap();

    let request = create_request("tools/list", json!({}));
    let result = server.handle_request(&request).await.unwrap();

    assert!(result.get("tools").is_some());
    let tools = result["tools"].as_array().unwrap();

    // Without any capabilities enabled, we should have 27 core tools
    // (31 total - 3 vision tools - 1 pdf tool = 27 core tools)
    // Actually: 31 total tools, 3 require Vision, 1 requires Pdf
    // So without capabilities: 31 - 3 - 1 = 27 core tools
    assert_eq!(
        tools.len(),
        27,
        "Expected 27 core tools without optional capabilities"
    );

    // Verify some expected tool names are present
    let tool_names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();

    assert!(
        tool_names.contains(&"browser_navigate"),
        "Should contain browser_navigate"
    );
    assert!(
        tool_names.contains(&"browser_click"),
        "Should contain browser_click"
    );
    assert!(
        tool_names.contains(&"browser_snapshot"),
        "Should contain browser_snapshot"
    );
    assert!(
        tool_names.contains(&"browser_tabs"),
        "Should contain browser_tabs"
    );

    // Vision tools should NOT be present without vision capability
    assert!(
        !tool_names.contains(&"browser_mouse_click_xy"),
        "Should NOT contain vision tool"
    );
    // PDF tool should NOT be present without pdf capability
    assert!(
        !tool_names.contains(&"browser_pdf_save"),
        "Should NOT contain pdf tool"
    );
}

#[tokio::test]
async fn test_tools_call_unknown_tool() {
    let mut server = create_test_server();

    // Initialize first
    let init_request = create_request("initialize", json!({}));
    server.handle_request(&init_request).await.unwrap();

    let request = create_request(
        "tools/call",
        json!({
            "name": "unknown_tool",
            "arguments": {}
        }),
    );

    let result = server.handle_request(&request).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.error_code(), -32601); // Method not found
}

#[tokio::test]
async fn test_unknown_method() {
    let mut server = create_test_server();
    let request = create_request("unknown/method", json!({}));

    let result = server.handle_request(&request).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.error_code(), -32601); // Method not found
}

#[tokio::test]
async fn test_json_rpc_response_success() {
    let response = JsonRpcResponse::success(json!(1), json!({"result": "ok"}));

    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, json!(1));
    assert!(response.result.is_some());
    assert!(response.error.is_none());
}

#[tokio::test]
async fn test_json_rpc_response_error() {
    let response = JsonRpcResponse::error(json!(1), -32600, "Invalid Request".to_string());

    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, json!(1));
    assert!(response.result.is_none());
    assert!(response.error.is_some());

    let error = response.error.unwrap();
    assert_eq!(error.code, -32600);
    assert_eq!(error.message, "Invalid Request");
}

#[tokio::test]
async fn test_server_is_initialized() {
    let mut server = create_test_server();
    assert!(!server.is_initialized());

    let request = create_request("initialize", json!({}));
    server.handle_request(&request).await.unwrap();

    assert!(server.is_initialized());
}

#[tokio::test]
async fn test_tools_registered_with_vision_capability() {
    let config = ServerConfig {
        browser: BrowserConfig::default(),
        capabilities: vec!["vision".to_string()],
        ..Default::default()
    };
    let mut server = McpServer::new(config);

    // Initialize first
    let init_request = create_request("initialize", json!({}));
    server.handle_request(&init_request).await.unwrap();

    let request = create_request("tools/list", json!({}));
    let result = server.handle_request(&request).await.unwrap();

    let tools = result["tools"].as_array().unwrap();

    // With vision enabled: 27 core + 3 vision = 30 tools
    assert_eq!(tools.len(), 30, "Expected 30 tools with vision capability");

    let tool_names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();

    // Vision tools should now be present
    assert!(tool_names.contains(&"browser_mouse_click_xy"));
    assert!(tool_names.contains(&"browser_mouse_drag_xy"));
    assert!(tool_names.contains(&"browser_mouse_move_xy"));
}

#[tokio::test]
async fn test_tools_registered_with_all_capabilities() {
    let config = ServerConfig {
        browser: BrowserConfig::default(),
        capabilities: vec!["vision".to_string(), "pdf".to_string()],
        ..Default::default()
    };
    let mut server = McpServer::new(config);

    // Initialize first
    let init_request = create_request("initialize", json!({}));
    server.handle_request(&init_request).await.unwrap();

    let request = create_request("tools/list", json!({}));
    let result = server.handle_request(&request).await.unwrap();

    let tools = result["tools"].as_array().unwrap();

    // With all capabilities: all 31 tools
    assert_eq!(tools.len(), 31, "Expected 31 tools with all capabilities");

    let tool_names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();

    // All optional tools should be present
    assert!(tool_names.contains(&"browser_mouse_click_xy"));
    assert!(tool_names.contains(&"browser_pdf_save"));
}

// =============================================================================
// ContentItem serialization tests
// =============================================================================

#[test]
fn test_content_item_text_serialization() {
    let item = ContentItem::text("Hello, world!");
    let json = serde_json::to_value(&item).unwrap();

    assert_eq!(json["type"], "text");
    assert_eq!(json["text"], "Hello, world!");
    // Ensure no extra fields
    assert!(json.get("data").is_none());
    assert!(json.get("mimeType").is_none());
}

#[test]
fn test_content_item_image_serialization() {
    let item = ContentItem::image("base64encodeddata".to_string(), "image/jpeg");
    let json = serde_json::to_value(&item).unwrap();

    assert_eq!(json["type"], "image");
    assert_eq!(json["data"], "base64encodeddata");
    assert_eq!(json["mimeType"], "image/jpeg");
    // Ensure no extra fields
    assert!(json.get("text").is_none());
}

#[test]
fn test_content_item_text_with_special_characters() {
    let item = ContentItem::text("Hello \"world\"\nNew line\tTab");
    let json = serde_json::to_value(&item).unwrap();

    assert_eq!(json["type"], "text");
    assert_eq!(json["text"], "Hello \"world\"\nNew line\tTab");
}

#[test]
fn test_content_item_image_with_png_mime() {
    let item = ContentItem::image("pngdata".to_string(), "image/png");
    let json = serde_json::to_value(&item).unwrap();

    assert_eq!(json["type"], "image");
    assert_eq!(json["mimeType"], "image/png");
}

#[test]
fn test_tool_output_text_helper() {
    let output = ToolOutput::text("Simple response");
    assert_eq!(output.content.len(), 1);

    if let ContentItem::Text { text } = &output.content[0] {
        assert_eq!(text, "Simple response");
    } else {
        panic!("Expected Text content item");
    }
}

#[test]
fn test_tool_output_with_multiple_items() {
    let output = ToolOutput::new(vec![
        ContentItem::text("Screenshot saved to path/file.png"),
        ContentItem::image("base64data".to_string(), "image/jpeg"),
    ]);

    assert_eq!(output.content.len(), 2);

    // First item should be text
    if let ContentItem::Text { text } = &output.content[0] {
        assert!(text.contains("Screenshot saved"));
    } else {
        panic!("Expected Text content item first");
    }

    // Second item should be image
    if let ContentItem::Image { data, mime_type } = &output.content[1] {
        assert_eq!(data, "base64data");
        assert_eq!(mime_type, "image/jpeg");
    } else {
        panic!("Expected Image content item second");
    }
}

#[test]
fn test_tool_call_result_serialization_success() {
    let result = ToolCallResult {
        content: vec![ContentItem::text("Operation completed")],
        is_error: false,
    };
    let json = serde_json::to_value(&result).unwrap();

    // is_error should be omitted when false (skip_serializing_if)
    assert!(json.get("isError").is_none());
    assert!(json.get("content").is_some());

    let content = json["content"].as_array().unwrap();
    assert_eq!(content.len(), 1);
    assert_eq!(content[0]["type"], "text");
    assert_eq!(content[0]["text"], "Operation completed");
}

#[test]
fn test_tool_call_result_serialization_error() {
    let result = ToolCallResult {
        content: vec![ContentItem::text("Error: something went wrong")],
        is_error: true,
    };
    let json = serde_json::to_value(&result).unwrap();

    // is_error should be present when true
    assert_eq!(json["isError"], true);

    let content = json["content"].as_array().unwrap();
    assert_eq!(content.len(), 1);
    assert_eq!(content[0]["type"], "text");
}

#[test]
fn test_tool_call_result_with_image_content() {
    let result = ToolCallResult {
        content: vec![
            ContentItem::text("Screenshot saved to ./screenshots/page.png"),
            ContentItem::image("iVBORw0KGgo=".to_string(), "image/jpeg"),
        ],
        is_error: false,
    };
    let json = serde_json::to_value(&result).unwrap();

    let content = json["content"].as_array().unwrap();
    assert_eq!(content.len(), 2);

    // Verify text item
    assert_eq!(content[0]["type"], "text");
    assert!(content[0]["text"].as_str().unwrap().contains("Screenshot"));

    // Verify image item
    assert_eq!(content[1]["type"], "image");
    assert_eq!(content[1]["data"], "iVBORw0KGgo=");
    assert_eq!(content[1]["mimeType"], "image/jpeg");
}
