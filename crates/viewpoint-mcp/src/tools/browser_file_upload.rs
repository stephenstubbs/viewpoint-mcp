//! Browser file upload tool for uploading files to file inputs

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};
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
    fn name(&self) -> &str {
        "browser_file_upload"
    }

    fn description(&self) -> &str {
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
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // If no paths provided, cancel the file chooser
        if input.paths.is_empty() {
            // Set empty files on the file input to cancel
            let locator = page.locator("input[type=file]");
            locator.set_input_files::<&str>(&[]).await.map_err(|e| {
                ToolError::ExecutionFailed(format!("Failed to cancel file chooser: {}", e))
            })?;

            return Ok("File chooser cancelled".to_string());
        }

        // Validate all paths exist
        for path in &input.paths {
            let file_path = Path::new(path);
            if !file_path.exists() {
                return Err(ToolError::InvalidParams(format!(
                    "File not found: {}",
                    path
                )));
            }
            if !file_path.is_file() {
                return Err(ToolError::InvalidParams(format!(
                    "Path is not a file: {}",
                    path
                )));
            }
        }

        // Convert paths to &str slice for the API
        let path_refs: Vec<&str> = input.paths.iter().map(String::as_str).collect();

        // Set the files on the file input using locator
        let locator = page.locator("input[type=file]");
        locator
            .set_input_files(&path_refs)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to upload files: {}", e)))?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserFileUploadTool::new();

        assert_eq!(tool.name(), "browser_file_upload");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        // paths is optional, so no required fields
        assert!(schema.get("required").is_none());
    }

    #[test]
    fn test_input_parsing_with_paths() {
        let input: BrowserFileUploadInput = serde_json::from_value(json!({
            "paths": ["/path/to/file1.txt", "/path/to/file2.pdf"]
        }))
        .unwrap();

        assert_eq!(input.paths.len(), 2);
        assert_eq!(input.paths[0], "/path/to/file1.txt");
        assert_eq!(input.paths[1], "/path/to/file2.pdf");
    }

    #[test]
    fn test_input_parsing_empty_paths() {
        let input: BrowserFileUploadInput = serde_json::from_value(json!({
            "paths": []
        }))
        .unwrap();

        assert!(input.paths.is_empty());
    }

    #[test]
    fn test_input_parsing_no_paths() {
        let input: BrowserFileUploadInput = serde_json::from_value(json!({})).unwrap();

        assert!(input.paths.is_empty());
    }

    #[test]
    fn test_single_file() {
        let input: BrowserFileUploadInput = serde_json::from_value(json!({
            "paths": ["/home/user/documents/report.pdf"]
        }))
        .unwrap();

        assert_eq!(input.paths.len(), 1);
        assert_eq!(input.paths[0], "/home/user/documents/report.pdf");
    }
}
