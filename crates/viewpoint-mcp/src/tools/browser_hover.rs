//! Browser hover tool for hovering over elements

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

/// Browser hover tool - hovers over an element
pub struct BrowserHoverTool;

/// Input parameters for `browser_hover`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserHoverInput {
    /// Element reference from snapshot
    #[serde(rename = "ref")]
    pub element_ref: String,

    /// Human-readable element description for verification
    pub element: String,
}

impl BrowserHoverTool {
    /// Create a new browser hover tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserHoverTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserHoverTool {
    fn name(&self) -> &str {
        "browser_hover"
    }

    fn description(&self) -> &str {
        "Hover the mouse over an element on the page. Useful for triggering hover states, \
         tooltips, or dropdown menus."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["ref", "element"],
            "properties": {
                "ref": {
                    "type": "string",
                    "description": "Element reference from browser_snapshot"
                },
                "element": {
                    "type": "string",
                    "description": "Human-readable description of the element"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserHoverInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Get active page
        let context = browser
            .active_context()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        let page = context
            .active_page()
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Capture current snapshot for validation
        let options = SnapshotOptions::default();
        let snapshot = AccessibilitySnapshot::capture(page, options)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        // Validate the ref exists in the snapshot
        snapshot.lookup(&input.element_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Element ref '{}': {}", input.element_ref, e))
        })?;

        // Use native ref resolution API from viewpoint 0.2.9
        let locator = page.locator_from_ref(&input.element_ref);

        // Perform the hover
        locator.hover().await.map_err(|e| {
            ToolError::ExecutionFailed(format!(
                "Failed to hover over element '{}': {}",
                input.element, e
            ))
        })?;

        Ok(format!(
            "Hovering over {} [ref={}]",
            input.element, input.element_ref
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserHoverTool::new();

        assert_eq!(tool.name(), "browser_hover");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("ref")));
    }

    #[test]
    fn test_input_parsing() {
        let input: BrowserHoverInput = serde_json::from_value(json!({
            "ref": "e1a2b3c",
            "element": "Menu item"
        }))
        .unwrap();

        assert_eq!(input.element_ref, "e1a2b3c");
        assert_eq!(input.element, "Menu item");
    }
}
