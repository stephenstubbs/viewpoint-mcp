//! Browser snapshot tool for capturing accessibility tree

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

/// Browser snapshot tool - captures accessibility tree for LLM consumption
pub struct BrowserSnapshotTool;

/// Input parameters for `browser_snapshot`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserSnapshotInput {
    /// Whether to include all refs (bypass compact mode)
    #[serde(default)]
    pub all_refs: bool,
}

impl BrowserSnapshotTool {
    /// Create a new browser snapshot tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserSnapshotTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserSnapshotTool {
    #[allow(clippy::unnecessary_literal_bound)]
    fn name(&self) -> &str {
        "browser_snapshot"
    }

    #[allow(clippy::unnecessary_literal_bound)]
    fn description(&self) -> &str {
        "Capture accessibility snapshot of the current page. Returns a structured text \
         representation of the page's accessibility tree, with element references (refs) \
         that can be used to interact with elements."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "allRefs": {
                    "type": "boolean",
                    "description": "Include refs for all interactive elements, bypassing compact mode. \
                                   Use when page has many elements and you need to interact with \
                                   Tier 2 (contextually interactive) elements.",
                    "default": false
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserSnapshotInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

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

        // Get context name for multi-context ref prefixing
        let context_name = if browser.list_contexts().len() > 1 {
            Some(context.name.clone())
        } else {
            None
        };

        // Capture snapshot
        let options = SnapshotOptions {
            all_refs: input.all_refs,
            context: context_name,
        };

        let snapshot = AccessibilitySnapshot::capture(page, options)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        // Format for output
        let output = snapshot.format();

        // Add metadata
        let ref_count = snapshot.ref_count();
        let element_count = snapshot.element_count();
        let compact = snapshot.is_compact();

        let mut result = format!(
            "Page snapshot ({} elements, {} refs{})\n\n{}",
            element_count,
            ref_count,
            if compact { ", compact mode" } else { "" },
            output
        );

        // Add usage hint if in compact mode
        if compact {
            result.push_str(
                "\n\n[Hint: Use allRefs: true to see refs for all interactive elements]",
            );
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserSnapshotTool::new();

        assert_eq!(tool.name(), "browser_snapshot");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["allRefs"].is_object());
    }

    #[test]
    fn test_input_parsing() {
        let input: BrowserSnapshotInput = serde_json::from_value(json!({})).unwrap();
        assert!(!input.all_refs);

        let input: BrowserSnapshotInput = serde_json::from_value(json!({
            "allRefs": true
        }))
        .unwrap();
        assert!(input.all_refs);
    }
}
