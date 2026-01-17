//! Tool framework for MCP
//!
//! This module provides the tool trait and registry for MCP tool definitions.

#[cfg(test)]
mod tests;

// Navigation tools
mod browser_navigate;
mod browser_navigate_back;

// Interaction tools
mod browser_click;
mod browser_drag;
mod browser_file_upload;
mod browser_fill_form;
mod browser_hover;
mod browser_press_key;
mod browser_scroll_into_view;
mod browser_select_option;
mod browser_type;

// Inspection tools
mod browser_console_messages;
mod browser_network_requests;
mod browser_snapshot;
mod browser_take_screenshot;

// State tools
mod browser_evaluate;
mod browser_handle_dialog;
mod browser_wait_for;

// Management tools
mod browser_close;
mod browser_install;
mod browser_resize;
mod browser_tabs;

// Context management tools
mod browser_context_close;
mod browser_context_create;
mod browser_context_list;
mod browser_context_save_storage;
mod browser_context_switch;

// Optional capability tools (vision)
mod browser_mouse_click_xy;
mod browser_mouse_drag_xy;
mod browser_mouse_move_xy;

// Optional capability tools (pdf)
mod browser_pdf_save;

// Framework
mod error;
mod registry;
mod traits;

// Re-export navigation tools
pub use browser_navigate::BrowserNavigateTool;
pub use browser_navigate_back::BrowserNavigateBackTool;

// Re-export interaction tools
pub use browser_click::BrowserClickTool;
pub use browser_drag::BrowserDragTool;
pub use browser_file_upload::BrowserFileUploadTool;
pub use browser_fill_form::BrowserFillFormTool;
pub use browser_hover::BrowserHoverTool;
pub use browser_press_key::BrowserPressKeyTool;
pub use browser_scroll_into_view::BrowserScrollIntoViewTool;
pub use browser_select_option::BrowserSelectOptionTool;
pub use browser_type::BrowserTypeTool;

// Re-export inspection tools
pub use browser_console_messages::BrowserConsoleMessagesTool;
pub use browser_network_requests::BrowserNetworkRequestsTool;
pub use browser_snapshot::BrowserSnapshotTool;
pub use browser_take_screenshot::BrowserTakeScreenshotTool;

// Re-export state tools
pub use browser_evaluate::BrowserEvaluateTool;
pub use browser_handle_dialog::BrowserHandleDialogTool;
pub use browser_wait_for::BrowserWaitForTool;

// Re-export management tools
pub use browser_close::BrowserCloseTool;
pub use browser_install::BrowserInstallTool;
pub use browser_resize::BrowserResizeTool;
pub use browser_tabs::BrowserTabsTool;

// Re-export context management tools
pub use browser_context_close::BrowserContextCloseTool;
pub use browser_context_create::BrowserContextCreateTool;
pub use browser_context_list::BrowserContextListTool;
pub use browser_context_save_storage::BrowserContextSaveStorageTool;
pub use browser_context_switch::BrowserContextSwitchTool;

// Re-export optional vision tools
pub use browser_mouse_click_xy::BrowserMouseClickXyTool;
pub use browser_mouse_drag_xy::BrowserMouseDragXyTool;
pub use browser_mouse_move_xy::BrowserMouseMoveXyTool;

// Re-export optional PDF tools
pub use browser_pdf_save::BrowserPdfSaveTool;

// Re-export framework types
pub use error::ToolError;
pub use registry::{ToolRegistry, register_all_tools};
pub use traits::{Capability, Tool, ToolResult};

// Re-export tool output types from server module for convenience
pub use crate::server::{ContentItem, ToolOutput};

/// Result type for tool operations
pub type Result<T> = std::result::Result<T, ToolError>;
