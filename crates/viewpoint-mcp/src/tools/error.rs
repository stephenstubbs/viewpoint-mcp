//! Tool error types

use thiserror::Error;

/// Errors that can occur during tool execution
#[derive(Debug, Error)]
pub enum ToolError {
    /// Invalid input parameters
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// Tool execution failed
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    /// Browser not available
    #[error("Browser not available: {0}")]
    BrowserNotAvailable(String),

    /// Element not found
    #[error("Element not found: {0}")]
    ElementNotFound(String),

    /// Timeout during execution
    #[error("Timeout: {0}")]
    Timeout(String),

    /// JSON serialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
