//! Browser tabs tool for tab management

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser tabs tool - list, create, close, or select tabs
pub struct BrowserTabsTool;

/// Input parameters for `browser_tabs`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserTabsInput {
    /// Action to perform: "list", "new", "close", or "select"
    pub action: TabAction,

    /// Tab index for close/select operations
    pub index: Option<usize>,
}

/// Tab actions
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TabAction {
    /// List all open tabs
    List,
    /// Create a new tab
    New,
    /// Close a tab (by index, or current if not specified)
    Close,
    /// Select/switch to a tab by index
    Select,
}

impl BrowserTabsTool {
    /// Create a new browser tabs tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserTabsTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserTabsTool {
    fn name(&self) -> &'static str {
        "browser_tabs"
    }

    fn description(&self) -> &'static str {
        "Manage browser tabs. Actions: 'list' shows all tabs, 'new' creates a tab, \
         'close' closes a tab by index (or current), 'select' switches to a tab by index."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["action"],
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "new", "close", "select"],
                    "description": "Operation to perform on tabs"
                },
                "index": {
                    "type": "number",
                    "description": "Tab index for close/select operations. If omitted for close, closes the current tab."
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserTabsInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Ensure browser is initialized
        browser
            .initialize()
            .await
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        match input.action {
            TabAction::List => self.list_tabs(browser).await,
            TabAction::New => self.new_tab(browser).await,
            TabAction::Close => self.close_tab(browser, input.index).await,
            TabAction::Select => self.select_tab(browser, input.index).await,
        }
    }
}

impl BrowserTabsTool {
    async fn list_tabs(&self, browser: &BrowserState) -> ToolResult {
        use std::fmt::Write;

        let context = browser
            .active_context()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        let pages = context
            .pages()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get pages: {e}")))?;

        let page_count = pages.len();
        let active_index = context.active_page_index().await;

        if page_count == 0 {
            return Ok("No tabs open".to_string());
        }

        let mut result = format!("Tabs ({page_count} total):\n");

        for (i, page) in pages.iter().enumerate() {
            let marker = if i == active_index { " [active]" } else { "" };
            let url = page.url().await.unwrap_or_else(|_| "unknown".to_string());
            let _ = writeln!(result, "  {i}: {url}{marker}");
        }

        Ok(result.trim_end().to_string())
    }

    async fn new_tab(&self, browser: &mut BrowserState) -> ToolResult {
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        context
            .new_page()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create new tab: {e}")))?;

        let new_index = context.active_page_index().await;
        let page_count = context
            .page_count()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get page count: {e}")))?;

        // Invalidate cache for new tab
        context.invalidate_cache();

        Ok(format!(
            "Created new tab at index {new_index} ({page_count} tabs total)"
        ))
    }

    async fn close_tab(&self, browser: &mut BrowserState, index: Option<usize>) -> ToolResult {
        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        let page_count = context
            .page_count()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get page count: {e}")))?;
        if page_count == 0 {
            return Err(ToolError::BrowserNotAvailable(
                "No tabs to close".to_string(),
            ));
        }

        let target_index = match index {
            Some(i) => i,
            None => context.active_page_index().await,
        };

        if target_index >= page_count {
            return Err(ToolError::InvalidParams(format!(
                "Tab index {target_index} out of range (0-{})",
                page_count - 1
            )));
        }

        context
            .close_page(target_index)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to close tab: {e}")))?;

        // Invalidate cache
        context.invalidate_cache();

        let remaining = page_count - 1;
        Ok(format!(
            "Closed tab at index {target_index} ({remaining} tabs remaining)"
        ))
    }

    async fn select_tab(&self, browser: &mut BrowserState, index: Option<usize>) -> ToolResult {
        let index = index.ok_or_else(|| {
            ToolError::InvalidParams("index is required for select action".to_string())
        })?;

        let context = browser
            .active_context_mut()
            .map_err(|e| ToolError::BrowserNotAvailable(e.to_string()))?;

        let page_count = context
            .page_count()
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to get page count: {e}")))?;
        if index >= page_count {
            return Err(ToolError::InvalidParams(format!(
                "Tab index {index} out of range (0-{})",
                page_count.saturating_sub(1)
            )));
        }

        let switched = context.switch_page(index).await;

        if switched {
            // Invalidate cache when switching tabs
            context.invalidate_cache();
            Ok(format!("Switched to tab at index {index}"))
        } else {
            Err(ToolError::ExecutionFailed(format!(
                "Failed to switch to tab at index {index}"
            )))
        }
    }
}
