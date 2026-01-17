//! Browser take screenshot tool for capturing page screenshots
//!
//! Screenshots are saved to the screenshot directory (default: `.viewpoint-mcp-screenshots/`)
//! and can optionally return inline image data based on the `--image-responses` configuration.

use std::io::Cursor;

use async_trait::async_trait;
use chrono::Utc;
use image::ImageFormat as ImgFormat;
use image::imageops::FilterType;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{ContentItem, Tool, ToolError, ToolOutput, ToolResult};
use crate::browser::BrowserState;
use crate::server::ImageResponseMode;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

/// Maximum dimension for inline images (per Claude's vision guidelines)
pub(crate) const MAX_INLINE_DIMENSION: u32 = 1568;

/// Maximum megapixels for inline images (per Claude's vision guidelines)
pub(crate) const MAX_INLINE_MEGAPIXELS: f64 = 1.15;

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

/// Generate an ISO 8601 timestamp suitable for filenames.
/// Format: `2026-01-13T15-30-45-123Z` (colons and dots replaced with dashes)
fn generate_timestamp_filename(extension: &str) -> String {
    let now = Utc::now();
    let formatted = now.format("%Y-%m-%dT%H-%M-%S-%3fZ").to_string();
    format!("page-{formatted}.{extension}")
}

/// Scale an image to fit within Claude's vision limits for inline images.
///
/// Constraints:
/// - Max 1568px on any dimension
/// - Max 1.15 megapixels total
/// - Convert to JPEG at quality 80 for smaller size
///
/// Returns the scaled image as JPEG bytes.
pub(crate) fn scale_image_for_inline(image_bytes: &[u8]) -> Result<Vec<u8>, String> {
    // Load the image
    let img =
        image::load_from_memory(image_bytes).map_err(|e| format!("Failed to decode image: {e}"))?;

    let (width, height) = (img.width(), img.height());
    let megapixels = (width as f64 * height as f64) / 1_000_000.0;

    // Calculate scaling factors
    let mut scale = 1.0_f64;

    // Scale down if any dimension exceeds max
    if width > MAX_INLINE_DIMENSION {
        scale = scale.min(MAX_INLINE_DIMENSION as f64 / width as f64);
    }
    if height > MAX_INLINE_DIMENSION {
        scale = scale.min(MAX_INLINE_DIMENSION as f64 / height as f64);
    }

    // Scale down if total megapixels exceed limit
    if megapixels * scale * scale > MAX_INLINE_MEGAPIXELS {
        let target_scale = (MAX_INLINE_MEGAPIXELS / megapixels).sqrt();
        scale = scale.min(target_scale);
    }

    // Apply scaling if needed
    let scaled_img = if scale < 1.0 {
        let new_width = ((width as f64) * scale).round() as u32;
        let new_height = ((height as f64) * scale).round() as u32;
        img.resize(new_width, new_height, FilterType::Lanczos3)
    } else {
        img
    };

    // Encode as JPEG
    let mut buffer = Cursor::new(Vec::new());
    scaled_img
        .write_to(&mut buffer, ImgFormat::Jpeg)
        .map_err(|e| format!("Failed to encode JPEG: {e}"))?;

    // Note: The image crate doesn't support quality settings in this API
    // For better quality control, we'd need to use the jpeg encoder directly
    // For now, the default quality is acceptable

    Ok(buffer.into_inner())
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

        // Get screenshot configuration from browser state
        let screenshot_dir = browser.screenshot_dir().clone();
        let image_responses = browser.image_responses();

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

        // Determine file extension and generate filename
        let extension = match input.image_type {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
        };
        let filename = input
            .filename
            .unwrap_or_else(|| generate_timestamp_filename(extension));

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

        // Create screenshot directory if it doesn't exist
        if !screenshot_dir.exists() {
            std::fs::create_dir_all(&screenshot_dir).map_err(|e| {
                ToolError::ExecutionFailed(format!(
                    "Failed to create screenshot directory '{}': {e}",
                    screenshot_dir.display()
                ))
            })?;
        }

        // Save screenshot to file (always, regardless of mode)
        let file_path = screenshot_dir.join(&filename);
        std::fs::write(&file_path, &screenshot_bytes).map_err(|e| {
            ToolError::ExecutionFailed(format!(
                "Failed to save screenshot to '{}': {e}",
                file_path.display()
            ))
        })?;

        // Build relative path for response
        let relative_path = format!("{}/{}", screenshot_dir.display(), filename);

        // Describe what was captured
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

        // Build response based on image response mode
        match image_responses {
            ImageResponseMode::Omit => {
                // Minimal response - just confirmation
                Ok(ToolOutput::text(format!(
                    "Screenshot captured ({description})"
                )))
            }
            ImageResponseMode::File => {
                // Include relative file path in response
                Ok(ToolOutput::text(format!(
                    "Screenshot saved to {} ({description})",
                    relative_path
                )))
            }
            ImageResponseMode::Inline => {
                // Include file path AND base64 image
                let scaled_bytes = scale_image_for_inline(&screenshot_bytes).map_err(|e| {
                    ToolError::ExecutionFailed(format!("Failed to scale image for inline: {e}"))
                })?;

                let base64_data = STANDARD.encode(&scaled_bytes);

                Ok(ToolOutput::new(vec![
                    ContentItem::text(format!(
                        "Screenshot saved to {} ({description})",
                        relative_path
                    )),
                    ContentItem::image(base64_data, "image/jpeg"),
                ]))
            }
        }
    }
}
