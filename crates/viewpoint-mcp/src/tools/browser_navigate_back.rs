//! Browser navigate back tool for navigating back in history

use async_trait::async_trait;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser navigate back tool - navigates back in browser history
pub struct BrowserNavigateBackTool;

impl BrowserNavigateBackTool {
    /// Create a new browser navigate back tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserNavigateBackTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserNavigateBackTool {
    fn name(&self) -> &'static str {
        "browser_navigate_back"
    }

    fn description(&self) -> &'static str {
        "Navigate back to the previous page in the browser history."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }

    async fn execute(&self, _args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Get active context (immutable) to get page and navigate
        let url = {
            let context = browser
                .active_context()
                .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

            let page = context
                .active_page()
                .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

            // Navigate back
            page.go_back()
                .await
                .map_err(|e| ToolError::ExecutionFailed(format!("Navigation back failed: {e}")))?;

            // Get URL for response
            page.url().await.ok()
        };

        // Now get mutable context to invalidate cache and update URL
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        context.invalidate_cache();

        if let Some(url) = url {
            context.current_url = Some(url.clone());
            Ok(format!("Navigated back to {url}"))
        } else {
            Ok("Navigated back".to_string())
        }
    }
}
