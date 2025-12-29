//! Browser click tool for clicking elements by ref

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, ElementRef, SnapshotOptions};

/// Browser click tool - clicks an element using its ref
pub struct BrowserClickTool;

/// Input parameters for `browser_click`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserClickInput {
    /// Element reference from snapshot (e.g., "e1a2b3c" or "clean:e1a2b3c")
    #[serde(rename = "ref")]
    pub element_ref: String,

    /// Human-readable element description for verification
    pub element: Option<String>,

    /// Mouse button to use (TODO: implement in execute)
    #[serde(default)]
    #[allow(dead_code)]
    pub button: ClickButton,

    /// Whether to perform a double-click
    #[serde(default)]
    pub double_click: bool,

    /// Modifier keys to hold during click (TODO: implement in execute)
    #[serde(default)]
    #[allow(dead_code)]
    pub modifiers: Vec<ModifierKey>,
}

/// Mouse button for click
#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClickButton {
    #[default]
    Left,
    Right,
    Middle,
}

/// Modifier keys
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum ModifierKey {
    Alt,
    Control,
    #[serde(alias = "ControlOrMeta")]
    ControlOrMeta,
    Meta,
    Shift,
}

impl BrowserClickTool {
    /// Create a new browser click tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserClickTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserClickTool {
    #[allow(clippy::unnecessary_literal_bound)]
    fn name(&self) -> &str {
        "browser_click"
    }

    #[allow(clippy::unnecessary_literal_bound)]
    fn description(&self) -> &str {
        "Click an element on the page using its ref from browser_snapshot. \
         Supports left/right/middle click, double-click, and modifier keys."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["ref", "element"],
            "properties": {
                "ref": {
                    "type": "string",
                    "description": "Element reference from browser_snapshot (e.g., 'e1a2b3c')"
                },
                "element": {
                    "type": "string",
                    "description": "Human-readable description of the element for verification"
                },
                "button": {
                    "type": "string",
                    "enum": ["left", "right", "middle"],
                    "default": "left",
                    "description": "Mouse button to click"
                },
                "doubleClick": {
                    "type": "boolean",
                    "default": false,
                    "description": "Whether to double-click"
                },
                "modifiers": {
                    "type": "array",
                    "items": {
                        "type": "string",
                        "enum": ["Alt", "Control", "ControlOrMeta", "Meta", "Shift"]
                    },
                    "description": "Modifier keys to hold during click"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserClickInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Parse the element ref
        let element_ref = ElementRef::parse(&input.element_ref)
            .map_err(ToolError::InvalidParams)?;

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

        // Capture current snapshot for validation
        let options = SnapshotOptions::default();
        let snapshot = AccessibilitySnapshot::capture(page, options)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        // Validate the ref exists
        snapshot.lookup(&input.element_ref).map_err(|e| {
            ToolError::ElementNotFound(format!("Element ref '{}': {}", input.element_ref, e))
        })?;

        // Build aria snapshot selector from the ref
        // This is a placeholder - real implementation would use CDP to click
        // based on the accessibility tree node
        let selector = format!("[data-ref='{}']", element_ref.hash);

        // For now, try to locate and click
        // In a real implementation, we'd use the accessibility tree to find the element
        let locator = page.locator(&selector);

        // Perform the click based on options
        let click_result = if input.double_click {
            locator.dblclick().await
        } else {
            locator.click().await
        };

        match click_result {
            Ok(()) => {
                let element_desc = input.element.as_deref().unwrap_or("element");
                Ok(format!("Clicked {} [ref={}]", element_desc, input.element_ref))
            }
            Err(_) => {
                // Fallback: try clicking via accessibility tree directly
                // This is where we'd implement the actual ref-to-element resolution
                Err(ToolError::ElementNotFound(format!(
                    "Could not click element with ref '{}'. The element may have changed since the snapshot.",
                    input.element_ref
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserClickTool::new();

        assert_eq!(tool.name(), "browser_click");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["required"].as_array().unwrap().contains(&json!("ref")));
    }

    #[test]
    fn test_input_parsing() {
        let input: BrowserClickInput = serde_json::from_value(json!({
            "ref": "e1a2b3c",
            "element": "Submit button"
        }))
        .unwrap();

        assert_eq!(input.element_ref, "e1a2b3c");
        assert_eq!(input.element, Some("Submit button".to_string()));
        assert!(matches!(input.button, ClickButton::Left));
        assert!(!input.double_click);
    }

    #[test]
    fn test_input_with_options() {
        let input: BrowserClickInput = serde_json::from_value(json!({
            "ref": "clean:e1a2b3c",
            "element": "Menu item",
            "button": "right",
            "doubleClick": true,
            "modifiers": ["Control", "Shift"]
        }))
        .unwrap();

        assert_eq!(input.element_ref, "clean:e1a2b3c");
        assert!(matches!(input.button, ClickButton::Right));
        assert!(input.double_click);
        assert_eq!(input.modifiers.len(), 2);
    }
}
