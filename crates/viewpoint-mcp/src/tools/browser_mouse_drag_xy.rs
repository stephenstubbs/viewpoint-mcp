//! Browser mouse drag by coordinates tool for vision-enabled LLMs

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::traits::Capability;
use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser mouse drag by coordinates tool
pub struct BrowserMouseDragXyTool;

/// Input parameters for `browser_mouse_drag_xy`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserMouseDragXyInput {
    /// Starting X coordinate (CSS pixels from left edge of viewport)
    pub start_x: f64,

    /// Starting Y coordinate (CSS pixels from top edge of viewport)
    pub start_y: f64,

    /// Ending X coordinate (CSS pixels from left edge of viewport)
    pub end_x: f64,

    /// Ending Y coordinate (CSS pixels from top edge of viewport)
    pub end_y: f64,

    /// Number of intermediate steps for smooth drag (default: 10)
    #[serde(default = "default_steps")]
    pub steps: u32,
}

fn default_steps() -> u32 {
    10
}

impl BrowserMouseDragXyTool {
    /// Create a new browser mouse drag xy tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserMouseDragXyTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserMouseDragXyTool {
    fn name(&self) -> &'static str {
        "browser_mouse_drag_xy"
    }

    fn description(&self) -> &'static str {
        "Drag from one viewport coordinate to another. For vision-enabled LLMs. \
         Performs mouse down at start, moves to end, then releases."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["startX", "startY", "endX", "endY"],
            "properties": {
                "startX": {
                    "type": "number",
                    "description": "Starting X coordinate in CSS pixels"
                },
                "startY": {
                    "type": "number",
                    "description": "Starting Y coordinate in CSS pixels"
                },
                "endX": {
                    "type": "number",
                    "description": "Ending X coordinate in CSS pixels"
                },
                "endY": {
                    "type": "number",
                    "description": "Ending Y coordinate in CSS pixels"
                },
                "steps": {
                    "type": "integer",
                    "minimum": 1,
                    "default": 10,
                    "description": "Number of intermediate steps for smooth drag movement"
                }
            }
        })
    }

    fn required_capability(&self) -> Option<Capability> {
        Some(Capability::Vision)
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserMouseDragXyInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Validate coordinates are non-negative
        if input.start_x < 0.0 || input.start_y < 0.0 || input.end_x < 0.0 || input.end_y < 0.0 {
            return Err(ToolError::InvalidParams(
                "Coordinates must be non-negative".to_string(),
            ));
        }

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

        // Perform drag operation:
        // 1. Move to start position
        // 2. Mouse down
        // 3. Move to end position (with steps for smooth drag)
        // 4. Mouse up
        let mouse = page.mouse();

        // Move to start position
        mouse
            .move_(input.start_x, input.start_y)
            .send()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to move to start: {e}")))?;

        // Mouse down
        mouse
            .down()
            .send()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed mouse down: {e}")))?;

        // Move to end position with steps
        mouse
            .move_(input.end_x, input.end_y)
            .steps(input.steps)
            .send()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to drag to end: {e}")))?;

        // Mouse up
        mouse
            .up()
            .send()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed mouse up: {e}")))?;

        // Invalidate cache after drag (DOM may have changed)
        context.invalidate_cache();

        Ok(format!(
            "Dragged from ({}, {}) to ({}, {}) in {} steps",
            input.start_x, input.start_y, input.end_x, input.end_y, input.steps
        ))
    }
}
