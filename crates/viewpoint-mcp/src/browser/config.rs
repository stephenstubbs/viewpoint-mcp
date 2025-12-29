//! Browser configuration types

use std::path::PathBuf;

/// Browser configuration
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

/// Viewport size configuration
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

/// Proxy configuration for browser contexts
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
