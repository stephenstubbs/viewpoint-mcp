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
    /// Get the capability name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vision => "vision",
            Self::Pdf => "pdf",
        }
    }
}

impl std::str::FromStr for Capability {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vision" => Ok(Self::Vision),
            "pdf" => Ok(Self::Pdf),
            other => Err(format!("Unknown capability: {other}")),
        }
    }
}

/// Tool trait for MCP tool implementations
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool name
    fn name(&self) -> &'static str;

    /// Get the tool description
    fn description(&self) -> &'static str;

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
