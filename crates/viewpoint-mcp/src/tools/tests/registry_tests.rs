//! Tests for tool registry

use crate::browser::BrowserState;
use crate::tools::registry::ToolRegistry;
use crate::tools::traits::Capability;
use crate::tools::{Tool, ToolError, ToolResult};
use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;

struct MockTool {
    name: &'static str,
    capability: Option<Capability>,
}

#[async_trait]
impl Tool for MockTool {
    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        "Mock tool for testing"
    }

    fn input_schema(&self) -> Value {
        json!({"type": "object"})
    }

    fn required_capability(&self) -> Option<Capability> {
        self.capability
    }

    async fn execute(&self, _args: &Value, _browser: &mut BrowserState) -> ToolResult {
        Err(ToolError::ExecutionFailed("Mock".to_string()))
    }
}

#[test]
fn test_registry_without_capabilities() {
    let mut registry = ToolRegistry::new();

    // Register tools with different capability requirements
    registry.register(Arc::new(MockTool {
        name: "basic_tool",
        capability: None,
    }));
    registry.register(Arc::new(MockTool {
        name: "vision_tool",
        capability: Some(Capability::Vision),
    }));
    registry.register(Arc::new(MockTool {
        name: "pdf_tool",
        capability: Some(Capability::Pdf),
    }));

    // Without capabilities enabled, only basic tool is available
    assert!(registry.get("basic_tool").is_some());
    assert!(registry.get("vision_tool").is_none());
    assert!(registry.get("pdf_tool").is_none());

    // list() should only return available tools
    let available = registry.list();
    assert_eq!(available.len(), 1);
    assert_eq!(available[0].name(), "basic_tool");

    // list_all() returns all tools
    assert_eq!(registry.list_all().len(), 3);
}

#[test]
fn test_registry_with_vision_capability() {
    let mut registry = ToolRegistry::with_capabilities([Capability::Vision]);

    registry.register(Arc::new(MockTool {
        name: "basic_tool",
        capability: None,
    }));
    registry.register(Arc::new(MockTool {
        name: "vision_tool",
        capability: Some(Capability::Vision),
    }));
    registry.register(Arc::new(MockTool {
        name: "pdf_tool",
        capability: Some(Capability::Pdf),
    }));

    // With vision enabled, basic and vision tools are available
    assert!(registry.get("basic_tool").is_some());
    assert!(registry.get("vision_tool").is_some());
    assert!(registry.get("pdf_tool").is_none());

    let available = registry.list();
    assert_eq!(available.len(), 2);
}

#[test]
fn test_registry_with_all_capabilities() {
    let mut registry = ToolRegistry::with_capabilities([Capability::Vision, Capability::Pdf]);

    registry.register(Arc::new(MockTool {
        name: "basic_tool",
        capability: None,
    }));
    registry.register(Arc::new(MockTool {
        name: "vision_tool",
        capability: Some(Capability::Vision),
    }));
    registry.register(Arc::new(MockTool {
        name: "pdf_tool",
        capability: Some(Capability::Pdf),
    }));

    // All tools available
    assert!(registry.get("basic_tool").is_some());
    assert!(registry.get("vision_tool").is_some());
    assert!(registry.get("pdf_tool").is_some());

    assert_eq!(registry.list().len(), 3);
}

#[test]
fn test_enable_capability() {
    let mut registry = ToolRegistry::new();

    registry.register(Arc::new(MockTool {
        name: "pdf_tool",
        capability: Some(Capability::Pdf),
    }));

    assert!(registry.get("pdf_tool").is_none());

    registry.enable_capability(Capability::Pdf);

    assert!(registry.get("pdf_tool").is_some());
}

#[test]
fn test_get_unchecked() {
    let mut registry = ToolRegistry::new();

    registry.register(Arc::new(MockTool {
        name: "vision_tool",
        capability: Some(Capability::Vision),
    }));

    // get() returns None because capability not enabled
    assert!(registry.get("vision_tool").is_none());

    // get_unchecked() returns the tool anyway
    assert!(registry.get_unchecked("vision_tool").is_some());

    // Unknown tool returns None for both
    assert!(registry.get("unknown").is_none());
    assert!(registry.get_unchecked("unknown").is_none());
}

#[test]
fn test_capability_from_str() {
    assert_eq!("vision".parse::<Capability>(), Ok(Capability::Vision));
    assert_eq!("Vision".parse::<Capability>(), Ok(Capability::Vision));
    assert_eq!("VISION".parse::<Capability>(), Ok(Capability::Vision));
    assert_eq!("pdf".parse::<Capability>(), Ok(Capability::Pdf));
    assert_eq!("PDF".parse::<Capability>(), Ok(Capability::Pdf));
    assert!("unknown".parse::<Capability>().is_err());
    assert!("".parse::<Capability>().is_err());
}

#[test]
fn test_capability_as_str() {
    assert_eq!(Capability::Vision.as_str(), "vision");
    assert_eq!(Capability::Pdf.as_str(), "pdf");
}
