//! Integration tests for stale reference detection and multi-context refs
//!
//! These tests verify that the system properly:
//! - Detects stale references when elements change or are removed
//! - Handles context-prefixed refs correctly
//! - Isolates refs between different browser contexts
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test stale_refs
//! ```
#![cfg(feature = "integration")]

mod stale_refs {
    pub mod detection;
    pub mod edge_cases;
    pub mod multi_context;

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

    /// Extract the first ref from a snapshot string
    /// New format: c{ctx}p{page}f{frame}e{counter} (e.g., c0p0f0e1)
    pub fn extract_first_ref(snapshot: &str) -> Option<String> {
        let re = regex::Regex::new(r"\[ref=(c\d+p\d+f\d+e\d+)\]").unwrap();
        re.captures(snapshot)
            .map(|c| c.get(1).unwrap().as_str().to_string())
    }

    /// Extract a ref from a snapshot (alias for extract_first_ref with same behavior)
    /// New format: c{ctx}p{page}f{frame}e{counter} (e.g., c0p0f0e1)
    pub fn extract_ref(snapshot: &str) -> Option<String> {
        // Look for refs in the new format
        let re = regex::Regex::new(r"\[ref=(c\d+p\d+f\d+e\d+)\]").unwrap();
        re.captures(snapshot)
            .map(|c| c.get(1).unwrap().as_str().to_string())
    }
}
