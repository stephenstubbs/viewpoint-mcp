//! Browser file upload tool for uploading files to file inputs

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::Path;

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser file upload tool - uploads files to a file input
pub struct BrowserFileUploadTool;

/// Input parameters for `browser_file_upload`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserFileUploadInput {
    /// Absolute paths to the files to upload
    /// If empty or omitted, the file chooser dialog is cancelled
    #[serde(default)]
    pub paths: Vec<String>,
}

impl BrowserFileUploadTool {
    /// Create a new browser file upload tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserFileUploadTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserFileUploadTool {
    fn name(&self) -> &'static str {
        "browser_file_upload"
    }

    fn description(&self) -> &'static str {
        "Upload one or multiple files to a file input. Call this after clicking a file input \
         or button that triggers a file chooser. If paths is empty or omitted, the file \
         chooser dialog is cancelled."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "paths": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Absolute paths to the files to upload. If omitted or empty, the file chooser is cancelled."
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserFileUploadInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

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
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get active page: {e}")))?
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // If no paths provided, cancel the file chooser
        if input.paths.is_empty() {
            // Set empty files on the file input to cancel
            // Try multiple selectors to find any file input (including hidden ones)
            let empty: &[&str] = &[];
            if let Err(e) = Self::set_files_on_any_input(&page, empty).await {
                return Err(ToolError::ExecutionFailed(format!(
                    "Failed to cancel file chooser: {e}"
                )));
            }
            return Ok("File chooser cancelled".to_string());
        }

        // Validate all paths exist
        for path in &input.paths {
            let file_path = Path::new(path);
            if !file_path.exists() {
                return Err(ToolError::InvalidParams(format!("File not found: {path}")));
            }
            if !file_path.is_file() {
                return Err(ToolError::InvalidParams(format!(
                    "Path is not a file: {path}"
                )));
            }
        }

        // Convert paths to &str slice for the API
        let path_refs: Vec<&str> = input.paths.iter().map(String::as_str).collect();

        // Set the files on the file input using multiple strategies
        Self::set_files_on_any_input(&page, &path_refs)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to upload files: {e}")))?;

        // Invalidate cache after file upload
        context.invalidate_cache();

        let file_names: Vec<&str> = input
            .paths
            .iter()
            .filter_map(|p| Path::new(p).file_name()?.to_str())
            .collect();

        Ok(format!(
            "Uploaded {} file(s): {}",
            input.paths.len(),
            file_names.join(", ")
        ))
    }
}

impl BrowserFileUploadTool {
    /// Try multiple strategies to find and set files on a file input element.
    ///
    /// File inputs are often hidden for styling purposes, so we need to try
    /// different approaches:
    /// 1. Standard visible file input: `input[type=file]`
    /// 2. Hidden file input (opacity/visibility): Still matched by type selector
    /// 3. Any input with accept attribute (common pattern)
    async fn set_files_on_any_input<P: AsRef<std::path::Path>>(
        page: &viewpoint_core::Page,
        files: &[P],
    ) -> Result<(), String> {
        // Try the standard file input selector first
        let locator = page.locator("input[type=file]");

        // The locator.set_input_files handles hidden inputs correctly
        // by using CDP's DOM.setFileInputFiles which works regardless of visibility
        match locator.set_input_files(files).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                // Check if it's a "no elements found" error
                let error_str = e.to_string();
                if !error_str.contains("not found")
                    && !error_str.contains("No elements")
                    && !error_str.contains("no element")
                {
                    // Different error, propagate it
                    return Err(error_str);
                }
                // Fall through to try alternative selectors
            }
        }

        // Try to find input by accept attribute (common for hidden file inputs)
        let locator_with_accept = page.locator("input[accept]");
        match locator_with_accept.set_input_files(files).await {
            Ok(()) => return Ok(()),
            Err(_) => {}
        }

        // If all strategies failed, provide a helpful error message
        Err(
            "No file input element found on the page. Make sure to click the upload button \
             or file input element before calling this tool."
                .to_string(),
        )
    }
}
