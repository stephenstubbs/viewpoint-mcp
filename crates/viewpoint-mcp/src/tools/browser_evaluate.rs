//! Browser evaluate tool for executing JavaScript in page context

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use viewpoint_js::js;

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, SnapshotOptions};

/// Browser evaluate tool - executes JavaScript in page context
pub struct BrowserEvaluateTool;

/// Input parameters for `browser_evaluate`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserEvaluateInput {
    /// JavaScript function or expression to execute
    /// For element evaluation: `(element) => { /* code */ }`
    /// For page evaluation: `() => { /* code */ }`
    pub function: String,

    /// Element reference from snapshot (optional)
    /// When provided, the element will be passed as the first argument to the function
    #[serde(rename = "ref")]
    pub element_ref: Option<String>,

    /// Human-readable element description (required if ref is provided)
    pub element: Option<String>,
}

impl BrowserEvaluateTool {
    /// Create a new browser evaluate tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserEvaluateTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserEvaluateTool {
    fn name(&self) -> &'static str {
        "browser_evaluate"
    }

    fn description(&self) -> &'static str {
        "Execute JavaScript in the page context. When an element ref is provided, \
         the function receives that element as its first argument. Returns the \
         serialized result of the expression."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["function"],
            "properties": {
                "function": {
                    "type": "string",
                    "description": "JavaScript function or expression to execute. Use `() => { /* code */ }` for page-level code or `(element) => { /* code */ }` when element ref is provided."
                },
                "ref": {
                    "type": "string",
                    "description": "Element reference from browser_snapshot. When provided, the element will be passed to the function."
                },
                "element": {
                    "type": "string",
                    "description": "Human-readable description of the element. Required if ref is provided."
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserEvaluateInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Validate that element is provided if ref is provided
        if input.element_ref.is_some() && input.element.is_none() {
            return Err(ToolError::InvalidParams(
                "element description is required when ref is provided".to_string(),
            ));
        }

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

        // Execute JavaScript based on whether an element ref is provided
        let result = if let Some(ref element_ref_str) = input.element_ref {
            // Capture current snapshot for validation
            let options = SnapshotOptions::default();
            let snapshot = AccessibilitySnapshot::capture(&page, options)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

            // Validate the ref exists in the snapshot
            snapshot.lookup(element_ref_str).map_err(|e| {
                ToolError::ElementNotFound(format!("Element ref '{element_ref_str}': {e}"))
            })?;

            // Use native ref resolution API from viewpoint
            let locator = page.locator_from_ref(element_ref_str);

            // Evaluate with element - the viewpoint API expects expressions using `element`
            // User provides a function like `(el) => el.textContent`, we need to convert it
            // to an expression: `((el) => el.textContent)(element)`
            // Using js! macro with @{} raw interpolation for the user's function
            let user_fn = &input.function;
            let expression = js! { (@{user_fn})(element) };

            locator.evaluate(&expression).await.map_err(|e| {
                ToolError::ExecutionFailed(format!("JavaScript evaluation failed: {e}"))
            })?
        } else {
            // Evaluate without element - page-level evaluation
            page.evaluate(&input.function).await.map_err(|e| {
                ToolError::ExecutionFailed(format!("JavaScript evaluation failed: {e}"))
            })?
        };

        // Invalidate cache after potential DOM modifications
        context.invalidate_cache();

        // Format the result
        let result_str = match result {
            Value::Null => "null".to_string(),
            Value::String(s) => s,
            other => serde_json::to_string_pretty(&other).unwrap_or_else(|_| format!("{other:?}")),
        };

        if let Some(element_desc) = input.element {
            Ok(format!(
                "Evaluated on {} [ref={}]: {}",
                element_desc,
                input.element_ref.unwrap_or_default(),
                result_str
            ))
        } else {
            Ok(format!("Evaluation result: {result_str}"))
        }
    }
}
