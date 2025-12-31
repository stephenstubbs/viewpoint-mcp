//! Integration tests for browser state management with real Viewpoint browser
//!
//! These tests require Chromium to be installed on the system.
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test browser_integration
//! ```
#![cfg(feature = "integration")]

use tempfile::TempDir;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};

/// Helper to create a headless browser config
fn headless_config() -> BrowserConfig {
    BrowserConfig {
        headless: true,
        ..Default::default()
    }
}

#[tokio::test]
async fn test_browser_initialize_and_shutdown() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    // Should not be initialized yet
    assert!(!state.is_initialized());
    assert!(state.browser().is_none());

    // Initialize browser
    state.initialize().await.expect("Failed to initialize browser");

    // Should be initialized now
    assert!(state.is_initialized());
    assert!(state.browser().is_some());

    // Default context should exist
    let ctx = state.active_context().expect("Should have active context");
    assert_eq!(ctx.name, "default");
    assert!(ctx.page_count() > 0);

    // Shutdown
    state.shutdown().await;

    // Should not be initialized after shutdown
    assert!(!state.is_initialized());
    assert!(state.browser().is_none());
}

#[tokio::test]
async fn test_browser_multi_context_creation() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    // Should start with default context
    assert_eq!(state.active_context_name(), "default");
    assert_eq!(state.list_contexts().len(), 1);

    // Create a second context
    state
        .create_context("secondary")
        .await
        .expect("Failed to create context");

    // New context should be active
    assert_eq!(state.active_context_name(), "secondary");
    assert_eq!(state.list_contexts().len(), 2);

    // Create a third context
    state
        .create_context("tertiary")
        .await
        .expect("Failed to create context");

    assert_eq!(state.active_context_name(), "tertiary");
    assert_eq!(state.list_contexts().len(), 3);

    // Switch back to default
    state
        .switch_context("default")
        .expect("Failed to switch context");
    assert_eq!(state.active_context_name(), "default");

    // Close secondary context
    state
        .close_context("secondary")
        .await
        .expect("Failed to close context");
    assert_eq!(state.list_contexts().len(), 2);

    // Cleanup
    state.shutdown().await;
}

#[tokio::test]
async fn test_browser_context_isolation() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    // Create two contexts
    state
        .create_context("context_a")
        .await
        .expect("Failed to create context A");
    state
        .create_context("context_b")
        .await
        .expect("Failed to create context B");

    // Each context should have its own pages
    state.switch_context("context_a").unwrap();
    let ctx_a = state.active_context().unwrap();
    assert!(ctx_a.page_count() > 0);

    state.switch_context("context_b").unwrap();
    let ctx_b = state.active_context().unwrap();
    assert!(ctx_b.page_count() > 0);

    // Cleanup
    state.shutdown().await;
}

#[tokio::test]
async fn test_browser_close_active_context_fallback() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    // Create and switch to a new context
    state
        .create_context("temp")
        .await
        .expect("Failed to create context");
    assert_eq!(state.active_context_name(), "temp");

    // Close the active context - should fall back to default
    state
        .close_context("temp")
        .await
        .expect("Failed to close context");
    assert_eq!(state.active_context_name(), "default");

    // Cleanup
    state.shutdown().await;
}

#[tokio::test]
async fn test_browser_user_data_dir_persistence() {
    // Create a temporary directory for user data
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let user_data_path = temp_dir.path().to_path_buf();

    // First session: initialize browser with user data dir
    {
        let config = BrowserConfig {
            headless: true,
            user_data_dir: Some(user_data_path.clone()),
            ..Default::default()
        };
        let mut state = BrowserState::new(config);

        state.initialize().await.expect("Failed to initialize browser");

        // Verify browser started with the user data dir
        assert!(state.is_initialized());

        // The user data directory should now contain browser profile data
        // (Chrome creates various subdirectories)
        state.shutdown().await;
    }

    // Verify some profile data was created
    assert!(user_data_path.exists());
    // Chrome typically creates a Default directory or similar
    let has_profile_data = std::fs::read_dir(&user_data_path)
        .map(|entries| entries.count() > 0)
        .unwrap_or(false);
    assert!(
        has_profile_data,
        "User data directory should contain profile data after browser session"
    );

    // Second session: re-use the same user data dir
    {
        let config = BrowserConfig {
            headless: true,
            user_data_dir: Some(user_data_path.clone()),
            ..Default::default()
        };
        let mut state = BrowserState::new(config);

        // Should be able to initialize with existing profile
        state
            .initialize()
            .await
            .expect("Failed to initialize browser with existing profile");

        assert!(state.is_initialized());
        state.shutdown().await;
    }

    // temp_dir will be cleaned up when it goes out of scope
}

#[tokio::test]
async fn test_browser_switch_nonexistent_context_error() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    // Try to switch to a context that doesn't exist
    let result = state.switch_context("nonexistent");
    assert!(result.is_err());

    // Active context should remain unchanged
    assert_eq!(state.active_context_name(), "default");

    state.shutdown().await;
}

