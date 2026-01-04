//! Browser context state management

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use tokio::sync::RwLock;
use viewpoint_core::error::{ContextError, PageError};
use viewpoint_core::{BrowserContext, HandlerId, Page};

use super::config::ProxyConfig;
use super::console::{SharedConsoleBuffer, StoredConsoleMessage, new_shared_buffer};
use crate::snapshot::AccessibilitySnapshot;

/// State for a browser context
///
/// Each context is isolated with its own cookies, storage, and cache.
/// Pages are tracked by viewpoint-core; we only track console buffers keyed by target_id.
pub struct ContextState {
    /// Context name (unique identifier)
    pub name: String,

    /// Active page index within this context
    pub active_page_index: usize,

    /// Current URL of the active page
    pub current_url: Option<String>,

    /// Proxy configuration for this context
    pub proxy: Option<ProxyConfig>,

    /// The actual Viewpoint browser context
    context: BrowserContext,

    /// Console message buffers per page, keyed by target_id.
    /// This allows tracking console for externally-opened pages.
    console_buffers: Arc<RwLock<HashMap<String, SharedConsoleBuffer>>>,

    /// Handler ID for the on_page event subscription (kept alive)
    _page_handler_id: HandlerId,

    /// Cached snapshot for the active page
    cached_snapshot: Option<CachedSnapshot>,
}

/// A cached accessibility snapshot with metadata
pub struct CachedSnapshot {
    /// The cached snapshot
    pub snapshot: AccessibilitySnapshot,

    /// When the snapshot was captured
    pub captured_at: Instant,

    /// URL when snapshot was captured
    pub url: String,

    /// Page index when captured
    pub page_index: usize,

    /// Whether `all_refs` mode was used when capturing
    pub all_refs: bool,
}

/// Default cache TTL in seconds
const SNAPSHOT_CACHE_TTL_SECS: u64 = 5;

impl std::fmt::Debug for ContextState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextState")
            .field("name", &self.name)
            .field("active_page_index", &self.active_page_index)
            .field("current_url", &self.current_url)
            .field("proxy", &self.proxy)
            .field("has_cached_snapshot", &self.cached_snapshot.is_some())
            .finish_non_exhaustive()
    }
}

impl ContextState {
    /// Create a new context state from a Viewpoint context.
    ///
    /// This sets up a subscription to `on_page` events so that console buffers
    /// are automatically created for any new pages (including externally-opened ones).
    pub async fn new(
        name: impl Into<String>,
        context: BrowserContext,
    ) -> Result<Self, ContextError> {
        let name = name.into();

        // Create shared console buffer storage
        let console_buffers: Arc<RwLock<HashMap<String, SharedConsoleBuffer>>> =
            Arc::new(RwLock::new(HashMap::new()));

        // Subscribe to on_page events for console buffer setup on all new pages
        let buffers_for_handler = console_buffers.clone();
        let page_handler_id = context
            .on_page(move |page: Page| {
                let buffers = buffers_for_handler.clone();
                async move {
                    let target_id = page.target_id().to_string();
                    let buffer = new_shared_buffer();

                    // Set up console handler for this page
                    let buffer_clone = buffer.clone();
                    page.on_console(move |msg| {
                        let buffer = buffer_clone.clone();
                        async move {
                            let stored = StoredConsoleMessage::from_viewpoint(&msg);
                            buffer.write().await.push(stored);
                        }
                    })
                    .await;

                    // Store the buffer keyed by target_id
                    buffers.write().await.insert(target_id, buffer);
                }
            })
            .await;

        // Create initial page (this will trigger our on_page handler)
        let _page = context.new_page().await?;

        Ok(Self {
            name,
            active_page_index: 0,
            current_url: None,
            proxy: None,
            context,
            console_buffers,
            _page_handler_id: page_handler_id,
            cached_snapshot: None,
        })
    }

    /// Create a new context state with proxy
    #[must_use]
    pub fn with_proxy(mut self, proxy: ProxyConfig) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Get the number of pages.
    ///
    /// # Errors
    ///
    /// Returns an error if the context is closed.
    pub async fn page_count(&self) -> Result<usize, ContextError> {
        self.context.page_count().await
    }

    /// Get the active page.
    ///
    /// Returns the page at the current `active_page_index`, or `None` if the
    /// index is out of bounds.
    ///
    /// # Errors
    ///
    /// Returns an error if the context is closed.
    pub async fn active_page(&self) -> Result<Option<Page>, ContextError> {
        let pages = self.context.pages().await?;
        Ok(pages.into_iter().nth(self.active_page_index))
    }

    /// Get the underlying browser context
    #[must_use]
    pub fn context(&self) -> &BrowserContext {
        &self.context
    }

