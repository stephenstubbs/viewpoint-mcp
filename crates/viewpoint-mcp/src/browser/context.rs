//! Browser context state management

use std::time::Instant;

use viewpoint_core::error::{ContextError, PageError};
use viewpoint_core::{BrowserContext, Page};

use super::config::ProxyConfig;
use crate::snapshot::AccessibilitySnapshot;

/// State for a browser context
///
/// Each context is isolated with its own cookies, storage, and cache.
pub struct ContextState {
    /// Context name (unique identifier)
    pub name: String,

    /// Active page index within this context
    pub active_page: usize,

    /// Current URL of the active page
    pub current_url: Option<String>,

    /// Proxy configuration for this context
    pub proxy: Option<ProxyConfig>,

    /// The actual Viewpoint browser context
    context: BrowserContext,

    /// Pages in this context
    pages: Vec<Page>,

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
}

/// Default cache TTL in seconds
const SNAPSHOT_CACHE_TTL_SECS: u64 = 5;

impl std::fmt::Debug for ContextState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextState")
            .field("name", &self.name)
            .field("active_page", &self.active_page)
            .field("page_count", &self.pages.len())
            .field("current_url", &self.current_url)
            .field("proxy", &self.proxy)
            .field("has_cached_snapshot", &self.cached_snapshot.is_some())
            .finish_non_exhaustive()
    }
}

impl ContextState {
    /// Create a new context state from a Viewpoint context
    pub async fn new(
        name: impl Into<String>,
        context: BrowserContext,
    ) -> Result<Self, ContextError> {
        let name = name.into();

        // Create initial page
        let page = context.new_page().await?;

        Ok(Self {
            name,
            active_page: 0,
            current_url: None,
            proxy: None,
            context,
            pages: vec![page],
            cached_snapshot: None,
        })
    }

    /// Create a new context state with proxy
    #[must_use]
    pub fn with_proxy(mut self, proxy: ProxyConfig) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Get the number of pages
    #[must_use]
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Get the active page
    #[must_use]
    pub fn active_page(&self) -> Option<&Page> {
        self.pages.get(self.active_page)
    }

    /// Get the active page mutably
    #[must_use]
    pub fn active_page_mut(&mut self) -> Option<&mut Page> {
        self.pages.get_mut(self.active_page)
    }

    /// Get the underlying browser context
    #[must_use]
    pub fn context(&self) -> &BrowserContext {
        &self.context
    }

    /// Create a new page in this context
    pub async fn new_page(&mut self) -> Result<&Page, ContextError> {
        let page = self.context.new_page().await?;
        self.pages.push(page);
        self.active_page = self.pages.len() - 1;
        Ok(self.pages.last().unwrap())
    }

    /// Close a page by index
    pub async fn close_page(&mut self, index: usize) -> Result<(), PageError> {
        if index >= self.pages.len() {
            return Ok(());
        }

        let mut page = self.pages.remove(index);
        page.close().await?;

        // Adjust active page index
        if self.active_page >= self.pages.len() && !self.pages.is_empty() {
            self.active_page = self.pages.len() - 1;
        }

        Ok(())
    }

    /// Switch to a page by index
    pub fn switch_page(&mut self, index: usize) -> bool {
        if index < self.pages.len() {
            self.active_page = index;
            true
        } else {
            false
        }
    }

    /// Close this context and all its pages
    pub async fn close(mut self) -> Result<(), ContextError> {
        for mut page in self.pages {
            let _ = page.close().await;
        }
        self.context.close().await
    }

    /// Get the cached snapshot if still valid
    ///
    /// Returns `None` if:
    /// - No snapshot is cached
    /// - The cache has expired (>5 seconds old)
    /// - The URL has changed
    /// - The active page has changed
    #[must_use]
    pub fn get_cached_snapshot(&self) -> Option<&AccessibilitySnapshot> {
        let cache = self.cached_snapshot.as_ref()?;

        // Check if cache is expired
        if cache.captured_at.elapsed().as_secs() > SNAPSHOT_CACHE_TTL_SECS {
            return None;
        }

        // Check if page or URL changed
        if cache.page_index != self.active_page {
            return None;
        }

        if let Some(current_url) = &self.current_url
            && cache.url != *current_url {
                return None;
            }

        Some(&cache.snapshot)
    }

    /// Cache a snapshot for the active page
    pub fn cache_snapshot(&mut self, snapshot: AccessibilitySnapshot) {
        self.cached_snapshot = Some(CachedSnapshot {
            snapshot,
            captured_at: Instant::now(),
            url: self.current_url.clone().unwrap_or_default(),
            page_index: self.active_page,
        });
    }

    /// Invalidate the cached snapshot
    ///
    /// Call this after navigation or any action that modifies the page
    pub fn invalidate_cache(&mut self) {
        self.cached_snapshot = None;
    }
}