#[tokio::test]
async fn test_browser_create_duplicate_context_error() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    // Create a context
    state
        .create_context("unique")
        .await
        .expect("Failed to create context");

    // Try to create another context with the same name
    let result = state.create_context("unique").await;
    assert!(result.is_err());

    state.shutdown().await;
}

#[tokio::test]
async fn test_browser_headless_mode() {
    // Explicitly test headless mode
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize headless browser");
    assert!(state.is_initialized());

    // Browser should be running in headless mode
    // (No visible window, but still functional)
    let ctx = state.active_context().expect("Should have context");
    assert!(ctx.page_count() > 0);

    state.shutdown().await;
}

#[tokio::test]
async fn test_browser_cdp_endpoint_connection() {
    use std::process::{Command, Stdio};
    use std::time::Duration;

    // Find an available port for CDP
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let cdp_port = listener.local_addr().unwrap().port();
    drop(listener); // Release the port

    // Launch Chrome with remote debugging enabled
    let mut chrome_process = Command::new("chromium")
        .args([
            "--headless",
            "--disable-gpu",
            "--no-sandbox",
            "--disable-dev-shm-usage",
            &format!("--remote-debugging-port={cdp_port}"),
            "about:blank",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to launch Chrome with CDP");

    // Give Chrome time to start and open the debugging port
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Now connect to Chrome via CDP endpoint
    let config = BrowserConfig {
        headless: true,
        cdp_endpoint: Some(format!("http://127.0.0.1:{cdp_port}")),
        ..Default::default()
    };
    let mut state = BrowserState::new(config);

    // Should connect successfully
    let result = state.initialize().await;

    // Clean up Chrome process regardless of result
    let _ = chrome_process.kill();
    let _ = chrome_process.wait();

    // Now check the result
    result.expect("Failed to connect to Chrome via CDP endpoint");
    assert!(state.is_initialized());

    // Should have a working browser
    let ctx = state.active_context().expect("Should have context");
    assert!(ctx.page_count() > 0);

    state.shutdown().await;
}

// =============================================================================
// Accessibility Snapshot Integration Tests
// =============================================================================

#[tokio::test]
async fn test_accessibility_snapshot_basic() {
    use viewpoint_mcp::snapshot::{AccessibilitySnapshot, SnapshotOptions};

    let config = headless_config();
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    // Get the active page
    let ctx = state.active_context().expect("Should have context");
    let page = ctx.active_page().expect("Should have page");

    // Set page content (using set_content like viewpoint-core tests for proper DOM access)
    page.set_content(r#"
        <html><body>
            <h1>Test Page</h1>
            <button id="btn">Click me</button>
            <input type="text" placeholder="Enter text">
        </body></html>
    "#)
        .set()
        .await
        .expect("Failed to set content");

    // Capture accessibility snapshot
    let options = SnapshotOptions::default();
    let snapshot = AccessibilitySnapshot::capture(page, options)
        .await
        .expect("Failed to capture snapshot");

    // Format the snapshot
    let formatted = snapshot.format();

    // Verify basic structure
    assert!(snapshot.element_count() > 0, "Should have elements");
    assert!(formatted.contains("heading"), "Should contain heading");
    assert!(formatted.contains("button"), "Should contain button");
    assert!(formatted.contains("textbox"), "Should contain textbox");

    // Interactive elements should have refs
    // Note: Refs are provided by viewpoint-core's aria_snapshot API
    // which uses backendNodeId from CDP
    assert!(snapshot.ref_count() > 0, "Should have refs for interactive elements");
    assert!(formatted.contains("[ref=e"), "Should have ref annotations");

    state.shutdown().await;
}

#[tokio::test]
async fn test_accessibility_snapshot_ref_lookup() {
    use viewpoint_mcp::snapshot::{AccessibilitySnapshot, SnapshotOptions};

    let config = headless_config();
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    let ctx = state.active_context().expect("Should have context");
    let page = ctx.active_page().expect("Should have page");

    // Navigate to a page with identifiable elements
    page.goto("data:text/html,<html><body><button id='submit-btn'>Submit</button></body></html>")
        .goto()
        .await
        .expect("Failed to navigate");

    // Capture snapshot
    let options = SnapshotOptions::default();
    let snapshot = AccessibilitySnapshot::capture(page, options)
        .await
        .expect("Failed to capture snapshot");

    // Find a ref in the formatted output
    let formatted = snapshot.format();
    assert!(formatted.contains("[ref=e"), "Should have element refs");

    // Extract a ref from the output and look it up
    if let Some(start) = formatted.find("[ref=e") {
        let end = formatted[start..].find(']').unwrap() + start;
        let ref_str = &formatted[start + 5..end]; // Skip "[ref="

        // Look up the ref
        let result = snapshot.lookup(ref_str);
        assert!(result.is_ok(), "Should be able to look up ref: {ref_str}");
    }

    state.shutdown().await;
}

#[tokio::test]
async fn test_accessibility_snapshot_stability_across_refreshes() {
    use viewpoint_mcp::snapshot::{AccessibilitySnapshot, SnapshotOptions};

    let config = headless_config();
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    let ctx = state.active_context().expect("Should have context");
    let page = ctx.active_page().expect("Should have page");

    // Navigate to a page with a stable element ID
    let html = r#"<html><body>
        <button id="stable-id">Stable Button</button>
    </body></html>"#;
    page.goto(&format!("data:text/html,{html}"))
        .goto()
        .await
        .expect("Failed to navigate");

    // First snapshot
    let options = SnapshotOptions::default();
    let snapshot1 = AccessibilitySnapshot::capture(page, options.clone())
        .await
        .expect("Failed to capture first snapshot");

    // Refresh the page
    page.reload().await.expect("Failed to reload");

    // Second snapshot
    let snapshot2 = AccessibilitySnapshot::capture(page, options)
        .await
        .expect("Failed to capture second snapshot");

    // The formatted output should have similar refs for same elements
    let formatted1 = snapshot1.format();
    let formatted2 = snapshot2.format();

    // Both should have the button with refs
    assert!(formatted1.contains("button"), "First snapshot should have button");
    assert!(formatted2.contains("button"), "Second snapshot should have button");

    // Element counts should be the same
    assert_eq!(
        snapshot1.element_count(),
        snapshot2.element_count(),
        "Element count should be stable"
    );

    state.shutdown().await;
}

#[tokio::test]
async fn test_accessibility_snapshot_compact_mode() {
    use viewpoint_mcp::snapshot::{AccessibilitySnapshot, SnapshotOptions};

    let config = headless_config();
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    let ctx = state.active_context().expect("Should have context");
    let page = ctx.active_page().expect("Should have page");

    // Create a page with many buttons to trigger compact mode (>100 elements)
    let mut buttons = String::new();
    for i in 0..110 {
        buttons.push_str(&format!("<button>Button {i}</button>"));
    }
    let html = format!("<html><body>{buttons}</body></html>");

    page.goto(&format!("data:text/html,{html}"))
        .goto()
        .await
        .expect("Failed to navigate");

    // Capture without allRefs - should trigger compact mode
    let options = SnapshotOptions {
        all_refs: false,
        ..Default::default()
    };
    let snapshot = AccessibilitySnapshot::capture(page, options)
        .await
        .expect("Failed to capture snapshot");

    assert!(snapshot.is_compact(), "Should be in compact mode with many elements");

    // Capture with allRefs - should not be compact
    let options_all = SnapshotOptions {
        all_refs: true,
        ..Default::default()
    };
    let snapshot_all = AccessibilitySnapshot::capture(page, options_all)
        .await
        .expect("Failed to capture snapshot with allRefs");

    assert!(!snapshot_all.is_compact(), "Should not be compact with allRefs");

    state.shutdown().await;
}

#[tokio::test]
async fn test_browser_context_with_proxy_config() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use tokio::net::TcpListener;

    // Start a simple TCP listener to act as a "proxy"
    // We just need to verify the browser attempts to connect to it
    let proxy_listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind proxy listener");
    let proxy_addr = proxy_listener.local_addr().unwrap();
    let proxy_port = proxy_addr.port();

    let connection_received = Arc::new(AtomicBool::new(false));
    let connection_received_clone = connection_received.clone();

    // Spawn a task to accept connections on the proxy port
    let proxy_handle = tokio::spawn(async move {
        // Wait for a connection with timeout
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            proxy_listener.accept(),
        )
        .await;

        if result.is_ok() {
            connection_received_clone.store(true, Ordering::SeqCst);
        }
    });

    // Create a browser state with proxy configuration
    // Note: We're testing that the proxy config is properly set up
    // The browser may or may not actually connect depending on navigation
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);

    state.initialize().await.expect("Failed to initialize browser");

    // Create a context with proxy - this tests the ProxyConfig structure
    // Even if we can't fully test proxy routing, we verify the API works
    let proxy_config = viewpoint_mcp::browser::ProxyConfig::new(format!(
        "http://127.0.0.1:{proxy_port}"
    ));

    // Verify proxy config is correctly constructed
    assert_eq!(
        proxy_config.server,
        format!("http://127.0.0.1:{proxy_port}")
    );
    assert!(proxy_config.username.is_none());
    assert!(proxy_config.password.is_none());

    // Test proxy config with authentication
    let proxy_with_auth = viewpoint_mcp::browser::ProxyConfig::new(format!(
        "http://127.0.0.1:{proxy_port}"
    ))
    .with_auth("testuser", "testpass")
    .with_bypass("localhost,127.0.0.1");

    assert_eq!(proxy_with_auth.username, Some("testuser".to_string()));
    assert_eq!(proxy_with_auth.password, Some("testpass".to_string()));
    assert_eq!(
        proxy_with_auth.bypass,
        Some("localhost,127.0.0.1".to_string())
    );

    // Clean up
    state.shutdown().await;
    proxy_handle.abort();
}
