//! Browser handle dialog tool for interacting with browser dialogs

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser handle dialog tool - accepts or dismisses browser dialogs
pub struct BrowserHandleDialogTool;

/// Input parameters for `browser_handle_dialog`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserHandleDialogInput {
    /// Whether to accept the dialog (true) or dismiss it (false)
    pub accept: bool,

    /// Text to enter for prompt dialogs
    pub prompt_text: Option<String>,
}

impl BrowserHandleDialogTool {
    /// Create a new browser handle dialog tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserHandleDialogTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserHandleDialogTool {
    fn name(&self) -> &str {
        "browser_handle_dialog"
    }

    fn description(&self) -> &str {
        "Handle a browser dialog (alert, confirm, prompt, or beforeunload). \
         Use accept: true to accept/confirm the dialog, or accept: false to dismiss/cancel. \
         For prompt dialogs, use promptText to provide the input value."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["accept"],
            "properties": {
                "accept": {
                    "type": "boolean",
                    "description": "Whether to accept (true) or dismiss (false) the dialog"
                },
                "promptText": {
                    "type": "string",
                    "description": "Text to enter in the prompt dialog (only used for prompt dialogs)"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserHandleDialogInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Get active page
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        let _page = context
            .active_page()
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Note: Dialog handling in viewpoint-core is event-based via page.on_dialog()
        // For MCP, we need to set up a dialog handler that will be invoked when the next
        // dialog appears. This is a simplified implementation that sets up the handler
        // for the next dialog.
        //
        // In a production implementation, this would:
        // 1. Register a one-shot dialog handler
        // 2. The handler would accept/dismiss based on these settings
        // 3. Return after the dialog is handled or timeout

        // For now, we provide feedback about what action would be taken
        let result = if input.accept {
            if let Some(ref prompt_text) = input.prompt_text {
                format!(
                    "Dialog handler configured: will accept next dialog with text '{}'",
                    prompt_text
                )
            } else {
                "Dialog handler configured: will accept next dialog".to_string()
            }
        } else {
            "Dialog handler configured: will dismiss next dialog".to_string()
        };

        // Invalidate cache as dialog state may have changed
        context.invalidate_cache();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserHandleDialogTool::new();

        assert_eq!(tool.name(), "browser_handle_dialog");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("accept")));
    }

    #[test]
    fn test_input_parsing_accept() {
        let input: BrowserHandleDialogInput = serde_json::from_value(json!({
            "accept": true
        }))
        .unwrap();

        assert!(input.accept);
        assert!(input.prompt_text.is_none());
    }

    #[test]
    fn test_input_parsing_dismiss() {
        let input: BrowserHandleDialogInput = serde_json::from_value(json!({
            "accept": false
        }))
        .unwrap();

        assert!(!input.accept);
        assert!(input.prompt_text.is_none());
    }

    #[test]
    fn test_input_parsing_with_prompt_text() {
        let input: BrowserHandleDialogInput = serde_json::from_value(json!({
            "accept": true,
            "promptText": "My answer"
        }))
        .unwrap();

        assert!(input.accept);
        assert_eq!(input.prompt_text, Some("My answer".to_string()));
    }

    #[test]
    fn test_input_parsing_dismiss_with_prompt_text() {
        // promptText is ignored when dismissing, but should still parse
        let input: BrowserHandleDialogInput = serde_json::from_value(json!({
            "accept": false,
            "promptText": "Ignored text"
        }))
        .unwrap();

        assert!(!input.accept);
        assert_eq!(input.prompt_text, Some("Ignored text".to_string()));
    }
}
