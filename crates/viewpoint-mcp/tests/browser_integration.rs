//! Integration tests for browser state management with real Viewpoint browser
//!
//! These tests require Chromium to be installed on the system.
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test browser_integration
//! ```
#![cfg(feature = "integration")]

mod browser_integration {
    pub mod snapshot_tests;
    pub mod state_tests;

    use viewpoint_mcp::browser::BrowserConfig;

    /// Helper to create a headless browser config
    pub fn headless_config() -> BrowserConfig {
        BrowserConfig {
            headless: true,
            ..Default::default()
        }
    }
}
