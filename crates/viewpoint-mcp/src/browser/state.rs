//! Browser state management

use std::collections::HashMap;

use viewpoint_core::Browser;
pub use viewpoint_core::ProxyConfig;

use super::config::BrowserConfig;
use super::context::ContextState;
use super::error::BrowserError;

/// Default context name
pub const DEFAULT_CONTEXT: &str = "default";

/// Browser state manager
///
/// Maintains a single browser instance with multiple named contexts.
/// Tools operate on the active context's active page.
pub struct BrowserState {
    /// Browser configuration
    config: BrowserConfig,

    /// Whether browser has been initialized
    initialized: bool,

    /// Named browser contexts
    contexts: HashMap<String, ContextState>,

    /// Currently active context name
    active_context: String,

    /// The actual Viewpoint browser instance
    browser: Option<Browser>,
}

impl std::fmt::Debug for BrowserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrowserState")
            .field("config", &self.config)
            .field("initialized", &self.initialized)
            .field("contexts", &self.contexts.keys().collect::<Vec<_>>())
            .field("active_context", &self.active_context)
            .field("browser", &self.browser.is_some())
            .finish()
    }
}

impl BrowserState {
    /// Create a new browser state manager
    #[must_use]
    pub fn new(config: BrowserConfig) -> Self {
        Self {
            config,
            initialized: false,
            contexts: HashMap::new(),
            active_context: DEFAULT_CONTEXT.to_string(),
            browser: None,
        }
    }

    /// Get the browser configuration
    #[must_use]
    pub const fn config(&self) -> &BrowserConfig {
        &self.config
    }

    /// Check if the browser has been initialized
    #[must_use]
    pub const fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get the underlying browser instance
    #[must_use]
    pub fn browser(&self) -> Option<&Browser> {
        self.browser.as_ref()
    }

    /// Initialize the browser (lazy initialization on first tool call)
    ///
    /// # Errors
    ///
    /// Returns an error if browser launch or CDP connection fails.
    pub async fn initialize(&mut self) -> super::Result<()> {
        if self.initialized {
            return Ok(());
        }

        tracing::info!(
            headless = self.config.headless,
            cdp_endpoint = ?self.config.cdp_endpoint,
            "Initializing browser"
        );

        // Launch or connect to browser
        let browser = if let Some(ref endpoint) = self.config.cdp_endpoint {
            // Use connect_over_cdp for HTTP endpoints (auto-discovers WebSocket URL)
            // or connect directly for WebSocket URLs
            if endpoint.starts_with("ws://") || endpoint.starts_with("wss://") {
                Browser::connect(endpoint)
                    .await
                    .map_err(|e| BrowserError::ConnectionFailed(e.to_string()))?
            } else {
                Browser::connect_over_cdp(endpoint)
                    .connect()
                    .await
                    .map_err(|e| BrowserError::ConnectionFailed(e.to_string()))?
            }
        } else {
            let mut launcher = Browser::launch()
                .headless(self.config.headless)
                // Prevent Chromium from opening its default window on startup.
                // We create our own context and page, so the default window is unnecessary.
                .args(["--no-startup-window"]);

            if let Some(ref user_data_dir) = self.config.user_data_dir {
                launcher = launcher.user_data_dir(user_data_dir);
            }

            launcher
                .launch()
                .await
                .map_err(|e| BrowserError::LaunchFailed(e.to_string()))?
        };

        self.browser = Some(browser);

        // Create default context (without proxy)
        self.create_context_internal(DEFAULT_CONTEXT, None).await?;

        self.initialized = true;
        Ok(())
    }

    /// Shutdown the browser
    pub async fn shutdown(&mut self) {
        if !self.initialized {
            return;
        }

        tracing::info!("Shutting down browser");

        // Close all contexts
        for (_, context) in self.contexts.drain() {
            let _ = context.close().await;
        }

        // Close browser
        if let Some(browser) = self.browser.take() {
            let _ = browser.close().await;
        }

        self.initialized = false;
    }

    /// Reset browser state after connection loss
    ///
    /// Unlike `shutdown()`, this method does NOT attempt to close connections
    /// since the browser process is dead or unreachable. It simply clears
    /// internal state to allow re-initialization on the next tool call.
    pub fn reset_on_connection_loss(&mut self) {
        tracing::warn!("Resetting browser state after connection loss");

        // Clear contexts without attempting to close them (browser is dead)
        self.contexts.clear();

        // Drop browser reference without closing (connection is lost)
        self.browser = None;

        // Reset to uninitialized state so next tool call re-launches browser
        self.initialized = false;

        // Reset active context to default
        self.active_context = DEFAULT_CONTEXT.to_string();

        tracing::info!("Browser state reset complete, ready for re-initialization");
    }

    /// Check if an error message indicates a browser connection loss
    ///
    /// Returns `true` if the error message suggests the WebSocket connection
    /// to the browser has been lost (e.g., browser crashed, killed, or timed out).
    #[must_use]
    pub fn is_connection_loss_error(error_msg: &str) -> bool {
        let patterns = [
            "WebSocket connection lost",
            "ConnectionLost",
            "connection lost",
            "connection closed",
            "WebSocket error",
            "WebSocket closed",
            "channel closed",
            "browser disconnected",
            "CDP connection",
        ];

        patterns.iter().any(|pattern| error_msg.contains(pattern))
    }

