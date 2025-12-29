//! Snapshot error types

use thiserror::Error;

/// Result type for snapshot operations
pub type SnapshotResult<T> = Result<T, SnapshotError>;

/// Errors that can occur during snapshot operations
#[derive(Debug, Error)]
pub enum SnapshotError {
    /// Failed to capture accessibility tree
    #[error("Failed to capture accessibility tree: {0}")]
    CaptureError(String),

    /// Element reference not found
    #[error("Element reference '{0}' not found in snapshot")]
    RefNotFound(String),

    /// Invalid reference format
    #[error("Invalid reference format: '{0}'. Expected format: e{{hash}} or {{context}}:e{{hash}}")]
    InvalidRefFormat(String),

    /// Stale reference detected
    #[error("Stale reference: {0}")]
    StaleRef(String),

    /// Page not available
    #[error("Page not available for snapshot")]
    PageNotAvailable,

    /// Viewpoint error
    #[error("Viewpoint error: {0}")]
    ViewpointError(String),
}

impl From<viewpoint_core::error::LocatorError> for SnapshotError {
    fn from(err: viewpoint_core::error::LocatorError) -> Self {
        Self::ViewpointError(err.to_string())
    }
}
