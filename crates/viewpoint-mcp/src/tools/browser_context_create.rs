//! Browser context create tool for creating isolated browser contexts

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
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
#[allow(dead_code)] // Fields will be used when proxy support is implemented
pub struct ProxyInput {
    /// Proxy server URL (e.g., "socks5://proxy:1080")
    pub server: String,

    /// Optional username for authentication
    pub username: Option<String>,

    /// Optional password for authentication
    pub password: Option<String>,
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
    fn name(&self) -> &str {
        "browser_context_create"
    }

    fn description(&self) -> &str {
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

        // Create the new context
        browser
            .create_context(&input.name)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create context: {e}")))?;

        // TODO: Apply proxy configuration when viewpoint-core supports it
        // TODO: Load storage state from file when viewpoint-core supports it

        let mut result = format!("Created browser context '{}' and set as active", input.name);

        if input.proxy.is_some() {
            result.push_str(" (proxy configuration noted but not yet applied)");
        }

        if input.storage_state.is_some() {
            result.push_str(" (storage state loading not yet implemented)");
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserContextCreateTool::new();

        assert_eq!(tool.name(), "browser_context_create");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("name")));
    }

    #[test]
    fn test_input_parsing_minimal() {
        let input: BrowserContextCreateInput = serde_json::from_value(json!({
            "name": "test-context"
        }))
        .unwrap();

        assert_eq!(input.name, "test-context");
        assert!(input.proxy.is_none());
        assert!(input.storage_state.is_none());
    }

    #[test]
    fn test_input_parsing_with_proxy() {
        let input: BrowserContextCreateInput = serde_json::from_value(json!({
            "name": "proxy-context",
            "proxy": {
                "server": "socks5://proxy.example.com:1080",
                "username": "user",
                "password": "pass"
            }
        }))
        .unwrap();

        assert_eq!(input.name, "proxy-context");
        let proxy = input.proxy.unwrap();
        assert_eq!(proxy.server, "socks5://proxy.example.com:1080");
        assert_eq!(proxy.username, Some("user".to_string()));
        assert_eq!(proxy.password, Some("pass".to_string()));
    }

    #[test]
    fn test_input_parsing_with_storage_state() {
        let input: BrowserContextCreateInput = serde_json::from_value(json!({
            "name": "auth-context",
            "storageState": "/path/to/storage.json"
        }))
        .unwrap();

        assert_eq!(input.name, "auth-context");
        assert_eq!(
            input.storage_state,
            Some("/path/to/storage.json".to_string())
        );
    }

    #[test]
    fn test_input_parsing_full() {
        let input: BrowserContextCreateInput = serde_json::from_value(json!({
            "name": "full-context",
            "proxy": {
                "server": "http://proxy:8080"
            },
            "storageState": "/tmp/state.json"
        }))
        .unwrap();

        assert_eq!(input.name, "full-context");
        assert!(input.proxy.is_some());
        assert!(input.storage_state.is_some());
    }
}
