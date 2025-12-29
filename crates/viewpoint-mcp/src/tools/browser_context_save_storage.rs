//! Browser context save storage tool for saving context storage state

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser context save storage tool - saves context storage state to a file
pub struct BrowserContextSaveStorageTool;

/// Input parameters for `browser_context_save_storage`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserContextSaveStorageInput {
    /// Name of the context to save (defaults to active context if not provided)
    pub name: Option<String>,

    /// File path to save the storage state JSON to
    pub path: String,
}

impl BrowserContextSaveStorageTool {
    /// Create a new browser context save storage tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserContextSaveStorageTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserContextSaveStorageTool {
    fn name(&self) -> &str {
        "browser_context_save_storage"
    }

    fn description(&self) -> &str {
        "Save the storage state (cookies and localStorage) of a browser context to a JSON file. \
         This can be used to persist authentication state for later use."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["path"],
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Name of the context to save. Defaults to the active context if not provided."
                },
                "path": {
                    "type": "string",
                    "description": "File path to save the storage state JSON to"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserContextSaveStorageInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Validate path is not empty
        if input.path.trim().is_empty() {
            return Err(ToolError::InvalidParams(
                "Path cannot be empty".to_string(),
            ));
        }

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Determine which context to use
        let context_name = input
            .name
            .as_deref()
            .unwrap_or_else(|| browser.active_context_name());

        // Get the context
        let _context = browser
            .active_context()
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get context: {e}")))?;

        // TODO: Implement actual storage state saving when viewpoint-core supports it
        // This would involve:
        // 1. Getting cookies from the context via CDP
        // 2. Getting localStorage from all pages via CDP
        // 3. Writing the combined state to the specified file

        // For now, return a message indicating the feature is not yet implemented
        Ok(format!(
            "Storage state saving for context '{}' to '{}' is not yet implemented. \
             This feature requires viewpoint-core storage state API support.",
            context_name, input.path
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserContextSaveStorageTool::new();

        assert_eq!(tool.name(), "browser_context_save_storage");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("path")));
        // name is not required
        assert!(!schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("name")));
    }

    #[test]
    fn test_input_parsing_minimal() {
        let input: BrowserContextSaveStorageInput = serde_json::from_value(json!({
            "path": "/tmp/storage.json"
        }))
        .unwrap();

        assert!(input.name.is_none());
        assert_eq!(input.path, "/tmp/storage.json");
    }

    #[test]
    fn test_input_parsing_with_name() {
        let input: BrowserContextSaveStorageInput = serde_json::from_value(json!({
            "name": "auth-context",
            "path": "/path/to/auth-storage.json"
        }))
        .unwrap();

        assert_eq!(input.name, Some("auth-context".to_string()));
        assert_eq!(input.path, "/path/to/auth-storage.json");
    }

    #[test]
    fn test_input_parsing_default_context() {
        let input: BrowserContextSaveStorageInput = serde_json::from_value(json!({
            "name": "default",
            "path": "./storage.json"
        }))
        .unwrap();

        assert_eq!(input.name, Some("default".to_string()));
        assert_eq!(input.path, "./storage.json");
    }
}
