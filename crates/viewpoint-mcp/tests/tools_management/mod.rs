//! Integration tests for management tools (tabs, resize, close, dialog, install)
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test tools_management
//! ```
#![cfg(feature = "integration")]

mod tabs_tests;
mod resize_tests;
mod close_tests;
mod dialog_tests;
mod install_tests;
mod integration_tests;

use viewpoint_mcp::browser::{BrowserConfig, BrowserState};

/// Helper to create a headless browser state
pub async fn create_browser() -> BrowserState {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);
    state
        .initialize()
        .await
        .expect("Failed to initialize browser");
    state
}
