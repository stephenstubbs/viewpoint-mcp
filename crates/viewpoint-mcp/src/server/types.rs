//! Server configuration types

use std::path::PathBuf;

use crate::browser::BrowserConfig;

/// How screenshot images are returned in MCP responses.
///
/// Controls whether screenshot data is included inline in tool responses
/// or only saved to files.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ImageResponseMode {
    /// Save to file, return relative path in text response (default)
    #[default]
    File,
    /// Save to file AND return base64 image in response (for LLMs without file reading)
    Inline,
    /// Save to file, return confirmation only (minimal response)
    Omit,
}

impl std::str::FromStr for ImageResponseMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(Self::File),
            "inline" => Ok(Self::Inline),
            "omit" => Ok(Self::Omit),
            other => Err(format!(
                "Unknown image response mode: '{other}'. Valid values: file, inline, omit"
            )),
        }
    }
}

impl ImageResponseMode {
    /// Get the mode name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Inline => "inline",
            Self::Omit => "omit",
        }
    }
}

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

    /// Directory for saving screenshots
    pub screenshot_dir: PathBuf,

    /// How screenshot images are included in responses
    pub image_responses: ImageResponseMode,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "viewpoint-mcp".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            browser: BrowserConfig::default(),
            capabilities: Vec::new(),
            screenshot_dir: PathBuf::from(".viewpoint-mcp-screenshots"),
            image_responses: ImageResponseMode::default(),
        }
    }
}
