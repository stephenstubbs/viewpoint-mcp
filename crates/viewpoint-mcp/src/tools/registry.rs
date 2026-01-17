//! Tool registry for managing available tools
//!
//! The registry tracks all registered tools and their capability requirements,
//! providing filtered access based on enabled capabilities.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::traits::{Capability, Tool};

/// Registry of available MCP tools.
///
/// The registry stores tool implementations and tracks which capabilities
/// are enabled. Tools requiring disabled capabilities are hidden from
/// listing and lookup.
///
/// # Examples
///
/// ```
/// use viewpoint_mcp::tools::{ToolRegistry, Capability, register_all_tools};
///
/// // Create registry with vision capability enabled
/// let mut registry = ToolRegistry::with_capabilities([Capability::Vision]);
/// register_all_tools(&mut registry);
///
/// // Only tools available with current capabilities are listed
/// let tools = registry.list();
/// assert!(tools.iter().any(|t| t.name() == "browser_mouse_click_xy"));
///
/// // Without vision capability, mouse tools are hidden
/// let mut registry = ToolRegistry::new();
/// register_all_tools(&mut registry);
/// let tools = registry.list();
/// assert!(!tools.iter().any(|t| t.name() == "browser_mouse_click_xy"));
/// ```
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
        self.tools
            .get(name)
            .filter(|tool| self.is_tool_available(tool))
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
/// This function registers all 31 browser tools:
/// - 28 core tools (always available)
/// - 3 vision tools (require Vision capability)
/// - 1 PDF tool (requires Pdf capability)
pub fn register_all_tools(registry: &mut ToolRegistry) {
    use std::sync::Arc;

    // Navigation tools (2)
    registry.register(Arc::new(super::BrowserNavigateTool::new()));
    registry.register(Arc::new(super::BrowserNavigateBackTool::new()));

    // Interaction tools (9)
    registry.register(Arc::new(super::BrowserClickTool::new()));
    registry.register(Arc::new(super::BrowserDragTool::new()));
    registry.register(Arc::new(super::BrowserFileUploadTool::new()));
    registry.register(Arc::new(super::BrowserFillFormTool::new()));
    registry.register(Arc::new(super::BrowserHoverTool::new()));
    registry.register(Arc::new(super::BrowserPressKeyTool::new()));
    registry.register(Arc::new(super::BrowserScrollIntoViewTool::new()));
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
