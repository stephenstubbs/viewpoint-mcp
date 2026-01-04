//! Browser console messages tool for retrieving console logs

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::{BrowserState, ConsoleLevel as BrowserConsoleLevel};

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

impl ConsoleLevel {
    /// Convert to browser console level for filtering.
    fn to_browser_level(self) -> BrowserConsoleLevel {
        match self {
            Self::Error => BrowserConsoleLevel::Error,
            Self::Warning => BrowserConsoleLevel::Warning,
            Self::Info => BrowserConsoleLevel::Info,
            Self::Debug => BrowserConsoleLevel::Debug,
        }
    }
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

        // Get active context
        let context = browser
            .active_context()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        // Get console buffer for active page
        let console_buffer = context.active_console_buffer().await.ok_or_else(|| {
            ToolError::BrowserNotAvailable("No active page for console messages".to_string())
        })?;

        // Read messages from buffer
        let buffer = console_buffer.read().await;
        let browser_level = input.level.to_browser_level();
        let messages = buffer.get_messages(browser_level);

        // Format the output
        let level_str = match input.level {
            ConsoleLevel::Error => "error",
            ConsoleLevel::Warning => "warning",
            ConsoleLevel::Info => "info",
            ConsoleLevel::Debug => "debug",
        };

        if messages.is_empty() {
            return Ok(format!(
                "Console messages (level >= {level_str}):\n\nNo messages captured."
            ));
        }

        // Format messages as JSON array for structured output
        let messages_json: Vec<_> = messages
            .iter()
            .map(|m| {
                json!({
                    "type": m.message_type.to_string(),
                    "text": m.text,
                    "timestamp": m.timestamp,
                    "url": m.url,
                    "lineNumber": m.line_number,
                })
            })
            .collect();

        Ok(format!(
            "Console messages (level >= {}):\n\n{}",
            level_str,
            serde_json::to_string_pretty(&messages_json).unwrap_or_else(|_| "[]".to_string())
        ))
    }
}
