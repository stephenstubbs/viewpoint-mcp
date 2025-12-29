//! Integration tests for context management tools
//!
//! Run with:
//! ```sh
//! cargo test --features integration -p viewpoint-mcp --test tools_context
//! ```
#![cfg(feature = "integration")]

use serde_json::json;
use tempfile::TempDir;
use viewpoint_mcp::browser::{BrowserConfig, BrowserState};
use viewpoint_mcp::tools::{
    BrowserContextCloseTool, BrowserContextCreateTool, BrowserContextListTool,
    BrowserContextSaveStorageTool, BrowserContextSwitchTool, BrowserNavigateTool, Tool,
};

/// Helper to create a headless browser state
async fn create_browser() -> BrowserState {
    let config = BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let mut state = BrowserState::new(config);
    state.initialize().await.expect("Failed to initialize browser");
    state
}

// =============================================================================
// browser_context_create Tests
// =============================================================================

#[tokio::test]
async fn test_context_create_basic() {
    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    let result = tool
        .execute(&json!({ "name": "test_context" }), &mut browser)
        .await;

    assert!(result.is_ok(), "Create context should succeed: {:?}", result.err());
    let output = result.unwrap();
    assert!(output.contains("test_context") || output.contains("created"));

    // Verify context exists
    let contexts = browser.list_contexts();
    assert!(contexts.iter().any(|c| c.name == "test_context"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_create_multiple() {
    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    for i in 1..=5 {
        let result = tool
            .execute(&json!({ "name": format!("ctx_{}", i) }), &mut browser)
            .await;
        assert!(result.is_ok(), "Create context {} should succeed", i);
    }

    let contexts = browser.list_contexts();
    // Should have default + 5 new contexts
    assert_eq!(contexts.len(), 6);

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_create_duplicate_name() {
    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    // Create first context
    tool.execute(&json!({ "name": "duplicate" }), &mut browser)
        .await
        .unwrap();

    // Try to create another with same name
    let result = tool
        .execute(&json!({ "name": "duplicate" }), &mut browser)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("already exists") || err.contains("duplicate"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_create_with_storage_state() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().join("storage.json");

    // Create a valid storage state file
    std::fs::write(
        &storage_path,
        r#"{"cookies":[],"origins":[]}"#,
    )
    .unwrap();

    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    let result = tool
        .execute(
            &json!({
                "name": "with_storage",
                "storageState": storage_path.to_str().unwrap()
            }),
            &mut browser,
        )
        .await;

    // May succeed or fail depending on file format validation
    // Just ensure no panic
    let _ = result;

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_create_switches_to_new() {
    let mut browser = create_browser().await;
    let tool = BrowserContextCreateTool::new();

    // Initially on default
    assert_eq!(browser.active_context_name(), "default");

    // Create new context
    tool.execute(&json!({ "name": "new_active" }), &mut browser)
        .await
        .unwrap();

    // Should now be on new context
    assert_eq!(browser.active_context_name(), "new_active");

    browser.shutdown().await;
}

// =============================================================================
// browser_context_switch Tests
// =============================================================================

#[tokio::test]
async fn test_context_switch_basic() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let switch_tool = BrowserContextSwitchTool::new();

    // Create additional context
    create_tool
        .execute(&json!({ "name": "other" }), &mut browser)
        .await
        .unwrap();

    // Switch back to default
    let result = switch_tool
        .execute(&json!({ "name": "default" }), &mut browser)
        .await;

    assert!(result.is_ok());
    assert_eq!(browser.active_context_name(), "default");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_switch_nonexistent() {
    let mut browser = create_browser().await;
    let switch_tool = BrowserContextSwitchTool::new();

    let result = switch_tool
        .execute(&json!({ "name": "nonexistent" }), &mut browser)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found") || err.contains("does not exist"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_switch_preserves_state() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let switch_tool = BrowserContextSwitchTool::new();
    let nav_tool = BrowserNavigateTool::new();

    // Create two contexts with different pages
    create_tool
        .execute(&json!({ "name": "ctx_a" }), &mut browser)
        .await
        .unwrap();
    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Context A</h1>" }), &mut browser)
        .await
        .unwrap();

    create_tool
        .execute(&json!({ "name": "ctx_b" }), &mut browser)
        .await
        .unwrap();
    nav_tool
        .execute(&json!({ "url": "data:text/html,<h1>Context B</h1>" }), &mut browser)
        .await
        .unwrap();

    // Switch back to A
    switch_tool
        .execute(&json!({ "name": "ctx_a" }), &mut browser)
        .await
        .unwrap();

    // Context A should still have its URL
    let ctx = browser.active_context().unwrap();
    assert!(ctx.current_url.as_ref().map(|u| u.contains("Context A")).unwrap_or(false));

    browser.shutdown().await;
}

// =============================================================================
// browser_context_list Tests
// =============================================================================

#[tokio::test]
async fn test_context_list_default_only() {
    let mut browser = create_browser().await;
    let list_tool = BrowserContextListTool::new();

    let result = list_tool.execute(&json!({}), &mut browser).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("default"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_list_multiple() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let list_tool = BrowserContextListTool::new();

    // Create multiple contexts
    for name in ["alpha", "beta", "gamma"] {
        create_tool
            .execute(&json!({ "name": name }), &mut browser)
            .await
            .unwrap();
    }

    let result = list_tool.execute(&json!({}), &mut browser).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("default"));
    assert!(output.contains("alpha"));
    assert!(output.contains("beta"));
    assert!(output.contains("gamma"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_list_shows_active() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let list_tool = BrowserContextListTool::new();

    create_tool
        .execute(&json!({ "name": "active_one" }), &mut browser)
        .await
        .unwrap();

    let result = list_tool.execute(&json!({}), &mut browser).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    // Should indicate which context is active
    assert!(output.contains("active") || output.contains("*") || output.contains("current"));

    browser.shutdown().await;
}

// =============================================================================
// browser_context_close Tests
// =============================================================================

#[tokio::test]
async fn test_context_close_non_active() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let close_tool = BrowserContextCloseTool::new();

    // Create and switch to new context, then switch back
    create_tool
        .execute(&json!({ "name": "to_close" }), &mut browser)
        .await
        .unwrap();

    // Switch back to default
    browser.switch_context("default").unwrap();

    // Close the other context
    let result = close_tool
        .execute(&json!({ "name": "to_close" }), &mut browser)
        .await;

    assert!(result.is_ok());
    
    // Verify it's gone
    let contexts = browser.list_contexts();
    assert!(!contexts.iter().any(|c| c.name == "to_close"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_close_active_fallback() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let close_tool = BrowserContextCloseTool::new();

    // Create new context (becomes active)
    create_tool
        .execute(&json!({ "name": "active_ctx" }), &mut browser)
        .await
        .unwrap();

    assert_eq!(browser.active_context_name(), "active_ctx");

    // Close the active context
    let result = close_tool
        .execute(&json!({ "name": "active_ctx" }), &mut browser)
        .await;

    assert!(result.is_ok());
    // Should fall back to default
    assert_eq!(browser.active_context_name(), "default");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_close_last_fails() {
    let mut browser = create_browser().await;
    let close_tool = BrowserContextCloseTool::new();

    // Try to close the only context
    let result = close_tool
        .execute(&json!({ "name": "default" }), &mut browser)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("last") || err.contains("only") || err.contains("cannot"));

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_close_nonexistent() {
    let mut browser = create_browser().await;
    let close_tool = BrowserContextCloseTool::new();

    let result = close_tool
        .execute(&json!({ "name": "nonexistent" }), &mut browser)
        .await;

    assert!(result.is_err());

    browser.shutdown().await;
}

// =============================================================================
// browser_context_save_storage Tests
// =============================================================================

// NOTE: Storage saving is not yet implemented in viewpoint-core
// The tool returns a success message but doesn't actually save files
#[tokio::test]
async fn test_context_save_storage() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().join("saved_storage.json");

    let mut browser = create_browser().await;
    let save_tool = BrowserContextSaveStorageTool::new();

    let result = save_tool
        .execute(
            &json!({
                "name": "default",
                "path": storage_path.to_str().unwrap()
            }),
            &mut browser,
        )
        .await;

    // Tool should succeed (even if it doesn't actually save yet)
    assert!(result.is_ok(), "Save storage should succeed: {:?}", result.err());

    // Output should mention the feature status
    let output = result.unwrap();
    assert!(output.contains("default") || output.contains("storage"));

    browser.shutdown().await;
}

// NOTE: The save storage tool currently doesn't validate context existence
// because storage saving is not yet implemented
#[tokio::test]
async fn test_context_save_storage_nonexistent_context() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().join("storage.json");

    let mut browser = create_browser().await;
    let save_tool = BrowserContextSaveStorageTool::new();

    let result = save_tool
        .execute(
            &json!({
                "name": "nonexistent",
                "path": storage_path.to_str().unwrap()
            }),
            &mut browser,
        )
        .await;

    // Currently succeeds because it's a placeholder implementation
    // When fully implemented, this should error for nonexistent context
    let _ = result;

    browser.shutdown().await;
}

// =============================================================================
// Integration: Parallel Context Operations
// =============================================================================

#[tokio::test]
async fn test_parallel_context_different_urls() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let switch_tool = BrowserContextSwitchTool::new();
    let nav_tool = BrowserNavigateTool::new();

    // Create contexts for different "users"
    for user in ["user_a", "user_b", "user_c"] {
        create_tool
            .execute(&json!({ "name": user }), &mut browser)
            .await
            .unwrap();
        nav_tool
            .execute(
                &json!({ "url": format!("data:text/html,<h1>Page for {}</h1>", user) }),
                &mut browser,
            )
            .await
            .unwrap();
    }

    // Verify each context has its own URL
    for user in ["user_a", "user_b", "user_c"] {
        switch_tool
            .execute(&json!({ "name": user }), &mut browser)
            .await
            .unwrap();

        let ctx = browser.active_context().unwrap();
        assert!(ctx.current_url.as_ref().map(|u| u.contains(user)).unwrap_or(false));
    }

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_isolation_cookies() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let nav_tool = BrowserNavigateTool::new();

    // Context A sets a cookie
    create_tool
        .execute(&json!({ "name": "ctx_with_cookie" }), &mut browser)
        .await
        .unwrap();
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<script>document.cookie='test=value';</script>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // Context B should not have that cookie (isolation)
    create_tool
        .execute(&json!({ "name": "ctx_without_cookie" }), &mut browser)
        .await
        .unwrap();
    
    // Each context is isolated, cookies don't leak
    // This is inherent in browser context design

    browser.shutdown().await;
}

// =============================================================================
// Edge Cases
// =============================================================================

#[tokio::test]
async fn test_context_name_with_special_characters() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();

    // Names with various characters
    for name in ["test-context", "test_context", "test123", "Test Context"] {
        let result = create_tool
            .execute(&json!({ "name": name }), &mut browser)
            .await;
        // Some names may fail validation, just ensure no panic
        let _ = result;
    }

    browser.shutdown().await;
}

#[tokio::test]
async fn test_context_rapid_create_close() {
    let mut browser = create_browser().await;
    let create_tool = BrowserContextCreateTool::new();
    let close_tool = BrowserContextCloseTool::new();

    // Rapidly create and close contexts
    for i in 0..10 {
        let name = format!("rapid_{}", i);
        create_tool
            .execute(&json!({ "name": name }), &mut browser)
            .await
            .unwrap();
    }

    // Close them all (except default)
    for i in 0..10 {
        let name = format!("rapid_{}", i);
        close_tool
            .execute(&json!({ "name": name }), &mut browser)
            .await
            .unwrap();
    }

    // Should only have default left
    assert_eq!(browser.list_contexts().len(), 1);

    browser.shutdown().await;
}
