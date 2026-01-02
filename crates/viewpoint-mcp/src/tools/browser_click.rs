//! Browser click tool for clicking elements by ref

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use viewpoint_core::MouseButton;

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

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

    /// Mouse button to use
    #[serde(default)]
    pub button: ClickButton,

    /// Whether to perform a double-click
    #[serde(default)]
    pub double_click: bool,

    /// Modifier keys to hold during click
    #[serde(default)]
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

impl ClickButton {
    /// Convert to viewpoint-core `MouseButton`
    fn to_mouse_button(self) -> MouseButton {
        match self {
            Self::Left => MouseButton::Left,
            Self::Right => MouseButton::Right,
            Self::Middle => MouseButton::Middle,
        }
    }
}

impl ModifierKey {
    /// Convert to CDP modifier bitmask value
    fn to_cdp_modifier(self) -> i32 {
        use viewpoint_cdp::protocol::input::modifiers;
        match self {
            Self::Alt => modifiers::ALT,
            Self::Control => modifiers::CTRL,
            Self::ControlOrMeta => {
                // On macOS, ControlOrMeta means Meta; on other platforms, it means Ctrl
                // Since we can't detect platform at runtime reliably, use Ctrl as the default
                // (this matches Playwright's behavior on Linux/Windows)
                modifiers::CTRL
            }
            Self::Meta => modifiers::META,
            Self::Shift => modifiers::SHIFT,
        }
    }
}

/// Convert a list of modifier keys to a combined CDP modifiers bitmask
fn modifiers_to_bitmask(modifiers: &[ModifierKey]) -> i32 {
    modifiers.iter().fold(0, |acc, m| acc | m.to_cdp_modifier())
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
    fn name(&self) -> &'static str {
        "browser_click"
    }

    #[allow(clippy::unnecessary_literal_bound)]
    fn description(&self) -> &'static str {
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

        // Use native ref resolution API from viewpoint
        let locator = page.locator_from_ref(&input.element_ref);

        // Build the click operation with button and modifier support
        let modifiers_bitmask = modifiers_to_bitmask(&input.modifiers);

        // Perform the click based on options
        let click_result = if input.double_click {
            // Double-click with modifiers (button is always left for double-click)
            // Note: DblclickBuilder doesn't support non-left buttons by design
            let mut builder = locator.dblclick();
            if modifiers_bitmask != 0 {
                builder = builder.modifiers(modifiers_bitmask);
            }
            builder.await
        } else {
            // Single click with button and modifiers
            let mut builder = locator.click();
            if !matches!(input.button, ClickButton::Left) {
                builder = builder.button(input.button.to_mouse_button());
            }
            if modifiers_bitmask != 0 {
                builder = builder.modifiers(modifiers_bitmask);
            }
            builder.await
        };

        match click_result {
            Ok(()) => {
                // Invalidate cache after successful click (DOM may have changed)
                context.invalidate_cache();

                let element_desc = input.element.as_deref().unwrap_or("element");
                Ok(format!(
                    "Clicked {} [ref={}]",
                    element_desc, input.element_ref
                ))
            }
            Err(e) => Err(ToolError::ExecutionFailed(format!(
                "Failed to click element '{}' [ref={}]: {}. The element may have changed since the snapshot.",
                input.element.as_deref().unwrap_or("element"),
                input.element_ref,
                e
            ))),
        }
    }
}
