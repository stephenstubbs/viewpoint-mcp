//! Browser mouse click at coordinates tool for vision-enabled LLMs

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::traits::Capability;
use super::{Tool, ToolError, ToolOutput, ToolResult};
use crate::browser::BrowserState;

/// Browser mouse click at coordinates tool
pub struct BrowserMouseClickXyTool;

/// Input parameters for `browser_mouse_click_xy`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserMouseClickXyInput {
    /// X coordinate (CSS pixels from left edge of viewport)
    pub x: f64,

    /// Y coordinate (CSS pixels from top edge of viewport)
    pub y: f64,

    /// Mouse button to use
    #[serde(default)]
    pub button: MouseButton,

    /// Number of clicks (1 for single, 2 for double-click)
    #[serde(default = "default_click_count")]
    pub click_count: u32,

    /// Description of what is being clicked (for logging/verification)
    pub element: Option<String>,
}

fn default_click_count() -> u32 {
    1
}

/// Mouse button for click
#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    #[default]
    Left,
    Right,
    Middle,
}

impl From<MouseButton> for viewpoint_core::MouseButton {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => Self::Left,
            MouseButton::Right => Self::Right,
            MouseButton::Middle => Self::Middle,
        }
    }
}

impl BrowserMouseClickXyTool {
    /// Create a new browser mouse click xy tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserMouseClickXyTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserMouseClickXyTool {
    fn name(&self) -> &'static str {
        "browser_mouse_click_xy"
    }

    fn description(&self) -> &'static str {
        "Click at specific viewport coordinates. For vision-enabled LLMs that can identify \
         element positions from screenshots. Coordinates are in CSS pixels relative to viewport."
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
                "button": {
                    "type": "string",
                    "enum": ["left", "right", "middle"],
                    "default": "left",
                    "description": "Mouse button to click"
                },
                "clickCount": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 3,
                    "default": 1,
                    "description": "Number of clicks (1 for single, 2 for double-click)"
                },
                "element": {
                    "type": "string",
                    "description": "Optional description of what is being clicked (for logging)"
                }
            }
        })
    }

    fn required_capability(&self) -> Option<Capability> {
        Some(Capability::Vision)
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserMouseClickXyInput = serde_json::from_value(args.clone())
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

        // Get active page (need mutable context for cache invalidation)
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        let page = context
            .active_page()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get active page: {e}")))?
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Perform the click using page.mouse()
        let mouse = page.mouse();
        let button: viewpoint_core::MouseButton = input.button.into();

        if input.click_count == 2 {
            // Double-click
            mouse
                .dblclick(input.x, input.y)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        } else {
            // Single or triple click with specified button
            mouse
                .click(input.x, input.y)
                .button(button)
                .click_count(i32::try_from(input.click_count).unwrap_or(1))
                .send()
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        }

        // Invalidate cache after click (DOM may have changed)
        context.invalidate_cache();

        let element_desc = input.element.as_deref().unwrap_or("position");
        let click_type = match input.click_count {
            2 => "Double-clicked",
            3 => "Triple-clicked",
            _ => "Clicked",
        };
        let button_str = match input.button {
            MouseButton::Left => "",
            MouseButton::Right => " (right button)",
            MouseButton::Middle => " (middle button)",
        };

        Ok(ToolOutput::text(format!(
            "{} {} at ({}, {}){}",
            click_type, element_desc, input.x, input.y, button_str
        )))
    }
}
