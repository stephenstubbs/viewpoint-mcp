//! Integration tests for management tools (tabs, resize, close, dialog, install)
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test tools_management
//! ```
#![cfg(feature = "integration")]

mod tools_management {
    pub mod close_tests;
    pub mod dialog_tests;
    pub mod install_tests;
    pub mod integration_tests;
    pub mod resize_tests;
    pub mod tabs_tests;

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
}
