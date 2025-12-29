//! Transport error types

use thiserror::Error;

/// Errors that can occur during transport operations
#[derive(Debug, Error)]
pub enum TransportError {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Connection closed
    #[error("Connection closed")]
    ConnectionClosed,

    /// Invalid message format
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Server bind error
    #[error("Failed to bind server: {0}")]
    BindFailed(String),
}
