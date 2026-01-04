//! Browser PDF save tool for saving pages as PDF

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::traits::Capability;
use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser PDF save tool
pub struct BrowserPdfSaveTool;

/// Input parameters for `browser_pdf_save`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPdfSaveInput {
    /// Path to save the PDF file (required)
    pub path: String,

    /// Paper format
    #[serde(default)]
    pub format: PaperFormat,

    /// Use landscape orientation
    #[serde(default)]
    pub landscape: bool,

    /// Print background graphics
    #[serde(default)]
    pub print_background: bool,

    /// Scale factor (0.1 to 2.0, default 1.0)
    pub scale: Option<f64>,

    /// Page ranges to print (e.g., "1-5, 8, 11-13")
    pub page_ranges: Option<String>,

    /// Margin in inches (uniform on all sides)
    pub margin: Option<f64>,
}

/// Paper format options
#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PaperFormat {
    #[default]
    Letter,
    Legal,
    Tabloid,
    Ledger,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
}

impl From<PaperFormat> for viewpoint_core::PaperFormat {
    fn from(format: PaperFormat) -> Self {
        match format {
            PaperFormat::Letter => Self::Letter,
            PaperFormat::Legal => Self::Legal,
            PaperFormat::Tabloid => Self::Tabloid,
            PaperFormat::Ledger => Self::Ledger,
            PaperFormat::A0 => Self::A0,
            PaperFormat::A1 => Self::A1,
            PaperFormat::A2 => Self::A2,
            PaperFormat::A3 => Self::A3,
            PaperFormat::A4 => Self::A4,
            PaperFormat::A5 => Self::A5,
            PaperFormat::A6 => Self::A6,
        }
    }
}

impl BrowserPdfSaveTool {
    /// Create a new browser PDF save tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserPdfSaveTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserPdfSaveTool {
    fn name(&self) -> &'static str {
        "browser_pdf_save"
    }

    fn description(&self) -> &'static str {
        "Save the current page as a PDF file. Supports various paper formats, \
         orientation, scaling, and page range selection."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["path"],
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path to save the PDF (e.g., '/tmp/page.pdf')"
                },
                "format": {
                    "type": "string",
                    "enum": ["letter", "legal", "tabloid", "ledger", "a0", "a1", "a2", "a3", "a4", "a5", "a6"],
                    "default": "letter",
                    "description": "Paper format"
                },
                "landscape": {
                    "type": "boolean",
                    "default": false,
                    "description": "Use landscape orientation"
                },
                "printBackground": {
                    "type": "boolean",
                    "default": false,
                    "description": "Print background graphics"
                },
                "scale": {
                    "type": "number",
                    "minimum": 0.1,
                    "maximum": 2.0,
                    "description": "Scale factor (default 1.0)"
                },
                "pageRanges": {
                    "type": "string",
                    "description": "Page ranges to print (e.g., '1-5, 8, 11-13')"
                },
                "margin": {
                    "type": "number",
                    "description": "Margin in inches (uniform on all sides)"
                }
            }
        })
    }

    fn required_capability(&self) -> Option<Capability> {
        Some(Capability::Pdf)
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserPdfSaveInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Validate path
        if input.path.is_empty() {
            return Err(ToolError::InvalidParams("Path cannot be empty".to_string()));
        }

        // Validate scale if provided
        if let Some(scale) = input.scale
            && !(0.1..=2.0).contains(&scale)
        {
            return Err(ToolError::InvalidParams(
                "Scale must be between 0.1 and 2.0".to_string(),
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

        // Build PDF with options
        let mut pdf_builder = page
            .pdf()
            .format(input.format.into())
            .landscape(input.landscape)
            .print_background(input.print_background)
            .path(&input.path);

        if let Some(scale) = input.scale {
            pdf_builder = pdf_builder.scale(scale);
        }

        if let Some(ref page_ranges) = input.page_ranges {
            pdf_builder = pdf_builder.page_ranges(page_ranges);
        }

        if let Some(margin) = input.margin {
            pdf_builder = pdf_builder.margin(margin);
        }

        // Generate the PDF
        let data = pdf_builder
            .generate()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        Ok(format!(
            "PDF saved to '{}' ({} bytes, format: {:?}, landscape: {})",
            input.path,
            data.len(),
            input.format,
            input.landscape
        ))
    }
}
