//! Browser close tool for closing the current page

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser close tool - closes the current page
pub struct BrowserCloseTool;

/// Input parameters for `browser_close`
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserCloseInput {
    // No required parameters - closes the current active page
}

impl BrowserCloseTool {
    /// Create a new browser close tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserCloseTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserCloseTool {
    fn name(&self) -> &str {
        "browser_close"
    }

    fn description(&self) -> &str {
        "Close the current page. If there are multiple pages open, this closes only the \
         active page. The browser context remains open with any remaining pages."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input (validates the JSON structure even though there are no params)
        let _input: BrowserCloseInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Ensure browser is initialized
        if !browser.is_initialized() {
            return Err(ToolError::BrowserNotAvailable(
                "Browser is not initialized".to_string(),
            ));
        }

        // Get active context and page info before closing
        let context = browser
            .active_context()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        let page_count = context.page_count();
        let active_page_index = context.active_page;
        let current_url = context.current_url.clone();

        if page_count == 0 {
            return Err(ToolError::BrowserNotAvailable(
                "No pages to close".to_string(),
            ));
        }

        // Close the active page
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        context
            .close_page(active_page_index)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to close page: {e}")))?;

        // Invalidate cache since the page is gone
        context.invalidate_cache();

        // Build result message
        let url_info = current_url.map(|u| format!(" ({})", u)).unwrap_or_default();

        let remaining = page_count - 1;
        let remaining_info = if remaining > 0 {
            format!(", {} page(s) remaining", remaining)
        } else {
            ", no pages remaining".to_string()
        };

        Ok(format!("Closed page{url_info}{remaining_info}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserCloseTool::new();

        assert_eq!(tool.name(), "browser_close");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        // No required properties
        assert!(schema.get("required").is_none());
    }

    #[test]
    fn test_input_parsing_empty() {
        let input: BrowserCloseInput = serde_json::from_value(json!({})).unwrap();
        // Just verify it parses without error
        let _ = input;
    }

    #[test]
    fn test_input_parsing_with_extra_fields() {
        // Extra fields should be ignored
        let input: BrowserCloseInput = serde_json::from_value(json!({
            "unknownField": "value"
        }))
        .unwrap();
        let _ = input;
    }
}
