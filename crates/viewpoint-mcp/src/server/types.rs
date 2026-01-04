//! Server configuration types

use crate::browser::BrowserConfig;

/// MCP Server configuration.
///
/// Controls how the server identifies itself and what browser/capability
/// configuration to use.
///
/// # Examples
///
/// ```
/// use viewpoint_mcp::ServerConfig;
/// use viewpoint_mcp::browser::BrowserConfig;
///
/// // Default configuration
/// let config = ServerConfig::default();
///
/// // Custom configuration with headless browser
/// let config = ServerConfig {
///     browser: BrowserConfig {
///         headless: true,
///         ..Default::default()
///     },
///     capabilities: vec!["vision".to_string()],
///     ..Default::default()
/// };
/// ```
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
