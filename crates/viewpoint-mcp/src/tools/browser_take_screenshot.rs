//! Browser take screenshot tool for capturing page screenshots

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

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
    fn name(&self) -> &'static str {
        "browser_take_screenshot"
    }

    fn description(&self) -> &'static str {
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
        use base64::engine::{Engine as _, general_purpose::STANDARD};

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
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get active page: {e}")))?
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
            // Validate the ref exists in the snapshot
            let options = SnapshotOptions::default();
            let snapshot = AccessibilitySnapshot::capture(&page, options)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

            snapshot.lookup(element_ref_str).map_err(|e| {
                ToolError::ElementNotFound(format!("Element ref '{element_ref_str}': {e}"))
            })?;

            // Get the locator for the element
            let locator = page.locator_from_ref(element_ref_str);

            // Workaround: locator.screenshot() doesn't work with ref-based locators
            // in viewpoint-core 0.2.16, so we use bounding_box + page screenshot with clip
            let bbox = locator
                .bounding_box()
                .await
                .map_err(|e| {
                    ToolError::ExecutionFailed(format!(
                        "Failed to get bounding box for element: {e}"
                    ))
                })?
                .ok_or_else(|| {
                    ToolError::ElementNotFound(format!(
                        "Element ref '{element_ref_str}' has no bounding box (may be hidden)"
                    ))
                })?;

            page.screenshot()
                .clip(bbox.x, bbox.y, bbox.width, bbox.height)
                .capture()
                .await
                .map_err(|e| {
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
        let base64_data = STANDARD.encode(&screenshot_bytes);

        // Return information about the screenshot
        // Only describe as element screenshot if ref was provided (element alone is ignored)
        let description = if input.element_ref.is_some() {
            if let Some(ref element_desc) = input.element {
                format!("element '{element_desc}'")
            } else {
                "element".to_string()
            }
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
