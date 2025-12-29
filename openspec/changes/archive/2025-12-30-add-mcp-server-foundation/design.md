# Design: MCP Server Foundation

## Context
The MCP (Model Context Protocol) server must expose browser automation as tools that LLMs can invoke. Microsoft's playwright-mcp uses Node.js/Playwright; we use Rust/Viewpoint for a native implementation.

## Goals
- Drop-in replacement for playwright-mcp (same tool names, same CLI flags)
- Support stdio (local) and SSE (remote) transports
- Manage browser lifecycle across tool invocations

## Non-Goals
- Firefox/WebKit support (Viewpoint is Chromium-only)
- Browser extension mode

## Decisions

### Crate Structure
```
crates/
├── viewpoint-mcp/          # Library: MCP protocol, tools, browser management
│   └── src/
│       ├── server/         # MCP protocol handling
│       ├── tools/          # Tool implementations
│       ├── browser/        # Browser state management
│       └── transport/      # stdio, SSE
└── viewpoint-mcp-cli/      # Binary: CLI entrypoint
    └── src/
        └── main.rs
```

**Rationale**: Separating library from CLI follows hexagonal architecture and enables embedding the MCP server in other applications.

### Transport Layer
- **stdio**: Read JSON-RPC from stdin, write to stdout. Default for local MCP clients. No authentication required (trusted local process).
- **SSE**: HTTP server with Server-Sent Events for responses. Use `--port` flag to enable. Requires API key authentication.

**Rationale**: stdio matches playwright-mcp. SSE adds API key auth (improvement over playwright-mcp) because remote deployments need access control.

### Browser State Management
The server maintains a single browser instance with multiple named contexts:

```rust
struct BrowserState {
    browser: Option<Browser>,
    contexts: HashMap<String, ContextState>,
    active_context: String,
}

struct ContextState {
    context: BrowserContext,
    pages: Vec<Page>,
    active_page: usize,
    proxy: Option<ProxyConfig>,
}
```

Tools operate on the active context's active page. The `browser_context_*` tools manage contexts, and `browser_tabs` switches pages within a context.

**Rationale**: Multiple contexts enable parallel scraping with different profiles (cookies, proxies, storage) for use cases like ad comparison and geo-targeted content analysis. Each context is isolated—cookies and storage don't leak between contexts.

### CLI Compatibility
Support these playwright-mcp flags:
- `--headless` / no flag for headed
- `--browser <chrome|chromium>` 
- `--cdp-endpoint <url>` - Connect to existing browser
- `--port <n>` - Enable SSE transport
- `--viewport-size <WxH>`
- `--user-data-dir <path>`
- `--caps <vision,pdf>` - Enable optional capabilities

Additional flags (viewpoint-mcp extensions):
- `--api-key <key>` - API key for SSE authentication (auto-generated if omitted with `--port`)

### Error Handling
- Tools return MCP error responses with structured error codes
- Browser errors map to appropriate MCP error types
- Connection failures return clear error messages

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| MCP protocol changes | Pin to stable MCP spec version |
| Browser crashes | Implement graceful recovery, allow `browser_close` to reset state |
| Memory leaks from long sessions | Document recommended usage patterns |

## Resolved Questions

### MCP Resources for Page Content
**Decision**: Deferred to v2.

MCP resources could expose page content as read-only data sources (e.g., `resource://browser/page/html`). However:
- playwright-mcp achieves the same functionality via tools
- Tool-based approach (`browser_evaluate`, `browser_snapshot`) is proven in production
- Resources would add complexity without clear benefit for current use cases

Will evaluate based on user feedback after v1.

### Multiple Browser Contexts
**Decision**: Implemented in v1 with proxy support.

Multiple isolated contexts enable:
- Parallel scraping with different user profiles
- Ad comparison (logged-in vs anonymous, different geos)
- Geo-targeted content analysis via per-context proxies
- Avoiding cookie/storage cross-contamination

See Browser State Management section for implementation details.

## Deferred to v2

### MCP Resources
Potential future addition for static content subscriptions and caching. Not needed for v1 scraping use cases.
