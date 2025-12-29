//! Browser context switch tool for switching between browser contexts

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser context switch tool - switches to an existing browser context
pub struct BrowserContextSwitchTool;

/// Input parameters for `browser_context_switch`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserContextSwitchInput {
    /// Name of the context to switch to
    pub name: String,
}

impl BrowserContextSwitchTool {
    /// Create a new browser context switch tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserContextSwitchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserContextSwitchTool {
    fn name(&self) -> &str {
        "browser_context_switch"
    }

    fn description(&self) -> &str {
        "Switch to an existing browser context by name. \
         The context must have been previously created with browser_context_create."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["name"],
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Name of the browser context to switch to"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserContextSwitchInput = serde_json::from_value(args.clone())
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

        // Get previous context name for the message
        let previous_context = browser.active_context_name().to_string();

        // Switch to the specified context
        browser
            .switch_context(&input.name)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to switch context: {e}")))?;

        Ok(format!(
            "Switched from context '{}' to '{}'",
            previous_context, input.name
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserContextSwitchTool::new();

        assert_eq!(tool.name(), "browser_context_switch");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("name")));
    }

    #[test]
    fn test_input_parsing() {
        let input: BrowserContextSwitchInput = serde_json::from_value(json!({
            "name": "my-context"
        }))
        .unwrap();

        assert_eq!(input.name, "my-context");
    }

    #[test]
    fn test_input_parsing_default_context() {
        let input: BrowserContextSwitchInput = serde_json::from_value(json!({
            "name": "default"
        }))
        .unwrap();

        assert_eq!(input.name, "default");
    }
}
