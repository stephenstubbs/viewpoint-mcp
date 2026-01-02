//! Browser snapshot tool for capturing accessibility tree

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::{debug, instrument};

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
    fn name(&self) -> &'static str {
        "browser_snapshot"
    }

    #[allow(clippy::unnecessary_literal_bound)]
    fn description(&self) -> &'static str {
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

    #[instrument(skip(self, args, browser), fields(all_refs))]
    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserSnapshotInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        tracing::Span::current().record("all_refs", input.all_refs);

        // Ensure browser is initialized
        debug!("browser_initialize: start");
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;
        debug!("browser_initialize: complete");

        // Get context name for multi-context ref prefixing
        let context_name = if browser.list_contexts().len() > 1 {
            Some(
                browser
                    .active_context()
                    .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?
                    .name
                    .clone(),
            )
        } else {
            None
        };

        // Try to get cached snapshot first
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        if let Some(cached) = context.get_cached_snapshot(input.all_refs) {
            debug!("snapshot cache hit");

            // Use single-pass counting
            let (ref_count, element_count) = cached.root().counts();
            let compact = cached.is_compact();

            debug!(element_count, ref_count, "format_snapshot: cached");
            let output = cached.format();

            let mut result = format!(
                "Page snapshot ({element_count} elements, {ref_count} refs{})\n\n{output}",
                if compact { ", compact mode" } else { "" },
            );

            if compact {
                result.push_str(
                    "\n\n[Hint: Use allRefs: true to see refs for all interactive elements]",
                );
            }

            return Ok(result);
        }

        debug!("snapshot cache miss");

        // Get active page for capture
        let page = context
            .active_page()
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Capture new snapshot
        debug!("capture_snapshot: start");

        let options = SnapshotOptions {
            all_refs: input.all_refs,
            context: context_name,
        };

        let snapshot = AccessibilitySnapshot::capture(page, options)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        debug!("capture_snapshot: complete");

        // Use single-pass counting
        let (ref_count, element_count) = snapshot.root().counts();
        let compact = snapshot.is_compact();

        debug!(element_count, ref_count, "format_snapshot: fresh");
        let output = snapshot.format();

        let mut result = format!(
            "Page snapshot ({element_count} elements, {ref_count} refs{})\n\n{output}",
            if compact { ", compact mode" } else { "" },
        );

        // Add usage hint if in compact mode
        if compact {
            result
                .push_str("\n\n[Hint: Use allRefs: true to see refs for all interactive elements]");
        }

        // Cache the snapshot for future requests
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;
        context.cache_snapshot(snapshot, input.all_refs);

        Ok(result)
    }
}
