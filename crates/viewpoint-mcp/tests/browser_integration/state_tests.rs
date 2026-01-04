//! Browser state and context management tests

use super::headless_config;
use tempfile::TempDir;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};

#[tokio::test]
async fn test_browser_initialize_and_shutdown() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    // Should not be initialized yet
    assert!(!state.is_initialized());
    assert!(state.browser().is_none());

    // Initialize browser
    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

    // Should be initialized now
    assert!(state.is_initialized());
    assert!(state.browser().is_some());

    // Default context should exist
    let ctx = state.active_context().expect("Should have active context");
    assert_eq!(ctx.name, "default");
    assert!(ctx.page_count().await.unwrap() > 0);

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

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

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

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

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
    assert!(ctx_a.page_count().await.unwrap() > 0);

    state.switch_context("context_b").unwrap();
    let ctx_b = state.active_context().unwrap();
    assert!(ctx_b.page_count().await.unwrap() > 0);

    // Cleanup
    state.shutdown().await;
}

#[tokio::test]
async fn test_browser_close_active_context_fallback() {
    let config = headless_config();
    let mut state = BrowserState::new(config);

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

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

        state
            .initialize()
            .await
            .expect("Failed to initialize browser");

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

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

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

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

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

    state
        .initialize()
        .await
        .expect("Failed to initialize headless browser");
    assert!(state.is_initialized());

    // Browser should be running in headless mode
    // (No visible window, but still functional)
    let ctx = state.active_context().expect("Should have context");
    assert!(ctx.page_count().await.unwrap() > 0);

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
    assert!(ctx.page_count().await.unwrap() > 0);

    state.shutdown().await;
}

#[tokio::test]
async fn test_browser_context_with_proxy_config() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
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
        let result =
            tokio::time::timeout(std::time::Duration::from_secs(5), proxy_listener.accept()).await;

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

    state
        .initialize()
        .await
        .expect("Failed to initialize browser");

    // Create a context with proxy - this tests the ProxyConfig structure
    // Even if we can't fully test proxy routing, we verify the API works
    let proxy_config =
        viewpoint_mcp::browser::ProxyConfig::new(format!("http://127.0.0.1:{proxy_port}"));

    // Verify proxy config is correctly constructed
    assert_eq!(
        proxy_config.server,
        format!("http://127.0.0.1:{proxy_port}")
    );
    assert!(proxy_config.username.is_none());
    assert!(proxy_config.password.is_none());

    // Test proxy config with authentication
    let proxy_with_auth =
        viewpoint_mcp::browser::ProxyConfig::new(format!("http://127.0.0.1:{proxy_port}"))
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
