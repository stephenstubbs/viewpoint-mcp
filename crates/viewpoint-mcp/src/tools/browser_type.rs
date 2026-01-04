//! Browser type tool for typing text into elements

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

/// Browser type tool - types text into an element
pub struct BrowserTypeTool;

/// Input parameters for `browser_type`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserTypeInput {
    /// Element reference from snapshot (e.g., "e1a2b3c" or "clean:e1a2b3c")
    #[serde(rename = "ref")]
    pub element_ref: String,

    /// Human-readable element description for verification
    pub element: String,

    /// Text to type into the element
    pub text: String,

    /// Whether to type slowly (character by character)
    #[serde(default)]
    pub slowly: bool,

    /// Whether to submit (press Enter) after typing
    #[serde(default)]
    pub submit: bool,
}

impl BrowserTypeTool {
    /// Create a new browser type tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserTypeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserTypeTool {
    fn name(&self) -> &'static str {
        "browser_type"
    }

    fn description(&self) -> &'static str {
        "Type text into an editable element on the page. Use 'slowly: true' for character-by-character \
         typing that triggers key handlers. Use 'submit: true' to press Enter after typing."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["ref", "element", "text"],
            "properties": {
                "ref": {
                    "type": "string",
                    "description": "Element reference from browser_snapshot"
                },
                "element": {
                    "type": "string",
                    "description": "Human-readable description of the element"
                },
                "text": {
                    "type": "string",
                    "description": "Text to type into the element"
                },
                "slowly": {
                    "type": "boolean",
                    "default": false,
                    "description": "Type one character at a time (triggers key handlers)"
                },
                "submit": {
                    "type": "boolean",
                    "default": false,
                    "description": "Press Enter after typing"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserTypeInput = serde_json::from_value(args.clone())
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

        // Validate the ref exists in the snapshot
        snapshot.lookup(&input.element_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Element ref '{}': {}", input.element_ref, e))
        })?;

        // Use native ref resolution API from viewpoint 0.2.9
        let locator = page.locator_from_ref(&input.element_ref);

        // Perform the typing
        let type_result = if input.slowly {
            locator.type_text(&input.text).await
        } else {
            locator.fill(&input.text).await
        };

        if let Err(e) = type_result {
            return Err(ToolError::ExecutionFailed(format!(
                "Failed to type into element '{}': {}",
                input.element, e
            )));
        }

        // Submit if requested - use page keyboard API for more reliable behavior
        // The locator.press() can have issues after fill() due to focus changes
        if input.submit
            && let Err(e) = page.keyboard().press("Enter").await
        {
            return Err(ToolError::ExecutionFailed(format!(
                "Failed to press Enter: {e}"
            )));
        }

        // Invalidate cache after interaction
        context.invalidate_cache();

        let mut result = format!(
            "Typed \"{}\" into {} [ref={}]",
            input.text, input.element, input.element_ref
        );
        if input.submit {
            result.push_str(" and submitted");
        }

        Ok(result)
    }
}
