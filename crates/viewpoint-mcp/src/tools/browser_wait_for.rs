//! Browser wait for tool for waiting on conditions

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser wait for tool - waits for text, text to disappear, or a specified time
pub struct BrowserWaitForTool;

/// Input parameters for `browser_wait_for`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserWaitForInput {
    /// Text to wait for to appear on the page
    pub text: Option<String>,

    /// Text to wait for to disappear from the page
    pub text_gone: Option<String>,

    /// Time to wait in seconds
    pub time: Option<f64>,
}

impl BrowserWaitForTool {
    /// Create a new browser wait for tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserWaitForTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserWaitForTool {
    fn name(&self) -> &'static str {
        "browser_wait_for"
    }

    fn description(&self) -> &'static str {
        "Wait for a condition: text to appear, text to disappear, or a specified time to pass. \
         Only one of text, textGone, or time should be provided."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": "Text to wait for to appear on the page"
                },
                "textGone": {
                    "type": "string",
                    "description": "Text to wait for to disappear from the page"
                },
                "time": {
                    "type": "number",
                    "description": "Time to wait in seconds"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserWaitForInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Count how many conditions are provided
        let condition_count = [
            input.text.is_some(),
            input.text_gone.is_some(),
            input.time.is_some(),
        ]
        .iter()
        .filter(|&&b| b)
        .count();

        if condition_count == 0 {
            return Err(ToolError::InvalidParams(
                "At least one of text, textGone, or time must be provided".to_string(),
            ));
        }

        if condition_count > 1 {
            return Err(ToolError::InvalidParams(
                "Only one of text, textGone, or time should be provided".to_string(),
            ));
        }

        // Handle simple time wait (doesn't require browser)
        if let Some(seconds) = input.time {
            if seconds < 0.0 {
                return Err(ToolError::InvalidParams(
                    "Time must be a positive number".to_string(),
                ));
            }
            if seconds > 60.0 {
                return Err(ToolError::InvalidParams(
                    "Time cannot exceed 60 seconds".to_string(),
                ));
            }

            tokio::time::sleep(Duration::from_secs_f64(seconds)).await;
            return Ok(format!("Waited for {seconds} seconds"));
        }

        // Ensure browser is initialized for text-based waits
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

        // Handle text appearance wait using wait_for_function
        if let Some(ref text) = input.text {
            // Escape the text for use in JavaScript
            let escaped_text = text.replace('\\', "\\\\").replace('"', "\\\"");
            let js_condition =
                format!(r#"() => document.body.innerText.includes("{escaped_text}")"#);

            page.wait_for_function(&js_condition)
                .wait()
                .await
                .map_err(|e| {
                    ToolError::Timeout(format!("Timeout waiting for text '{text}': {e}"))
                })?;

            // Invalidate cache as page content changed
            context.invalidate_cache();

            return Ok(format!("Text '{text}' appeared on page"));
        }

        // Handle text disappearance wait using wait_for_function
        if let Some(ref text) = input.text_gone {
            // Escape the text for use in JavaScript
            let escaped_text = text.replace('\\', "\\\\").replace('"', "\\\"");
            let js_condition =
                format!(r#"() => !document.body.innerText.includes("{escaped_text}")"#);

            page.wait_for_function(&js_condition)
                .wait()
                .await
                .map_err(|e| {
                    ToolError::Timeout(format!(
                        "Timeout waiting for text '{text}' to disappear: {e}"
                    ))
                })?;

            // Invalidate cache as page content changed
            context.invalidate_cache();

            return Ok(format!("Text '{text}' disappeared from page"));
        }

        // This shouldn't be reachable due to earlier validation
        Err(ToolError::InvalidParams(
            "No valid wait condition provided".to_string(),
        ))
    }
}
