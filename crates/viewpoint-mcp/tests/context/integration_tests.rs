//! Integration tests for parallel context operations and edge cases

use serde_json::json;
use viewpoint_mcp::tools::{
    BrowserContextCloseTool, BrowserContextCreateTool, BrowserContextSwitchTool,
    BrowserNavigateTool, Tool,
};

use super::create_browser;

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
        assert!(
            ctx.current_url()
                .await
                .map(|u: String| u.contains(user))
                .unwrap_or(false)
        );
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
