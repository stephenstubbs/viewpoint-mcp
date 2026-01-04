//! Snapshot caching integration tests

use serde_json::json;
use viewpoint_mcp::tools::{BrowserNavigateTool, BrowserSnapshotTool, Tool};

use super::create_browser;

#[tokio::test]
async fn test_snapshot_caching_same_page() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Navigate to a page
    nav_tool
        .execute(
            &json!({ "url": "data:text/html,<h1>Cache Test</h1><button>Click</button>" }),
            &mut browser,
        )
        .await
        .unwrap();

    // First snapshot (should cache)
    let start1 = std::time::Instant::now();
    let snap1 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let time1 = start1.elapsed();

    // Second snapshot (should use cache, be faster)
    let start2 = std::time::Instant::now();
    let snap2 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();
    let time2 = start2.elapsed();

    // Both snapshots should be identical (same cached data)
    assert_eq!(snap1, snap2, "Cached snapshot should match original");

    // Cache hit should generally be faster, but we just verify it works
    tracing::debug!(?time1, ?time2, "Snapshot timing");

    browser.shutdown().await;
}

#[tokio::test]
async fn test_snapshot_cache_invalidation_on_allrefs() {
    let mut browser = create_browser().await;
    let nav_tool = BrowserNavigateTool::new();
    let snapshot_tool = BrowserSnapshotTool::new();

    // Navigate to a page with many elements
    let mut buttons = String::new();
    for i in 0..50 {
        buttons.push_str(&format!("<button>Button {i}</button>"));
    }
    let html = format!("<html><body>{buttons}</body></html>");

    nav_tool
        .execute(
            &json!({ "url": format!("data:text/html,{}", html) }),
            &mut browser,
        )
        .await
        .unwrap();

    // First snapshot without allRefs
    let snap1 = snapshot_tool
        .execute(&json!({}), &mut browser)
        .await
        .unwrap();

    // Second snapshot with allRefs (should NOT use cache)
    let snap2 = snapshot_tool
        .execute(&json!({ "allRefs": true }), &mut browser)
        .await
        .unwrap();

    // Third snapshot with allRefs (should use cache from second)
    let snap3 = snapshot_tool
        .execute(&json!({ "allRefs": true }), &mut browser)
        .await
        .unwrap();

    // snap2 and snap3 should be identical (cached all_refs snapshot)
    assert_eq!(snap2, snap3, "Cached allRefs snapshot should match");

    // snap1 may differ from snap2/snap3 depending on compact mode
    assert!(snap1.contains("button"));
    assert!(snap2.contains("button"));

    browser.shutdown().await;
}
