//! Browser state management
//!
//! This module handles browser lifecycle and state management across MCP tool calls.

mod config;
mod context;
mod error;
mod state;

#[cfg(test)]
mod tests;

pub use config::{BrowserConfig, BrowserType, ProxyConfig, ViewportSize};
pub use context::ContextState;
pub use error::BrowserError;
pub use state::BrowserState;

/// Result type for browser operations
pub type Result<T> = std::result::Result<T, BrowserError>;
