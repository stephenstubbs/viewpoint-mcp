//! Browser resize tool for resizing the viewport

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser resize tool - resizes the browser viewport
pub struct BrowserResizeTool;

/// Input parameters for `browser_resize`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserResizeInput {
    /// Width of the viewport in pixels
    pub width: i32,

    /// Height of the viewport in pixels
    pub height: i32,
}

impl BrowserResizeTool {
    /// Create a new browser resize tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserResizeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserResizeTool {
    fn name(&self) -> &'static str {
        "browser_resize"
    }

    fn description(&self) -> &'static str {
        "Resize the browser viewport to the specified dimensions. \
         This affects how the page is rendered and can trigger responsive layouts."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["width", "height"],
            "properties": {
                "width": {
                    "type": "number",
                    "description": "Width of the viewport in pixels",
                    "minimum": 1
                },
                "height": {
                    "type": "number",
                    "description": "Height of the viewport in pixels",
                    "minimum": 1
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        const MAX_DIMENSION: i32 = 16384;

        // Parse input
        let input: BrowserResizeInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Validate dimensions
        if input.width <= 0 {
            return Err(ToolError::InvalidParams(
                "Width must be greater than 0".to_string(),
            ));
        }
        if input.height <= 0 {
            return Err(ToolError::InvalidParams(
                "Height must be greater than 0".to_string(),
            ));
        }

        // Reasonable maximum dimensions
        if input.width > MAX_DIMENSION {
            return Err(ToolError::InvalidParams(format!(
                "Width cannot exceed {MAX_DIMENSION} pixels"
            )));
        }
        if input.height > MAX_DIMENSION {
            return Err(ToolError::InvalidParams(format!(
                "Height cannot exceed {MAX_DIMENSION} pixels"
            )));
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

        // Set viewport size
        page.set_viewport_size(input.width, input.height)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to resize viewport: {e}")))?;

        // Invalidate cache since the layout may have changed
        context.invalidate_cache();

        Ok(format!(
            "Resized viewport to {}x{} pixels",
            input.width, input.height
        ))
    }
}
