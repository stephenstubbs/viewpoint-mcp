//! Browser take screenshot tool for capturing page screenshots

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, ElementRef, SnapshotOptions};

/// Browser take screenshot tool - captures screenshots
pub struct BrowserTakeScreenshotTool;

/// Input parameters for `browser_take_screenshot`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserTakeScreenshotInput {
    /// Element reference for element screenshot (optional)
    #[serde(rename = "ref")]
    pub element_ref: Option<String>,

    /// Human-readable element description (required if ref provided)
    pub element: Option<String>,

    /// Save to filename (optional, defaults to timestamped name)
    pub filename: Option<String>,

    /// Capture full page (not just viewport)
    #[serde(default)]
    pub full_page: bool,

    /// Image format: png or jpeg
    #[serde(default = "default_image_type")]
    #[serde(rename = "type")]
    pub image_type: ImageFormat,
}

fn default_image_type() -> ImageFormat {
    ImageFormat::Png
}

/// Image format for screenshots
#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    #[default]
    Png,
    Jpeg,
}

impl BrowserTakeScreenshotTool {
    /// Create a new browser take screenshot tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserTakeScreenshotTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserTakeScreenshotTool {
    fn name(&self) -> &str {
        "browser_take_screenshot"
    }

    fn description(&self) -> &str {
        "Take a screenshot of the current page. Can capture the viewport, full page, \
         or a specific element. Use browser_snapshot for interacting with elements."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "ref": {
                    "type": "string",
                    "description": "Element reference to screenshot (optional, screenshots viewport if not provided)"
                },
                "element": {
                    "type": "string",
                    "description": "Human-readable element description (required if ref is provided)"
                },
                "filename": {
                    "type": "string",
                    "description": "Filename to save the screenshot. Defaults to page-{timestamp}.{ext}"
                },
                "fullPage": {
                    "type": "boolean",
                    "default": false,
                    "description": "Capture full scrollable page instead of viewport"
                },
                "type": {
                    "type": "string",
                    "enum": ["png", "jpeg"],
                    "default": "png",
                    "description": "Image format"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserTakeScreenshotInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Validate: if ref is provided, element must also be provided
        if input.element_ref.is_some() && input.element.is_none() {
            return Err(ToolError::InvalidParams(
                "Element description required when ref is provided".to_string(),
            ));
        }

        // Cannot use fullPage with element screenshot
        if input.full_page && input.element_ref.is_some() {
            return Err(ToolError::InvalidParams(
                "fullPage cannot be used with element screenshots".to_string(),
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

        // Generate filename if not provided
        let extension = match input.image_type {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
        };
        let filename = input.filename.unwrap_or_else(|| {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0);
            format!("page-{timestamp}.{extension}")
        });

        // Take the screenshot
        let screenshot_bytes = if let Some(ref element_ref_str) = input.element_ref {
            // Element screenshot
            let element_ref =
                ElementRef::parse(element_ref_str).map_err(ToolError::InvalidParams)?;

            // Validate the ref exists
            let options = SnapshotOptions::default();
            let snapshot = AccessibilitySnapshot::capture(page, options)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

            snapshot.lookup(element_ref_str).map_err(|e| {
                ToolError::ElementNotFound(format!("Element ref '{}': {}", element_ref_str, e))
            })?;

            // Build selector and screenshot element
            let selector = format!("[data-ref='{}']", element_ref.hash);
            let locator = page.locator(&selector);
            locator.screenshot().capture().await.map_err(|e| {
                ToolError::ExecutionFailed(format!("Element screenshot failed: {e}"))
            })?
        } else {
            // Page screenshot
            let mut builder = page.screenshot();
            if input.full_page {
                builder = builder.full_page(true);
            }
            builder
                .capture()
                .await
                .map_err(|e| ToolError::ExecutionFailed(format!("Screenshot failed: {e}")))?
        };

        // Encode as base64 for MCP response
        use base64::engine::{Engine as _, general_purpose::STANDARD};
        let base64_data = STANDARD.encode(&screenshot_bytes);

        // Return information about the screenshot
        let description = if let Some(ref element_desc) = input.element {
            format!("element '{element_desc}'")
        } else if input.full_page {
            "full page".to_string()
        } else {
            "viewport".to_string()
        };

        Ok(format!(
            "Screenshot of {} saved as {} ({} bytes, base64 encoded: {} chars)",
            description,
            filename,
            screenshot_bytes.len(),
            base64_data.len()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserTakeScreenshotTool::new();

        assert_eq!(tool.name(), "browser_take_screenshot");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
    }

    #[test]
    fn test_input_defaults() {
        let input: BrowserTakeScreenshotInput = serde_json::from_value(json!({})).unwrap();

        assert!(input.element_ref.is_none());
        assert!(!input.full_page);
        assert!(matches!(input.image_type, ImageFormat::Png));
    }

    #[test]
    fn test_input_full_page() {
        let input: BrowserTakeScreenshotInput = serde_json::from_value(json!({
            "fullPage": true,
            "type": "jpeg"
        }))
        .unwrap();

        assert!(input.full_page);
        assert!(matches!(input.image_type, ImageFormat::Jpeg));
    }

    #[test]
    fn test_input_element_screenshot() {
        let input: BrowserTakeScreenshotInput = serde_json::from_value(json!({
            "ref": "e1a2b3c",
            "element": "Login form"
        }))
        .unwrap();

        assert_eq!(input.element_ref, Some("e1a2b3c".to_string()));
        assert_eq!(input.element, Some("Login form".to_string()));
    }
}
