//! Browser select option tool for selecting dropdown options

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, ElementRef, SnapshotOptions};

/// Browser select option tool - selects options in a dropdown
pub struct BrowserSelectOptionTool;

/// Input parameters for `browser_select_option`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserSelectOptionInput {
    /// Element reference from snapshot
    #[serde(rename = "ref")]
    pub element_ref: String,

    /// Human-readable element description
    pub element: String,

    /// Values to select (can be single or multiple for multi-select)
    pub values: Vec<String>,
}

impl BrowserSelectOptionTool {
    /// Create a new browser select option tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserSelectOptionTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserSelectOptionTool {
    fn name(&self) -> &str {
        "browser_select_option"
    }

    fn description(&self) -> &str {
        "Select an option in a dropdown element. For multi-select elements, \
         multiple values can be provided."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["ref", "element", "values"],
            "properties": {
                "ref": {
                    "type": "string",
                    "description": "Element reference from browser_snapshot"
                },
                "element": {
                    "type": "string",
                    "description": "Human-readable description of the dropdown"
                },
                "values": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Values to select (by value attribute or visible text)"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserSelectOptionInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        if input.values.is_empty() {
            return Err(ToolError::InvalidParams(
                "At least one value must be provided".to_string(),
            ));
        }

        // Parse the element ref
        let element_ref =
            ElementRef::parse(&input.element_ref).map_err(ToolError::InvalidParams)?;

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
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Capture current snapshot for validation
        let options = SnapshotOptions::default();
        let snapshot = AccessibilitySnapshot::capture(page, options)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        // Validate the ref exists
        snapshot.lookup(&input.element_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Element ref '{}': {}", input.element_ref, e))
        })?;

        // Build selector from the ref
        let selector = format!("[data-ref='{}']", element_ref.hash);
        let locator = page.locator(&selector);

        // Select the options
        let select_result = if input.values.len() == 1 {
            locator.select_option(&input.values[0]).await
        } else {
            let values_slice: Vec<&str> = input.values.iter().map(String::as_str).collect();
            locator.select_options(&values_slice).await
        };

        select_result.map_err(|e| {
            ToolError::ExecutionFailed(format!(
                "Failed to select option(s) in '{}': {}",
                input.element, e
            ))
        })?;

        // Invalidate cache after interaction
        context.invalidate_cache();

        Ok(format!(
            "Selected {:?} in {} [ref={}]",
            input.values, input.element, input.element_ref
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserSelectOptionTool::new();

        assert_eq!(tool.name(), "browser_select_option");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("values")));
    }

    #[test]
    fn test_input_parsing() {
        let input: BrowserSelectOptionInput = serde_json::from_value(json!({
            "ref": "e1a2b3c",
            "element": "Country dropdown",
            "values": ["US"]
        }))
        .unwrap();

        assert_eq!(input.element_ref, "e1a2b3c");
        assert_eq!(input.values, vec!["US"]);
    }

    #[test]
    fn test_multi_select() {
        let input: BrowserSelectOptionInput = serde_json::from_value(json!({
            "ref": "e1a2b3c",
            "element": "Colors select",
            "values": ["red", "blue", "green"]
        }))
        .unwrap();

        assert_eq!(input.values.len(), 3);
    }
}
