//! Browser navigate tool for navigating to URLs

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser navigate tool - navigates to a URL
pub struct BrowserNavigateTool;

/// Input parameters for `browser_navigate`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserNavigateInput {
    /// The URL to navigate to
    pub url: String,
}

impl BrowserNavigateTool {
    /// Create a new browser navigate tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserNavigateTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserNavigateTool {
    fn name(&self) -> &str {
        "browser_navigate"
    }

    fn description(&self) -> &str {
        "Navigate to a URL in the browser. The page will wait for the load event before returning."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["url"],
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to navigate to"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserNavigateInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Get active context
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Auto-create a new page if context has no pages
        if context.page_count() == 0 {
            context
                .new_page()
                .await
                .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create page: {e}")))?;
        }

        let page = context
            .active_page()
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Navigate to URL
        page.goto(&input.url)
            .goto()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Navigation failed: {e}")))?;

        // Update context's current URL
        context.current_url = Some(input.url.clone());

        // Invalidate cache after navigation
        context.invalidate_cache();

        Ok(format!("Navigated to {}", input.url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserNavigateTool::new();

        assert_eq!(tool.name(), "browser_navigate");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"].as_array().unwrap().contains(&json!("url")));
    }

    #[test]
    fn test_input_parsing() {
        let input: BrowserNavigateInput = serde_json::from_value(json!({
            "url": "https://example.com"
        }))
        .unwrap();

        assert_eq!(input.url, "https://example.com");
    }
}
