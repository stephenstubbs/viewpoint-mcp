//! Server configuration types

use crate::browser::BrowserConfig;

/// MCP Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Server name reported to clients
    pub name: String,

    /// Server version
    pub version: String,

    /// Browser configuration
    pub browser: BrowserConfig,

    /// Optional capabilities (e.g., "vision", "pdf")
    pub capabilities: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "viewpoint-mcp".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            browser: BrowserConfig::default(),
            capabilities: Vec::new(),
        }
    }
}