    /// Create a new page in this context.
    ///
    /// The page is automatically tracked by viewpoint-core, and console buffer
    /// setup is handled by our `on_page` subscription.
    ///
    /// # Errors
    ///
    /// Returns an error if page creation fails.
    pub async fn new_page(&mut self) -> Result<Page, ContextError> {
        let page = self.context.new_page().await?;
        // Update active page to the new page
        let page_count = self.context.page_count().await?;
        self.active_page_index = page_count.saturating_sub(1);
        Ok(page)
    }

    /// Close a page by index.
    ///
    /// # Errors
    ///
    /// Returns an error if closing the page fails.
    pub async fn close_page(&mut self, index: usize) -> Result<(), PageError> {
        let pages = self
            .context
            .pages()
            .await
            .map_err(|e| PageError::EvaluationFailed(format!("Failed to get pages: {e}")))?;

        if index >= pages.len() {
            return Ok(());
        }

        // Get the page to close
        let mut page = pages
            .into_iter()
            .nth(index)
            .ok_or_else(|| PageError::EvaluationFailed("Page not found".to_string()))?;

        // Remove the console buffer for this page
        let target_id = page.target_id().to_string();
        self.console_buffers.write().await.remove(&target_id);

        // Close the page
        page.close().await?;

        // Adjust active page index
        let new_count = self.context.page_count().await.unwrap_or(0);
        if self.active_page_index >= new_count && new_count > 0 {
            self.active_page_index = new_count - 1;
        }

        Ok(())
    }

    /// Switch to a page by index.
    ///
    /// Returns `true` if the switch was successful, `false` if the index is out of bounds.
    pub async fn switch_page(&mut self, index: usize) -> bool {
        let page_count = self.context.page_count().await.unwrap_or(0);
        if index < page_count {
            self.active_page_index = index;
            true
        } else {
            false
        }
    }

    /// Close this context and all its pages.
    ///
    /// # Errors
    ///
    /// Returns an error if closing fails.
    pub async fn close(mut self) -> Result<(), ContextError> {
        // viewpoint-core handles closing all pages when context is closed
        self.context.close().await
    }

    /// Get the cached snapshot if still valid
    ///
    /// Returns `None` if:
    /// - No snapshot is cached
    /// - The cache has expired (>5 seconds old)
    /// - The URL has changed
    /// - The active page has changed
    /// - The `all_refs` mode doesn't match (requesting `all_refs` when cached without, or vice versa)
    #[must_use]
    pub fn get_cached_snapshot(&self, all_refs: bool) -> Option<&AccessibilitySnapshot> {
        let cache = self.cached_snapshot.as_ref()?;

        // Check if cache is expired
        if cache.captured_at.elapsed().as_secs() > SNAPSHOT_CACHE_TTL_SECS {
            return None;
        }

        // Check if page or URL changed
        if cache.page_index != self.active_page_index {
            return None;
        }

        if let Some(current_url) = &self.current_url
            && cache.url != *current_url
        {
            return None;
        }

        // Check if all_refs mode matches
        // A cached all_refs snapshot can satisfy a non-all_refs request (superset)
        // But a non-all_refs snapshot cannot satisfy an all_refs request
        if all_refs && !cache.all_refs {
            return None;
        }

        Some(&cache.snapshot)
    }

    /// Cache a snapshot for the active page
    pub fn cache_snapshot(&mut self, snapshot: AccessibilitySnapshot, all_refs: bool) {
        self.cached_snapshot = Some(CachedSnapshot {
            snapshot,
            captured_at: Instant::now(),
            url: self.current_url.clone().unwrap_or_default(),
            page_index: self.active_page_index,
            all_refs,
        });
    }

    /// Invalidate the cached snapshot
    ///
    /// Call this after navigation or any action that modifies the page
    pub fn invalidate_cache(&mut self) {
        self.cached_snapshot = None;
    }

    /// Get the console buffer for the active page.
    ///
    /// Returns `None` if there's no active page or no buffer for it.
    pub async fn active_console_buffer(&self) -> Option<SharedConsoleBuffer> {
        let page = self.active_page().await.ok()??;
        let target_id = page.target_id();
        let buffers = self.console_buffers.read().await;
        buffers.get(target_id).cloned()
    }

    /// Get the current URL of the active page by querying the page directly.
    ///
    /// This method fetches the URL from the browser rather than relying on
    /// cached state, ensuring it's always accurate even after client-side
    /// navigation or other URL changes.
    ///
    /// Returns `None` if there's no active page or the URL query fails.
    pub async fn get_current_url(&self) -> Option<String> {
        let page = self.active_page().await.ok()??;
        page.url().await.ok()
    }

    /// Get all pages in this context.
    ///
    /// # Errors
    ///
    /// Returns an error if the context is closed.
    pub async fn pages(&self) -> Result<Vec<Page>, ContextError> {
        self.context.pages().await
    }

    /// Get the active page index.
    #[must_use]
    pub fn active_page_index(&self) -> usize {
        self.active_page_index
    }
}
