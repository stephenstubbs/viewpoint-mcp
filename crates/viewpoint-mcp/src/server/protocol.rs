//! MCP protocol implementation
//!
//! This module implements the Model Context Protocol (MCP) for browser automation.
//! It provides JSON-RPC request/response handling and tool dispatch.
//!
//! # Protocol Flow
//!
//! 1. Client sends `initialize` request
//! 2. Server responds with capabilities
//! 3. Client sends `initialized` notification
//! 4. Client can now call `tools/list` and `tools/call`

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;

use super::error::ServerError;
use super::types::ServerConfig;
use crate::browser::BrowserState;
use crate::tools::{Capability, ToolRegistry, register_all_tools};

/// JSON-RPC request
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,

    /// Request ID (null for notifications)
    #[serde(default)]
    pub id: Option<Value>,

    /// Method name
    pub method: String,

    /// Method parameters
    #[serde(default)]
    pub params: Value,
}

/// JSON-RPC response
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version
    pub jsonrpc: &'static str,

    /// Request ID
    pub id: Value,

    /// Result (mutually exclusive with error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,

    /// Error (mutually exclusive with result)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error
#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,

    /// Error message
    pub message: String,

    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcResponse {
    /// Create a success response
    #[must_use]
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    #[must_use]
    pub fn error(id: Value, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data: None,
            }),
        }
    }

    /// Create an error response from a `ServerError`
    #[must_use]
    pub fn from_error(id: Value, err: &ServerError) -> Self {
        Self::error(id, err.error_code(), err.to_string())
    }
}

/// MCP Server capabilities
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    /// Tool capabilities
    pub tools: ToolCapabilities,
}

/// Tool capabilities
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCapabilities {
    /// Whether tools support list changes
    pub list_changed: bool,
}

/// MCP initialization result
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    /// Protocol version
    pub protocol_version: String,

    /// Server capabilities
    pub capabilities: ServerCapabilities,

    /// Server info
    pub server_info: ServerInfo,
}

/// Server information
#[derive(Debug, Serialize)]
pub struct ServerInfo {
    /// Server name
    pub name: String,

    /// Server version
    pub version: String,
}

/// Tool definition for listing
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolDefinition {
    /// Tool name
    pub name: String,

    /// Tool description
    pub description: String,

    /// Input JSON schema
    pub input_schema: Value,
}

/// Tool call parameters
#[derive(Debug, Deserialize)]
pub struct ToolCallParams {
    /// Tool name
    pub name: String,

    /// Tool arguments
    #[serde(default)]
    pub arguments: Value,
}

/// Tool call result content
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultContent {
    /// Content type
    #[serde(rename = "type")]
    pub content_type: String,

    /// Text content
    pub text: String,
}

/// Tool call result
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallResult {
    /// Result content
    pub content: Vec<ToolResultContent>,

    /// Whether the tool execution errored
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_error: bool,
}

/// MCP Server for browser automation.
///
/// The server manages browser state and dispatches tool calls from MCP clients.
/// It supports the MCP protocol with JSON-RPC over stdio or HTTP transports.
///
/// # Examples
///
/// ```rust,ignore
/// use viewpoint_mcp::{McpServer, ServerConfig};
/// use viewpoint_mcp::transport::StdioTransport;
///
/// #[tokio::main]
/// async fn main() -> Result<(), viewpoint_mcp::transport::TransportError> {
///     let server = McpServer::new(ServerConfig::default());
///     let transport = StdioTransport::new(server);
///     transport.run().await?;
///     Ok(())
/// }
/// ```
pub struct McpServer {
    /// Server configuration
    config: ServerConfig,

    /// Tool registry
    tools: ToolRegistry,

    /// Browser state
    browser: Arc<RwLock<BrowserState>>,

    /// Whether the server has been initialized
    initialized: bool,
}

impl McpServer {
    /// Create a new MCP server
    #[must_use]
    pub fn new(config: ServerConfig) -> Self {
        let browser_config = config.browser.clone();

        // Parse capabilities from config and create registry with them enabled
        let capabilities: Vec<Capability> = config
            .capabilities
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut tools = ToolRegistry::with_capabilities(capabilities);

        // Register all browser tools
        register_all_tools(&mut tools);

        Self {
            config,
            tools,
            browser: Arc::new(RwLock::new(BrowserState::new(browser_config))),
            initialized: false,
        }
    }

    /// Handle a JSON-RPC request
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be processed.
    pub async fn handle_request(&mut self, request: &JsonRpcRequest) -> super::Result<Value> {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(&request.params).await,
            "initialized" => Ok(Value::Null), // Notification, no response needed
            "tools/list" => self.handle_tools_list().await,
            "tools/call" => self.handle_tools_call(&request.params).await,
            _ => Err(ServerError::MethodNotFound(request.method.clone())),
        }
    }

    async fn handle_initialize(&mut self, _params: &Value) -> super::Result<Value> {
        self.initialized = true;

        let result = InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ServerCapabilities {
                tools: ToolCapabilities {
                    list_changed: false,
                },
            },
            server_info: ServerInfo {
                name: self.config.name.clone(),
                version: self.config.version.clone(),
            },
        };

        Ok(serde_json::to_value(result)?)
    }

    async fn handle_tools_list(&self) -> super::Result<Value> {
        let tools: Vec<ToolDefinition> = self
            .tools
            .list()
            .iter()
            .map(|tool| ToolDefinition {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                input_schema: tool.input_schema(),
            })
            .collect();

        Ok(serde_json::to_value(serde_json::json!({ "tools": tools }))?)
    }

    async fn handle_tools_call(&self, params: &Value) -> super::Result<Value> {
        let call_params: ToolCallParams = serde_json::from_value(params.clone())
            .map_err(|e| ServerError::InvalidParams(e.to_string()))?;

        let tool = self
            .tools
            .get(&call_params.name)
            .ok_or_else(|| ServerError::MethodNotFound(call_params.name.clone()))?;

        let mut browser = self.browser.write().await;
        let result = tool.execute(&call_params.arguments, &mut browser).await;

        let call_result = match result {
            Ok(output) => ToolCallResult {
                content: vec![ToolResultContent {
                    content_type: "text".to_string(),
                    text: output,
                }],
                is_error: false,
            },
            Err(e) => {
                let error_msg = e.to_string();

                // Check for connection loss and reset state if needed
                // This allows the next tool call to re-initialize the browser
                browser.handle_potential_connection_loss(&error_msg);

                ToolCallResult {
                    content: vec![ToolResultContent {
                        content_type: "text".to_string(),
                        text: error_msg,
                    }],
                    is_error: true,
                }
            }
        };

        Ok(serde_json::to_value(call_result)?)
    }

    /// Get a reference to the browser state
    #[must_use]
    pub const fn browser_state(&self) -> &Arc<RwLock<BrowserState>> {
        &self.browser
    }

    /// Check if the server has been initialized
    #[must_use]
    pub const fn is_initialized(&self) -> bool {
        self.initialized
    }
}
