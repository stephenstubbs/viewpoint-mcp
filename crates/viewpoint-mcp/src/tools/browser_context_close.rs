//! Browser context close tool for closing browser contexts

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolOutput, ToolResult};
use crate::browser::BrowserState;

/// Browser context close tool - closes a browser context
pub struct BrowserContextCloseTool;

/// Input parameters for `browser_context_close`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserContextCloseInput {
    /// Name of the context to close
    pub name: String,
}

impl BrowserContextCloseTool {
    /// Create a new browser context close tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserContextCloseTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserContextCloseTool {
    fn name(&self) -> &'static str {
        "browser_context_close"
    }

    fn description(&self) -> &'static str {
        "Close a browser context by name. Cannot close the only remaining context. \
         If the closed context was active, switches to the default context."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["name"],
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Name of the browser context to close"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        use std::fmt::Write;

        // Parse input
        let input: BrowserContextCloseInput = serde_json::from_value(args.clone())
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

        // Check if this is the only context
        let context_count = browser.list_contexts().len();
        if context_count <= 1 {
            return Err(ToolError::ExecutionFailed(
                "Cannot close the only remaining context".to_string(),
            ));
        }

        // Check if we're closing the active context
        let was_active = browser.active_context_name() == input.name;

        // Close the context
        browser
            .close_context(&input.name)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to close context: {e}")))?;

        let mut result = format!("Closed browser context '{}'", input.name);

        if was_active {
            let _ = write!(
                result,
                ". Switched to context '{}'",
                browser.active_context_name()
            );
        }

        Ok(ToolOutput::text(result))
    }
}
