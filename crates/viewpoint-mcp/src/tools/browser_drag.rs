//! Browser drag tool for drag and drop operations

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

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
    fn name(&self) -> &'static str {
        "browser_drag"
    }

    fn description(&self) -> &'static str {
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
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get active page: {e}")))?
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Capture current snapshot for validation
        let options = SnapshotOptions::default();
        let snapshot = AccessibilitySnapshot::capture(&page, options)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        // Validate both refs exist in the snapshot
        snapshot.lookup(&input.start_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Source ref '{}': {}", input.start_ref, e))
        })?;
        snapshot.lookup(&input.end_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Target ref '{}': {}", input.end_ref, e))
        })?;

        // Use native ref resolution API from viewpoint 0.2.9
        let source = page.locator_from_ref(&input.start_ref);
        let target = page.locator_from_ref(&input.end_ref);

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
