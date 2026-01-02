//! Browser select option tool for selecting dropdown options

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

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
    fn name(&self) -> &'static str {
        "browser_select_option"
    }

    fn description(&self) -> &'static str {
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

        // Validate the ref exists in the snapshot
        snapshot.lookup(&input.element_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Element ref '{}': {}", input.element_ref, e))
        })?;

        // Use native ref resolution API from viewpoint 0.2.9
        let locator = page.locator_from_ref(&input.element_ref);

        // Select the options using the new builder API from viewpoint 0.2.10
        // Navigation waiting is automatic by default
        let select_result = if input.values.len() == 1 {
            locator.select_option().value(&input.values[0]).await
        } else {
            let values_slice: Vec<&str> = input.values.iter().map(String::as_str).collect();
            locator.select_option().values(&values_slice).await
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
