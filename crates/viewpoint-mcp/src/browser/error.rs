//! Browser error types

use thiserror::Error;

/// Errors that can occur during browser operations
#[derive(Debug, Error)]
pub enum BrowserError {
    /// Browser launch failed
    #[error("Failed to launch browser: {0}")]
    LaunchFailed(String),

    /// Connection to CDP endpoint failed
    #[error("Failed to connect to CDP endpoint: {0}")]
    ConnectionFailed(String),

    /// Browser not running
    #[error("Browser not running")]
    NotRunning,

    /// Context not found
    #[error("Context not found: {0}")]
    ContextNotFound(String),

    /// Page not found
    #[error("Page not found: {0}")]
    PageNotFound(String),

    /// Navigation failed
    #[error("Navigation failed: {0}")]
    NavigationFailed(String),

    /// JavaScript evaluation failed
    #[error("JavaScript evaluation failed: {0}")]
    EvaluationFailed(String),

    /// Timeout occurred
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
