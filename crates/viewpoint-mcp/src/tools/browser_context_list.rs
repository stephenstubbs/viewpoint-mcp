//! Browser context list tool for listing all browser contexts

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolOutput, ToolResult};
use crate::browser::BrowserState;

/// Browser context list tool - lists all browser contexts
pub struct BrowserContextListTool;

/// Input parameters for `browser_context_list`
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserContextListInput {
    // No required inputs - this is intentionally empty
    // but we keep the struct for consistency with other tools
}

impl BrowserContextListTool {
    /// Create a new browser context list tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserContextListTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserContextListTool {
    fn name(&self) -> &'static str {
        "browser_context_list"
    }

    fn description(&self) -> &'static str {
        "List all browser contexts with their details including name, active status, \
         page count, current URL, and proxy configuration."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input (even though it's empty, validate it's an object)
        let _input: BrowserContextListInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Get active context name
        let active_context_name = browser.active_context_name().to_string();

        // List all contexts with dynamically fetched URLs
        let contexts = browser.list_contexts_with_urls().await;

        if contexts.is_empty() {
            return Ok(ToolOutput::text("No browser contexts available"));
        }

        // Build context info list
        let context_infos: Vec<Value> = contexts
            .iter()
            .map(|ctx| {
                json!({
                    "name": ctx.name,
                    "isActive": ctx.is_active,
                    "pageCount": ctx.page_count,
                    "currentUrl": ctx.current_url,
                    "proxy": ctx.proxy.as_ref().map(|p| json!({
                        "server": p.server,
                        "hasAuth": p.username.is_some()
                    }))
                })
            })
            .collect();

        // Return as formatted JSON
        let result = json!({
            "contexts": context_infos,
            "activeContext": active_context_name,
            "totalCount": context_infos.len()
        });

        let output = serde_json::to_string_pretty(&result)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to serialize result: {e}")))?;
        Ok(ToolOutput::text(output))
    }
}
