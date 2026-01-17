//! Browser scroll into view tool for scrolling elements into the visible viewport

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolOutput, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

/// Browser scroll into view tool - scrolls an element into the visible viewport
pub struct BrowserScrollIntoViewTool;

/// Input parameters for `browser_scroll_into_view`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserScrollIntoViewInput {
    /// Element reference from snapshot
    #[serde(rename = "ref")]
    pub element_ref: String,

    /// Human-readable element description for verification
    pub element: String,
}

impl BrowserScrollIntoViewTool {
    /// Create a new browser scroll into view tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserScrollIntoViewTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserScrollIntoViewTool {
    fn name(&self) -> &'static str {
        "browser_scroll_into_view"
    }

    fn description(&self) -> &'static str {
        "Scroll an element into the visible viewport. Useful for bringing elements into view \
         before taking screenshots or when elements are outside the visible viewport."
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
        let input: BrowserScrollIntoViewInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Get active page (need mutable context for cache invalidation)
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

        // Validate the ref exists in the snapshot
        snapshot.lookup(&input.element_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Element ref '{}': {}", input.element_ref, e))
        })?;

        // Use native ref resolution API from viewpoint
        let locator = page.locator_from_ref(&input.element_ref);

        // Scroll the element into view
        locator.scroll_into_view_if_needed().await.map_err(|e| {
            ToolError::ExecutionFailed(format!(
                "Failed to scroll element '{}' into view: {}",
                input.element, e
            ))
        })?;

        // Invalidate cache after scroll (DOM may have changed via lazy loading)
        context.invalidate_cache();

        Ok(ToolOutput::text(format!(
            "Scrolled {} into view [ref={}]",
            input.element, input.element_ref
        )))
    }
}
