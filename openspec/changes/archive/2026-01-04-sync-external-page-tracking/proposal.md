# Proposal: Sync External Page Tracking

## Problem

viewpoint-mcp maintains its own internal `pages: Vec<Page>` that only includes pages created via `new_page()`. Pages opened externally (Ctrl+Click, `window.open()`, `target="_blank"`) are not tracked.

## Dependency

This change depends on viewpoint-core's `fix-pages-return-type` change, which makes `context.pages()` return `Vec<Page>` instead of `Vec<PageInfo>`.

## Solution

Remove duplicate page tracking from viewpoint-mcp and use `context.pages()` directly.

1. Remove `pages: Vec<Page>` from `ContextState`
2. Query `context.pages()` when page list/count is needed
3. Keep console buffer tracking keyed by target_id
4. Subscribe to `on_page` only for console buffer setup

## Scope

- Remove duplicate page tracking
- Update page access methods to use `context.pages()`
- Maintain console buffer tracking for external pages

## Acceptance Criteria

1. `browser_tabs list` shows externally-opened tabs
2. `browser_tabs select` can switch to externally-opened tabs
3. Console messages are captured for externally-opened pages
4. No duplicate page state maintained

## Risk Assessment

- **Low risk**: Simplifies code by removing duplicate state
- **Dependency**: Requires viewpoint-core `fix-pages-return-type` to be completed first
