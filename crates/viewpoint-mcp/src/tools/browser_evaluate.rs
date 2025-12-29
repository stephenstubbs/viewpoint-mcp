//! Browser evaluate tool for executing JavaScript in page context

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;
use crate::snapshot::{AccessibilitySnapshot, ElementRef, SnapshotOptions};

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
    fn name(&self) -> &str {
        "browser_evaluate"
    }

    fn description(&self) -> &str {
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
            .ok_or_else(|| ToolError::BrowserNotAvailable("No active page".to_string()))?;

        // Execute JavaScript based on whether an element ref is provided
        let result = if let Some(ref element_ref_str) = input.element_ref {
            // Parse and validate the element ref
            let element_ref =
                ElementRef::parse(element_ref_str).map_err(ToolError::InvalidParams)?;

            // Capture current snapshot for validation
            let options = SnapshotOptions::default();
            let snapshot = AccessibilitySnapshot::capture(page, options)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

            // Validate the ref exists
            snapshot.lookup(element_ref_str).map_err(|e| {
                ToolError::ElementNotFound(format!("Element ref '{}': {}", element_ref_str, e))
            })?;

            // Build selector from the ref
            let selector = format!("[data-ref='{}']", element_ref.hash);
            let locator = page.locator(&selector);

            // Evaluate with element - wrap the function to receive the element
            let wrapped_js = format!(
                "async (element) => {{ return ({})(element); }}",
                input.function
            );

            locator.evaluate(&wrapped_js).await.map_err(|e| {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata() {
        let tool = BrowserEvaluateTool::new();

        assert_eq!(tool.name(), "browser_evaluate");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(
            schema["required"]
                .as_array()
                .unwrap()
                .contains(&json!("function"))
        );
    }

    #[test]
    fn test_input_parsing_page_level() {
        let input: BrowserEvaluateInput = serde_json::from_value(json!({
            "function": "() => document.title"
        }))
        .unwrap();

        assert_eq!(input.function, "() => document.title");
        assert!(input.element_ref.is_none());
        assert!(input.element.is_none());
    }

    #[test]
    fn test_input_parsing_with_element() {
        let input: BrowserEvaluateInput = serde_json::from_value(json!({
            "function": "(el) => el.textContent",
            "ref": "e1a2b3c",
            "element": "Submit button"
        }))
        .unwrap();

        assert_eq!(input.function, "(el) => el.textContent");
        assert_eq!(input.element_ref, Some("e1a2b3c".to_string()));
        assert_eq!(input.element, Some("Submit button".to_string()));
    }

    #[test]
    fn test_input_parsing_complex_function() {
        let input: BrowserEvaluateInput = serde_json::from_value(json!({
            "function": "() => { const items = document.querySelectorAll('li'); return items.length; }"
        }))
        .unwrap();

        assert!(input.function.contains("querySelectorAll"));
    }
}
