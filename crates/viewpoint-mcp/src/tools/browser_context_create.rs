//! Browser context create tool for creating isolated browser contexts

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use viewpoint_core::ProxyConfig;

use super::{Tool, ToolError, ToolOutput, ToolResult};
use crate::browser::BrowserState;

/// Browser context create tool - creates a new isolated browser context
pub struct BrowserContextCreateTool;

/// Input parameters for `browser_context_create`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserContextCreateInput {
    /// Unique name for the new context
    pub name: String,

    /// Optional proxy configuration
    pub proxy: Option<ProxyInput>,

    /// Optional path to JSON file with cookies/localStorage
    pub storage_state: Option<String>,
}

/// Proxy configuration input
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyInput {
    /// Proxy server URL (e.g., "<socks5://proxy:1080>")
    pub server: String,

    /// Optional username for authentication
    pub username: Option<String>,

    /// Optional password for authentication
    pub password: Option<String>,
}

impl ProxyInput {
    /// Convert to viewpoint-core's `ProxyConfig`
    fn to_proxy_config(&self) -> ProxyConfig {
        let mut config = ProxyConfig::new(&self.server);
        if let (Some(username), Some(password)) = (&self.username, &self.password) {
            config = config.credentials(username, password);
        }
        config
    }
}

impl BrowserContextCreateTool {
    /// Create a new browser context create tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserContextCreateTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserContextCreateTool {
    fn name(&self) -> &'static str {
        "browser_context_create"
    }

    fn description(&self) -> &'static str {
        "Create a new isolated browser context with its own cookies, storage, and cache. \
         The new context becomes the active context."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["name"],
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Unique name for the new browser context"
                },
                "proxy": {
                    "type": "object",
                    "description": "Optional proxy configuration",
                    "properties": {
                        "server": {
                            "type": "string",
                            "description": "Proxy server URL (e.g., 'socks5://proxy:1080')"
                        },
                        "username": {
                            "type": "string",
                            "description": "Optional username for proxy authentication"
                        },
                        "password": {
                            "type": "string",
                            "description": "Optional password for proxy authentication"
                        }
                    },
                    "required": ["server"]
                },
                "storageState": {
                    "type": "string",
                    "description": "Path to JSON file with cookies/localStorage to restore"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        use std::fmt::Write;

        // Parse input
        let input: BrowserContextCreateInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Validate context name is not empty
        if input.name.trim().is_empty() {
            return Err(ToolError::InvalidParams(
                "Context name cannot be empty".to_string(),
            ));
        }

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Convert proxy input to ProxyConfig if provided
        let proxy_config = input.proxy.as_ref().map(ProxyInput::to_proxy_config);

        // Create the new context with optional proxy configuration
        browser
            .create_context_with_options(&input.name, proxy_config)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create context: {e}")))?;

        let mut result = format!("Created browser context '{}' and set as active", input.name);

        if let Some(ref proxy) = input.proxy {
            let _ = write!(result, " with proxy '{}'", proxy.server);
        }

        // Storage state loading not yet implemented in viewpoint-core
        if input.storage_state.is_some() {
            result.push_str(" (storage state loading not yet implemented)");
        }

        Ok(ToolOutput::text(result))
    }
}
