//! Browser configuration types

use std::path::PathBuf;

/// Browser configuration for the MCP server.
///
/// Controls how the browser is launched and configured. By default,
/// a headed Chromium browser is launched with a standard viewport.
///
/// # Examples
///
/// ```
/// use viewpoint_mcp::browser::{BrowserConfig, BrowserType, ViewportSize};
/// use std::path::PathBuf;
///
/// // Default configuration (headed Chromium)
/// let config = BrowserConfig::default();
///
/// // Headless browser with custom viewport
/// let config = BrowserConfig {
///     headless: true,
///     viewport: Some(ViewportSize::new(1920, 1080)),
///     ..Default::default()
/// };
///
/// // Connect to existing browser via CDP
/// let config = BrowserConfig {
///     cdp_endpoint: Some("http://localhost:9222".to_string()),
///     ..Default::default()
/// };
///
/// // Persistent browser profile
/// let config = BrowserConfig {
///     user_data_dir: Some(PathBuf::from("/tmp/browser-profile")),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct BrowserConfig {
    /// Run browser in headless mode
    pub headless: bool,

    /// Browser type (always chromium for Viewpoint)
    pub browser_type: BrowserType,

    /// Viewport size
    pub viewport: Option<ViewportSize>,

    /// CDP endpoint to connect to (instead of launching browser)
    pub cdp_endpoint: Option<String>,

    /// User data directory for persistent profile
    pub user_data_dir: Option<PathBuf>,

    /// Optional capabilities (vision, pdf)
    pub capabilities: Vec<String>,
}

/// Browser type
#[derive(Debug, Clone, Default)]
pub enum BrowserType {
    /// Chromium browser
    #[default]
    Chromium,
    /// Chrome browser (same as Chromium for Viewpoint)
    Chrome,
}

/// Viewport size configuration.
///
/// Defines the browser viewport dimensions in pixels.
///
/// # Examples
///
/// ```
/// use viewpoint_mcp::browser::ViewportSize;
///
/// // Create a 1280x720 viewport
/// let viewport = ViewportSize::new(1280, 720);
///
/// // Parse from string format "WxH"
/// let viewport = ViewportSize::parse("1920x1080").unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct ViewportSize {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
}

impl ViewportSize {
    /// Create a new viewport size
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Parse viewport size from `WxH` format
    ///
    /// # Errors
    ///
    /// Returns an error if the format is invalid.
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid viewport format: {s}. Expected WxH"));
        }

        let width = parts[0]
            .parse()
            .map_err(|_| format!("Invalid width: {}", parts[0]))?;
        let height = parts[1]
            .parse()
            .map_err(|_| format!("Invalid height: {}", parts[1]))?;

        Ok(Self { width, height })
    }
}

/// Proxy configuration for browser contexts.
///
/// Enables routing browser traffic through a proxy server.
/// Supports HTTP, HTTPS, and SOCKS5 proxies.
///
/// # Examples
///
/// ```
/// use viewpoint_mcp::browser::ProxyConfig;
///
/// // Simple SOCKS5 proxy
/// let proxy = ProxyConfig::new("socks5://proxy.example.com:1080");
///
/// // Proxy with authentication
/// let proxy = ProxyConfig::new("http://proxy.example.com:8080")
///     .with_auth("username", "password");
///
/// // Proxy with bypass list
/// let proxy = ProxyConfig::new("http://proxy.example.com:8080")
///     .with_bypass("localhost,*.internal.com");
/// ```
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// Proxy server URL (e.g., `socks5://proxy:1080`)
    pub server: String,

    /// Optional username for authentication
    pub username: Option<String>,

    /// Optional password for authentication
    pub password: Option<String>,

    /// Bypass list (comma-separated patterns)
    pub bypass: Option<String>,
}

impl ProxyConfig {
    /// Create a new proxy configuration
    #[must_use]
    pub fn new(server: impl Into<String>) -> Self {
        Self {
            server: server.into(),
            username: None,
            password: None,
            bypass: None,
        }
    }

    /// Set authentication credentials
    #[must_use]
    pub fn with_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Set bypass list
    #[must_use]
    pub fn with_bypass(mut self, bypass: impl Into<String>) -> Self {
        self.bypass = Some(bypass.into());
        self
    }
}
