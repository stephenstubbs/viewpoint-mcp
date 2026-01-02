//! Browser install tool for installing the browser if missing

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Tool, ToolError, ToolResult};
use crate::browser::BrowserState;

/// Browser install tool - checks for and installs the browser
pub struct BrowserInstallTool;

/// Input parameters for `browser_install`
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserInstallInput {
    // No required parameters
}

impl BrowserInstallTool {
    /// Create a new browser install tool
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BrowserInstallTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for BrowserInstallTool {
    fn name(&self) -> &'static str {
        "browser_install"
    }

    fn description(&self) -> &'static str {
        "Install the browser if it is not already installed. Call this if you get an error \
         about the browser not being installed. This will download and set up the browser \
         executable required for automation."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }

    async fn execute(&self, args: &Value, browser: &mut BrowserState) -> ToolResult {
        // Parse input (validates the JSON structure even though there are no params)
        let _input: BrowserInstallInput = serde_json::from_value(args.clone())
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;

        // Check if browser is already working by attempting to initialize
        // If it works, the browser is already installed
        if browser.is_initialized() {
            return Ok(
                "Browser is already installed and running. No installation needed.".to_string(),
            );
        }

        // Try to initialize - this will launch the browser
        match browser.initialize().await {
            Ok(()) => Ok("Browser is already installed and successfully initialized.".to_string()),
            Err(e) => {
                // Browser launch failed - attempt installation
                let error_msg = e.to_string();

                // Check if the error indicates the browser is not installed
                if error_msg.contains("not found")
                    || error_msg.contains("not installed")
                    || error_msg.contains("executable")
                    || error_msg.contains("No such file")
                {
                    // Provide manual installation instructions
                    Ok("Browser installation required.\n\
                        To install the browser manually, run:\n\
                        \n\
                        npx playwright install chromium\n\
                        \n\
                        Or install Playwright browsers with:\n\
                        \n\
                        npx playwright install\n\
                        \n\
                        After installation, browser tools should work correctly."
                        .to_string())
                } else {
                    // Some other error - might be a connection issue or config problem
                    Err(ToolError::ExecutionFailed(format!(
                        "Failed to initialize browser: {error_msg}. \
                         If the browser is not installed, the error message should indicate that."
                    )))
                }
            }
        }
    }
}
