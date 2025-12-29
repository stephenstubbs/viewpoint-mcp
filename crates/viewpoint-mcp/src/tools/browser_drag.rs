//! Browser drag tool for drag and drop operations

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, ElementRef, SnapshotOptions};

/// Browser drag tool - drags from one element to another
pub struct BrowserDragTool;

/// Input parameters for `browser_drag`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserDragInput {
    /// Source element reference
    pub start_ref: String,

    /// Source element description
    pub start_element: String,

    /// Target element reference
    pub end_ref: String,

    /// Target element description
    pub end_element: String,
}

impl BrowserDragTool {
    /// Create a new browser drag tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserDragTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserDragTool {
    fn name(&self) -> &str {
        "browser_drag"
    }

    fn description(&self) -> &str {
        "Perform a drag and drop operation from one element to another."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["startRef", "startElement", "endRef", "endElement"],
            "properties": {
                "startRef": {
                    "type": "string",
                    "description": "Source element reference from browser_snapshot"
                },
                "startElement": {
                    "type": "string",
                    "description": "Human-readable description of the source element"
                },
                "endRef": {
                    "type": "string",
                    "description": "Target element reference from browser_snapshot"
                },
                "endElement": {
                    "type": "string",
                    "description": "Human-readable description of the target element"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserDragInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Parse element refs
        let start_ref = ElementRef::parse(&input.start_ref).map_err(ToolError::InvalidParams)?;
        let end_ref = ElementRef::parse(&input.end_ref).map_err(ToolError::InvalidParams)?;

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

        // Validate both refs exist
        snapshot.lookup(&input.start_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Source ref '{}': {}", input.start_ref, e))
        })?;
        snapshot.lookup(&input.end_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Target ref '{}': {}", input.end_ref, e))
        })?;

        // Build selectors
        let source_selector = format!("[data-ref='{}']", start_ref.hash);
        let target_selector = format!("[data-ref='{}']", end_ref.hash);

        // Create locators
        let source = page.locator(&source_selector);
        let target = page.locator(&target_selector);

        // Perform drag and drop
        source.drag_to(&target).await.map_err(|e| {
            ToolError::ExecutionFailed(format!(
                "Failed to drag '{}' to '{}': {}",
                input.start_element, input.end_element, e
            ))
        })?;

        // Invalidate cache after interaction
        context.invalidate_cache();

        Ok(format!(
            "Dragged {} [ref={}] to {} [ref={}]",
            input.start_element, input.start_ref, input.end_element, input.end_ref
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserDragTool::new();

        assert_eq!(tool.name(), "browser_drag");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        let required = schema["required"].as_array().unwrap();
        assert!(required.contains(&json!("startRef")));
        assert!(required.contains(&json!("endRef")));
    }

    #[test]
    fn test_input_parsing() {
        let input: BrowserDragInput = serde_json::from_value(json!({
            "startRef": "e1a2b3c",
            "startElement": "Draggable item",
            "endRef": "e4d5e6f",
            "endElement": "Drop zone"
        }))
        .unwrap();

        assert_eq!(input.start_ref, "e1a2b3c");
        assert_eq!(input.end_ref, "e4d5e6f");
    }
}
