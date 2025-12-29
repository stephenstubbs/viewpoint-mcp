//! Browser context list tool for listing all browser contexts

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
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
    fn name(&self) -> &str {
        "browser_context_list"
    }

    fn description(&self) -> &str {
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

        // List all contexts
        let contexts = browser.list_contexts();

        if contexts.is_empty() {
            return Ok("No browser contexts available".to_string());
        }

        // Build context info list
        let context_infos: Vec<Value> = contexts
            .iter()
            .map(|ctx| {
                let is_active = ctx.name == active_context_name;
                json!({
                    "name": ctx.name,
                    "isActive": is_active,
                    "pageCount": ctx.page_count(),
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

        serde_json::to_string_pretty(&result)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to serialize result: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserContextListTool::new();

        assert_eq!(tool.name(), "browser_context_list");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        // No required fields
        assert!(schema.get("required").is_none());
    }

    #[test]
    fn test_input_parsing_empty() {
        let input: BrowserContextListInput = serde_json::from_value(json!({})).unwrap();
        // Just verify it parses without error
        let _ = input;
    }

    #[test]
    fn test_input_parsing_with_extra_fields() {
        // Should ignore extra fields
        let result: Result<BrowserContextListInput, _> = serde_json::from_value(json!({
            "extraField": "ignored"
        }));
        assert!(result.is_ok());
    }
}
