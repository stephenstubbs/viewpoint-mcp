//! Browser mouse move to coordinates tool for vision-enabled LLMs

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::traits::Capability;
use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser mouse move to coordinates tool
pub struct BrowserMouseMoveXyTool;

/// Input parameters for `browser_mouse_move_xy`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserMouseMoveXyInput {
    /// X coordinate (CSS pixels from left edge of viewport)
    pub x: f64,

    /// Y coordinate (CSS pixels from top edge of viewport)
    pub y: f64,

    /// Number of intermediate steps for smooth movement (default: 1 = instant)
    #[serde(default = "default_steps")]
    pub steps: u32,
}

fn default_steps() -> u32 {
    1
}

impl BrowserMouseMoveXyTool {
    /// Create a new browser mouse move xy tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserMouseMoveXyTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserMouseMoveXyTool {
    fn name(&self) -> &str {
        "browser_mouse_move_xy"
    }

    fn description(&self) -> &str {
        "Move the mouse to specific viewport coordinates without clicking. \
         For vision-enabled LLMs. Useful for triggering hover states or positioning before click."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["x", "y"],
            "properties": {
                "x": {
                    "type": "number",
                    "description": "X coordinate in CSS pixels from left edge of viewport"
                },
                "y": {
                    "type": "number",
                    "description": "Y coordinate in CSS pixels from top edge of viewport"
                },
                "steps": {
                    "type": "integer",
                    "minimum": 1,
                    "default": 1,
                    "description": "Number of intermediate steps for smooth movement (1 = instant)"
                }
            }
        })
    }

    fn required_capability(&self) -> Option<Capability> {
        Some(Capability::Vision)
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserMouseMoveXyInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Validate coordinates are non-negative
        if input.x < 0.0 || input.y < 0.0 {
            return Err(ToolError::InvalidParams(
                "Coordinates must be non-negative".to_string(),
            ));
        }

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

        // Move the mouse using page.mouse()
        page.mouse()
            .move_(input.x, input.y)
            .steps(input.steps)
            .send()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        if input.steps > 1 {
            Ok(format!(
                "Moved mouse to ({}, {}) in {} steps",
                input.x, input.y, input.steps
            ))
        } else {
            Ok(format!("Moved mouse to ({}, {})", input.x, input.y))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserMouseMoveXyTool::new();

        assert_eq!(tool.name(), "browser_mouse_move_xy");
        assert!(tool.description().contains("coordinates"));
        assert!(tool.description().contains("vision"));

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"].as_array().unwrap().contains(&json!("x")));
        assert!(schema["required"].as_array().unwrap().contains(&json!("y")));
    }

    #[test]
    fn test_input_parsing_minimal() {
        let input: BrowserMouseMoveXyInput = serde_json::from_value(json!({
            "x": 100.0,
            "y": 200.0
        }))
        .unwrap();

        assert!((input.x - 100.0).abs() < f64::EPSILON);
        assert!((input.y - 200.0).abs() < f64::EPSILON);
        assert_eq!(input.steps, 1);
    }

    #[test]
    fn test_input_parsing_with_steps() {
        let input: BrowserMouseMoveXyInput = serde_json::from_value(json!({
            "x": 50.5,
            "y": 75.25,
            "steps": 10
        }))
        .unwrap();

        assert!((input.x - 50.5).abs() < f64::EPSILON);
        assert!((input.y - 75.25).abs() < f64::EPSILON);
        assert_eq!(input.steps, 10);
    }
}
