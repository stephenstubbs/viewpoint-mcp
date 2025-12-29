//! Server error types

use thiserror::Error;

/// Errors that can occur during MCP server operations
#[derive(Debug, Error)]
pub enum ServerError {
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid JSON-RPC request
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Method not found
    #[error("Method not found: {0}")]
    MethodNotFound(String),

    /// Invalid parameters
    #[error("Invalid params: {0}")]
    InvalidParams(String),

    /// Internal server error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Tool execution error
    #[error("Tool error: {0}")]
    Tool(#[from] crate::tools::ToolError),

    /// Browser error
    #[error("Browser error: {0}")]
    Browser(#[from] crate::browser::BrowserError),

    /// Transport error
    #[error("Transport error: {0}")]
    Transport(#[from] crate::transport::TransportError),
}

impl ServerError {
    /// Get the JSON-RPC error code for this error
    #[must_use]
    pub const fn error_code(&self) -> i32 {
        match self {
            Self::Json(_) => -32700,           // Parse error
            Self::InvalidRequest(_) => -32600, // Invalid Request
            Self::MethodNotFound(_) => -32601, // Method not found
            Self::InvalidParams(_) => -32602,  // Invalid params
            Self::Internal(_) | Self::Tool(_) | Self::Browser(_) | Self::Transport(_) => -32603, // Internal error
        }
    }
}
