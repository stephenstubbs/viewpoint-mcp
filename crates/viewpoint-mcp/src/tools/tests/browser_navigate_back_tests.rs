//! Tests for `browser_navigate_back` tool

use crate::tools::Tool;
use crate::tools::browser_navigate_back::BrowserNavigateBackTool;

#[test]
fn test_tool_metadata() {
    let tool = BrowserNavigateBackTool::new();

    assert_eq!(tool.name(), "browser_navigate_back");
    assert!(!tool.description().is_empty());

    let schema = tool.input_schema();
    assert_eq!(schema["type"], "object");
}
