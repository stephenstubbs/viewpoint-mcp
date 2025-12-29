//! Tool registry for managing available tools

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::traits::{Capability, Tool};

/// Registry of available MCP tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
    enabled_capabilities: HashSet<Capability>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            enabled_capabilities: HashSet::new(),
        }
    }

    /// Create a new tool registry with specified capabilities enabled
    #[must_use]
    pub fn with_capabilities(capabilities: impl IntoIterator<Item = Capability>) -> Self {
        Self {
            tools: HashMap::new(),
            enabled_capabilities: capabilities.into_iter().collect(),
        }
    }

    /// Enable a capability
    pub fn enable_capability(&mut self, capability: Capability) {
        self.enabled_capabilities.insert(capability);
    }

    /// Check if a capability is enabled
    #[must_use]
    pub fn is_capability_enabled(&self, capability: Capability) -> bool {
        self.enabled_capabilities.contains(&capability)
    }

    /// Get the set of enabled capabilities
    #[must_use]
    pub fn enabled_capabilities(&self) -> &HashSet<Capability> {
        &self.enabled_capabilities
    }

    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    /// Get a tool by name, respecting capability requirements
    ///
    /// Returns `None` if the tool doesn't exist or if it requires
    /// a capability that isn't enabled.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name).filter(|tool| self.is_tool_available(tool))
    }

    /// Get a tool by name, ignoring capability requirements
    ///
    /// This is useful for error messages that need to distinguish
    /// between "unknown tool" and "capability not enabled".
    #[must_use]
    pub fn get_unchecked(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name)
    }

    /// Check if a tool is available based on its capability requirements
    #[must_use]
    pub fn is_tool_available(&self, tool: &Arc<dyn Tool>) -> bool {
        match tool.required_capability() {
            None => true,
            Some(cap) => self.enabled_capabilities.contains(&cap),
        }
    }

    /// List all registered tools that are currently available
    ///
    /// Tools requiring capabilities that aren't enabled are excluded.
    #[must_use]
    pub fn list(&self) -> Vec<&Arc<dyn Tool>> {
        self.tools
            .values()
            .filter(|tool| self.is_tool_available(tool))
            .collect()
    }

    /// List all registered tools, regardless of capability requirements
    #[must_use]
    pub fn list_all(&self) -> Vec<&Arc<dyn Tool>> {
        self.tools.values().collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Register all browser tools with the registry
///
/// This function registers all 30 browser tools:
/// - 27 core tools (always available)
/// - 3 vision tools (require Vision capability)
/// - 1 PDF tool (requires Pdf capability)
pub fn register_all_tools(registry: &mut ToolRegistry) {
    use std::sync::Arc;

    // Navigation tools (2)
    registry.register(Arc::new(super::BrowserNavigateTool::new()));
    registry.register(Arc::new(super::BrowserNavigateBackTool::new()));

    // Interaction tools (8)
    registry.register(Arc::new(super::BrowserClickTool::new()));
    registry.register(Arc::new(super::BrowserDragTool::new()));
    registry.register(Arc::new(super::BrowserFileUploadTool::new()));
    registry.register(Arc::new(super::BrowserFillFormTool::new()));
    registry.register(Arc::new(super::BrowserHoverTool::new()));
    registry.register(Arc::new(super::BrowserPressKeyTool::new()));
    registry.register(Arc::new(super::BrowserSelectOptionTool::new()));
    registry.register(Arc::new(super::BrowserTypeTool::new()));

    // Inspection tools (4)
    registry.register(Arc::new(super::BrowserConsoleMessagesTool::new()));
    registry.register(Arc::new(super::BrowserNetworkRequestsTool::new()));
    registry.register(Arc::new(super::BrowserSnapshotTool::new()));
    registry.register(Arc::new(super::BrowserTakeScreenshotTool::new()));

    // State tools (3)
    registry.register(Arc::new(super::BrowserEvaluateTool::new()));
    registry.register(Arc::new(super::BrowserHandleDialogTool::new()));
    registry.register(Arc::new(super::BrowserWaitForTool::new()));

    // Management tools (4)
    registry.register(Arc::new(super::BrowserCloseTool::new()));
    registry.register(Arc::new(super::BrowserInstallTool::new()));
    registry.register(Arc::new(super::BrowserResizeTool::new()));
    registry.register(Arc::new(super::BrowserTabsTool::new()));

    // Context management tools (5)
    registry.register(Arc::new(super::BrowserContextCloseTool::new()));
    registry.register(Arc::new(super::BrowserContextCreateTool::new()));
    registry.register(Arc::new(super::BrowserContextListTool::new()));
    registry.register(Arc::new(super::BrowserContextSaveStorageTool::new()));
    registry.register(Arc::new(super::BrowserContextSwitchTool::new()));

    // Optional vision tools (3) - require Vision capability
    registry.register(Arc::new(super::BrowserMouseClickXyTool::new()));
    registry.register(Arc::new(super::BrowserMouseDragXyTool::new()));
    registry.register(Arc::new(super::BrowserMouseMoveXyTool::new()));

    // Optional PDF tool (1) - requires Pdf capability
    registry.register(Arc::new(super::BrowserPdfSaveTool::new()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser::BrowserState;
    use crate::tools::{ToolError, ToolResult};
    use async_trait::async_trait;
    use serde_json::{json, Value};

    struct MockTool {
        name: &'static str,
        capability: Option<Capability>,
    }

    #[async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            self.name
        }

        fn description(&self) -> &str {
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
        let mut registry =
            ToolRegistry::with_capabilities([Capability::Vision, Capability::Pdf]);

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
        assert_eq!(Capability::from_str("vision"), Some(Capability::Vision));
        assert_eq!(Capability::from_str("Vision"), Some(Capability::Vision));
        assert_eq!(Capability::from_str("VISION"), Some(Capability::Vision));
        assert_eq!(Capability::from_str("pdf"), Some(Capability::Pdf));
        assert_eq!(Capability::from_str("PDF"), Some(Capability::Pdf));
        assert_eq!(Capability::from_str("unknown"), None);
        assert_eq!(Capability::from_str(""), None);
    }

    #[test]
    fn test_capability_as_str() {
        assert_eq!(Capability::Vision.as_str(), "vision");
        assert_eq!(Capability::Pdf.as_str(), "pdf");
    }
}
