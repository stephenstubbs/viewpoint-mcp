//! Transport layer for MCP communication
//!
//! This module provides stdio and SSE transport implementations.

mod error;
mod sse;
mod stdio;

#[cfg(test)]
mod tests;

pub use error::TransportError;
pub use sse::{SseConfig, SseTransport};
pub use stdio::StdioTransport;

/// Result type for transport operations
pub type Result<T> = std::result::Result<T, TransportError>;
