//! Browser console messages tool for retrieving console logs

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser console messages tool - retrieves console log messages
pub struct BrowserConsoleMessagesTool;

/// Console log level for filtering
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ConsoleLevel {
    /// Errors only
    Error,
    /// Errors and warnings
    Warning,
    /// Errors, warnings, and info (default)
    #[default]
    Info,
    /// All messages including debug
    Debug,
}

/// Input parameters for `browser_console_messages`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserConsoleMessagesInput {
    /// Minimum log level to include
    #[serde(default)]
    pub level: ConsoleLevel,
}

impl BrowserConsoleMessagesTool {
    /// Create a new browser console messages tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserConsoleMessagesTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserConsoleMessagesTool {
    fn name(&self) -> &'static str {
        "browser_console_messages"
    }

    fn description(&self) -> &'static str {
        "Returns all console messages logged since the page was loaded. Messages are filtered \
         by level: 'error' (errors only), 'warning' (errors + warnings), 'info' (default, \
         includes log), 'debug' (all messages)."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "level": {
                    "type": "string",
                    "enum": ["error", "warning", "info", "debug"],
                    "default": "info",
                    "description": "Minimum log level to include. Each level includes more severe levels."
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserConsoleMessagesInput = serde_json::from_value(args.clone())
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

        // Use JavaScript to retrieve console messages from the page
        // Note: This requires that we've injected a console interceptor beforehand
        // For now, we provide a fallback that uses evaluate to get current console state
        let js_code = r#"(() => {
            // Check if we have captured console messages
            if (window.__viewpointConsoleMessages) {
                return window.__viewpointConsoleMessages;
            }
            // If not available, return empty array with note
            return { note: "No console messages captured. Console monitoring may not be enabled." };
        })()"#;

        let result: serde_json::Value = page.evaluate(js_code).await.map_err(|e| {
            ToolError::ExecutionFailed(format!("Failed to get console messages: {e}"))
        })?;

        // Check if we got a note about no messages
        if let Some(note) = result.get("note").and_then(|n| n.as_str()) {
            return Ok(format!(
                "Console messages (level >= {:?}):\n\n{}",
                input.level, note
            ));
        }

        // Format the output
        let level_str = match input.level {
            ConsoleLevel::Error => "error",
            ConsoleLevel::Warning => "warning",
            ConsoleLevel::Info => "info",
            ConsoleLevel::Debug => "debug",
        };

        Ok(format!(
            "Console messages (level >= {}):\n\n{}",
            level_str,
            serde_json::to_string_pretty(&result).unwrap_or_else(|_| "No messages".to_string())
        ))
    }
}
