//! Integration tests for inspection tools (snapshot, screenshot, console, network)
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test inspection
//! ```
#![cfg(feature = "integration")]

mod inspection {
    pub mod console_network_tests;
    pub mod screenshot_tests;
    pub mod snapshot_tests;

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
