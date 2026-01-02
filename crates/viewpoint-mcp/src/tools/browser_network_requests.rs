//! Browser network requests tool for listing network requests

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser network requests tool - lists network requests since page load
pub struct BrowserNetworkRequestsTool;

/// Input parameters for `browser_network_requests`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserNetworkRequestsInput {
    /// Whether to include static resources (images, fonts, scripts)
    #[serde(default)]
    pub include_static: bool,
}

impl BrowserNetworkRequestsTool {
    /// Create a new browser network requests tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserNetworkRequestsTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserNetworkRequestsTool {
    fn name(&self) -> &'static str {
        "browser_network_requests"
    }

    fn description(&self) -> &'static str {
        "Returns all network requests made since loading the page. By default, excludes \
         successful static resources (images, fonts, scripts). Set includeStatic: true \
         to see all requests."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "includeStatic": {
                    "type": "boolean",
                    "default": false,
                    "description": "Include successful static resources like images, fonts, scripts"
                }
            }
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input
        let input: BrowserNetworkRequestsInput = serde_json::from_value(args.clone())
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

        // Use JavaScript to retrieve network requests via Performance API
        // This gives us the resource timing entries
        let include_static = input.include_static;
        let js_code = format!(
            r"(() => {{
            const entries = performance.getEntriesByType('resource');
            const staticTypes = ['img', 'font', 'stylesheet', 'script'];
            
            const requests = entries.map(entry => {{
                let resourceType = 'other';
                if (entry.initiatorType) {{
                    resourceType = entry.initiatorType;
                }}
                
                return {{
                    url: entry.name,
                    type: resourceType,
                    duration: Math.round(entry.duration),
                    size: entry.transferSize || 0,
                    status: entry.responseStatus || null
                }};
            }});
            
            const includeStatic = {include_static};
            if (includeStatic) {{
                return requests;
            }}
            
            // Filter out successful static resources
            return requests.filter(r => {{
                const isStatic = staticTypes.includes(r.type);
                const isSuccess = r.status === null || (r.status >= 200 && r.status < 400);
                return !(isStatic && isSuccess);
            }});
        }})()"
        );

        let result: serde_json::Value = page.evaluate(&js_code).await.map_err(|e| {
            ToolError::ExecutionFailed(format!("Failed to get network requests: {e}"))
        })?;

        // Format output
        let requests = result.as_array().map_or(0, Vec::len);

        if requests == 0 {
            return Ok("No network requests recorded.".to_string());
        }

        Ok(format!(
            "Network requests ({} total{}):\n\n{}",
            requests,
            if input.include_static {
                ""
            } else {
                ", excluding static resources"
            },
            serde_json::to_string_pretty(&result).unwrap_or_else(|_| "[]".to_string())
        ))
    }
}
