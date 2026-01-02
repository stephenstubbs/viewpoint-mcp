//! Browser context save storage tool for saving context storage state

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

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
    fn name(&self) -> &'static str {
        "browser_context_save_storage"
    }

    fn description(&self) -> &'static str {
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
            return Err(ToolError::InvalidParams("Path cannot be empty".to_string()));
        }

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Determine which context to use and get context reference
        let context_name = input
            .name
            .as_deref()
            .unwrap_or_else(|| browser.active_context_name())
            .to_string();

        // Get the context state - if a specific name was provided, use that context
        let context_state = if let Some(ref name) = input.name {
            browser.get_context(name).map_err(|e| {
                ToolError::ExecutionFailed(format!("Context '{name}' not found: {e}"))
            })?
        } else {
            browser.active_context().map_err(|e| {
                ToolError::ExecutionFailed(format!("Failed to get active context: {e}"))
            })?
        };

        // Get the underlying BrowserContext to collect storage state
        let vp_context = context_state.context();

        // Collect storage state from the context
        let storage_state = vp_context.storage_state().await.map_err(|e| {
            ToolError::ExecutionFailed(format!("Failed to collect storage state: {e}"))
        })?;

        // Save to the specified path
        storage_state.save(&input.path).await.map_err(|e| {
            ToolError::ExecutionFailed(format!(
                "Failed to save storage state to '{}': {e}",
                input.path
            ))
        })?;

        Ok(serde_json::to_string(&json!({
            "saved": true,
            "context": context_name,
            "path": input.path,
            "message": format!("Storage state for context '{}' saved to '{}'", context_name, input.path)
        })).unwrap_or_else(|_| format!("Storage state saved to '{}'", input.path)))
    }
}
