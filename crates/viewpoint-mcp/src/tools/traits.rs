//! Tool trait definitions

use ::async_trait::async_trait;
use serde_json::Value;

use super::ToolError;
use crate::browser::BrowserState;

/// Result type for tool execution
pub type ToolResult = Result<String, ToolError>;

/// Capability flags that can be enabled via command-line arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Vision capability - enables coordinate-based mouse tools
    Vision,
    /// PDF capability - enables PDF generation tools
    Pdf,
}

impl Capability {
    /// Parse a capability from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "vision" => Some(Capability::Vision),
            "pdf" => Some(Capability::Pdf),
            _ => None,
        }
    }

    /// Get the capability name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Capability::Vision => "vision",
            Capability::Pdf => "pdf",
        }
    }
}

/// Tool trait for MCP tool implementations
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool name
    fn name(&self) -> &str;

    /// Get the tool description
    fn description(&self) -> &str;

    /// Get the JSON schema for tool input
    fn input_schema(&self) -> Value;

    /// Get the capability required for this tool, if any.
    /// Tools returning `None` are always available.
    /// Tools returning `Some(cap)` are only available when that capability is enabled.
    fn required_capability(&self) -> Option<Capability> {
        None
    }

    /// Execute the tool with given arguments
    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult;
}
