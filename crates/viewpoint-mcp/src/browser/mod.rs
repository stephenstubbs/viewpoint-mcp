//! Browser state management
//!
//! This module handles browser lifecycle and state management across MCP tool calls.

mod config;
pub mod console;
mod context;
mod error;
mod state;

#[cfg(test)]
mod tests;

pub use config::{BrowserConfig, BrowserType, ProxyConfig, ViewportSize};
pub use console::{
    ConsoleBuffer, ConsoleLevel, SharedConsoleBuffer, StoredConsoleMessage,
    StoredConsoleMessageType, new_shared_buffer,
};
pub use context::ContextState;
pub use error::BrowserError;
pub use state::{BrowserState, ContextInfo};

/// Result type for browser operations
pub type Result<T> = std::result::Result<T, BrowserError>;
