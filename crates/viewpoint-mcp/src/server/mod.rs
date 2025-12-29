//! MCP Server implementation
//!
//! This module provides the core MCP protocol handling including:
//! - JSON-RPC message types
//! - MCP initialization handshake
//! - Tool listing and invocation

mod error;
pub mod protocol;
mod types;

#[cfg(test)]
mod tests;

pub use error::ServerError;
pub use protocol::McpServer;
pub use types::ServerConfig;

/// Result type for server operations
pub type Result<T> = std::result::Result<T, ServerError>;