    /// Handle a potential connection loss based on an error message
    ///
    /// If the error indicates a connection loss, resets browser state and returns `true`.
    /// Otherwise, returns `false` and leaves state unchanged.
    ///
    /// This should be called when a tool execution fails to check if recovery is needed.
    pub fn handle_potential_connection_loss(&mut self, error_msg: &str) -> bool {
        if Self::is_connection_loss_error(error_msg) {
            tracing::warn!(
                error = %error_msg,
                "Detected browser connection loss, triggering state reset"
            );
            self.reset_on_connection_loss();
            true
        } else {
            false
        }
    }

    /// Get the active context
    ///
    /// # Errors
    ///
    /// Returns an error if no active context exists.
    pub fn active_context(&self) -> super::Result<&ContextState> {
        self.contexts
            .get(&self.active_context)
            .ok_or_else(|| BrowserError::ContextNotFound(self.active_context.clone()))
    }

    /// Get the active context mutably
    ///
    /// # Errors
    ///
    /// Returns an error if no active context exists.
    pub fn active_context_mut(&mut self) -> super::Result<&mut ContextState> {
        let name = self.active_context.clone();
        self.contexts
            .get_mut(&name)
            .ok_or(BrowserError::ContextNotFound(name))
    }

    /// Get a context by name
    ///
    /// # Errors
    ///
    /// Returns an error if the context doesn't exist.
    pub fn get_context(&self, name: &str) -> super::Result<&ContextState> {
        self.contexts
            .get(name)
            .ok_or_else(|| BrowserError::ContextNotFound(name.to_string()))
    }

    /// Internal helper to create a context with optional proxy configuration
    async fn create_context_internal(
        &mut self,
        name: &str,
        proxy: Option<ProxyConfig>,
    ) -> super::Result<()> {
        let browser = self.browser.as_ref().ok_or(BrowserError::NotRunning)?;

        let vp_context = if let Some(proxy_config) = proxy {
            browser
                .new_context_builder()
                .proxy(proxy_config)
                .build()
                .await
                .map_err(|e| BrowserError::LaunchFailed(e.to_string()))?
        } else {
            browser
                .new_context()
                .await
                .map_err(|e| BrowserError::LaunchFailed(e.to_string()))?
        };

        let context_state = ContextState::new(name, vp_context).await.map_err(
            |e: viewpoint_core::error::ContextError| BrowserError::LaunchFailed(e.to_string()),
        )?;

        self.contexts.insert(name.to_string(), context_state);
        self.active_context = name.to_string();

        Ok(())
    }

    /// Create a new named context
    ///
    /// # Errors
    ///
    /// Returns an error if a context with the same name already exists.
    pub async fn create_context(&mut self, name: impl Into<String>) -> super::Result<()> {
        self.create_context_with_options(name, None).await
    }

    /// Create a new named context with optional proxy configuration
    ///
    /// # Errors
    ///
    /// Returns an error if a context with the same name already exists.
    pub async fn create_context_with_options(
        &mut self,
        name: impl Into<String>,
        proxy: Option<ProxyConfig>,
    ) -> super::Result<()> {
        let name = name.into();

        if self.contexts.contains_key(&name) {
            return Err(BrowserError::ContextNotFound(format!(
                "Context '{name}' already exists"
            )));
        }

        tracing::info!(name = %name, proxy = ?proxy.as_ref().map(|p| &p.server), "Creating browser context");

        self.create_context_internal(&name, proxy).await
    }

    /// Switch to a named context
    ///
    /// # Errors
    ///
    /// Returns an error if the context doesn't exist.
    pub fn switch_context(&mut self, name: &str) -> super::Result<()> {
        if !self.contexts.contains_key(name) {
            return Err(BrowserError::ContextNotFound(name.to_string()));
        }

        self.active_context = name.to_string();
        Ok(())
    }

    /// Close a named context
    ///
    /// # Errors
    ///
    /// Returns an error if the context doesn't exist.
    pub async fn close_context(&mut self, name: &str) -> super::Result<()> {
        let context = self
            .contexts
            .remove(name)
            .ok_or_else(|| BrowserError::ContextNotFound(name.to_string()))?;

        tracing::info!(name = %name, "Closing browser context");

        context
            .close()
            .await
            .map_err(|e: viewpoint_core::error::ContextError| {
                BrowserError::LaunchFailed(e.to_string())
            })?;

        // If we closed the active context, switch to default
        if self.active_context == name {
            self.active_context = DEFAULT_CONTEXT.to_string();

            // Ensure default context exists (without proxy)
            if !self.contexts.contains_key(DEFAULT_CONTEXT) {
                self.create_context_internal(DEFAULT_CONTEXT, None).await?;
            }
        }

        Ok(())
    }

    /// List all contexts
    #[must_use]
    pub fn list_contexts(&self) -> Vec<&ContextState> {
        self.contexts.values().collect()
    }

    /// Get the active context name
    #[must_use]
    pub fn active_context_name(&self) -> &str {
        &self.active_context
    }
}
