# Design: Sync External Page Tracking

## Current Architecture

```
viewpoint-mcp (ContextState):
  pages: Vec<Page>              // Duplicate tracking - manually maintained
  console_buffers: Vec<...>     // Per-page console capture
  context: BrowserContext       // The viewpoint-core context
```

## Proposed Architecture

```
viewpoint-mcp (ContextState):
  context: BrowserContext                      // Use context.pages() directly
  console_buffers: HashMap<String, ...>        // Keyed by target_id
  active_page_index: usize                     // Which page is active
```

## Changes

### Remove Duplicate Page Tracking

Replace internal `pages: Vec<Page>` with calls to `context.pages().await`.

### Console Buffer Tracking

Change from `Vec` (indexed by position) to `HashMap<String, SharedConsoleBuffer>` keyed by `target_id`. This handles:
- Pages appearing at any position
- Pages being closed and indices shifting
- External pages being added

### on_page Subscription

Subscribe to `on_page` event only to set up console capture for new pages:

```rust
context.on_page(move |page| {
    let buffers = buffers.clone();
    async move {
        let buffer = new_shared_buffer();
        setup_console_handler(&page, buffer.clone()).await;
        buffers.write().await.insert(page.target_id().to_string(), buffer);
    }
}).await;
```

### Page Access Methods

```rust
impl ContextState {
    pub async fn page_count(&self) -> usize {
        self.context.pages().await.map(|p| p.len()).unwrap_or(0)
    }
    
    pub async fn active_page(&self) -> Option<Page> {
        self.context.pages().await.ok()?.get(self.active_page_index).cloned()
    }
}
```

## Dependency

Requires viewpoint-core to return `Vec<Page>` from `context.pages()` (see `fix-pages-return-type` change in viewpoint repo).
