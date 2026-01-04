//! Browser press key tool for pressing keyboard keys

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser press key tool - presses a keyboard key
pub struct BrowserPressKeyTool;

/// Input parameters for `browser_press_key`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPressKeyInput {
    /// Key name to press (e.g., "Enter", "Tab", "Control+a", "`ArrowLeft`")
    pub key: String,
}

impl BrowserPressKeyTool {
    /// Create a new browser press key tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserPressKeyTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserPressKeyTool {
    fn name(&self) -> &'static str {
        "browser_press_key"
    }

    fn description(&self) -> &'static str {
        "Press a keyboard key. Supports key names like 'Enter', 'Tab', 'Escape', 'ArrowLeft', \
         and key combinations like 'Control+a', 'Shift+Tab', 'Alt+F4'."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["key"],
            "properties": {
                "key": {
                    "type": "string",
                    "description": "Name of the key to press or a character to generate, such as 'ArrowLeft', 'Enter', 'Tab', or 'a'. Key combinations use '+' (e.g., 'Control+a', 'Shift+Tab')."
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserPressKeyInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        if input.key.is_empty() {
            return Err(ToolError::InvalidParams("Key cannot be empty".to_string()));
        }

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Get active page
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        let page = context
            .active_page()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get active page: {e}")))?
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Press the key using the keyboard API
        page.keyboard().press(&input.key).await.map_err(|e| {
            ToolError::ExecutionFailed(format!("Failed to press key '{}': {}", input.key, e))
        })?;

        // Invalidate cache after keyboard interaction
        context.invalidate_cache();

        Ok(format!("Pressed key '{}'", input.key))
    }
}
