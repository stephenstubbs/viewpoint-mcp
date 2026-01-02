# mcp-server Specification

## Purpose
This specification defines the Model Context Protocol (MCP) server implementation that exposes browser automation capabilities to AI agents. The server handles protocol communication over stdio and SSE transports, manages tool registration and execution, and coordinates browser lifecycle across client sessions.
## Requirements
### Requirement: MCP Protocol Support
The server SHALL implement the Model Context Protocol (MCP) for tool-based LLM interaction.

#### Scenario: Initialize connection
- **WHEN** an MCP client connects
- **THEN** the server responds to the `initialize` request with server capabilities
- **AND** the server reports available tools

#### Scenario: List tools
- **WHEN** the client sends `tools/list`
- **THEN** the server returns all registered tool definitions with names, descriptions, and input schemas
- **AND** core tools (27) are always registered
- **AND** optional tools are registered based on enabled capabilities

#### Scenario: Call tool
- **WHEN** the client sends `tools/call` with a valid tool name and arguments
- **THEN** the server executes the tool and returns the result

#### Scenario: Unknown tool
- **WHEN** the client sends `tools/call` with an unknown tool name
- **THEN** the server returns an MCP error response with code `-32601` (method not found)

#### Scenario: Tool registration at startup
- **WHEN** the MCP server is created
- **THEN** all core browser tools are registered with the tool registry
- **AND** optional tools (vision, pdf) are registered only if their capability is enabled

### Requirement: Stdio Transport
The server SHALL support JSON-RPC communication over stdio (stdin/stdout).

#### Scenario: Receive request via stdin
- **WHEN** a JSON-RPC request is written to stdin
- **THEN** the server parses and processes the request
- **AND** writes the response to stdout

#### Scenario: Handle malformed JSON
- **WHEN** invalid JSON is written to stdin
- **THEN** the server returns a JSON-RPC parse error response

### Requirement: SSE Transport
The server SHALL support Server-Sent Events (SSE) transport when `--port` is specified.

#### Scenario: Enable SSE mode
- **WHEN** the server is started with `--port 8931`
- **THEN** the server listens on HTTP port 8931
- **AND** accepts MCP connections at `/mcp`

#### Scenario: Handle SSE client
- **WHEN** a client connects via SSE
- **THEN** tool results are streamed as SSE events

### Requirement: SSE Authentication
The server SHALL require API key authentication for SSE transport to prevent unauthorized access.

#### Scenario: Configure API key
- **WHEN** the server is started with `--port 8931 --api-key mysecretkey`
- **THEN** the server requires authentication for all SSE connections

#### Scenario: Generate API key
- **WHEN** the server is started with `--port 8931` without `--api-key`
- **THEN** the server generates a random API key
- **AND** prints the key to stderr on startup for operator use

#### Scenario: Authenticate with valid key
- **WHEN** a client connects to `/mcp` with header `Authorization: Bearer <valid-key>`
- **THEN** the connection is accepted
- **AND** MCP protocol proceeds normally

#### Scenario: Reject missing authentication
- **WHEN** a client connects to `/mcp` without an `Authorization` header
- **THEN** the server responds with HTTP 401 Unauthorized
- **AND** the response body indicates authentication is required

#### Scenario: Reject invalid key
- **WHEN** a client connects with header `Authorization: Bearer <invalid-key>`
- **THEN** the server responds with HTTP 403 Forbidden
- **AND** the connection is not established

#### Scenario: Stdio transport unaffected
- **WHEN** the server is started without `--port` (stdio mode)
- **THEN** no authentication is required
- **AND** `--api-key` flag is ignored if provided

### Requirement: Browser Lifecycle Management
The server SHALL manage browser state across tool invocations.

#### Scenario: Lazy browser initialization
- **WHEN** the first browser tool is called
- **AND** no browser is running
- **THEN** the server launches a browser instance

#### Scenario: Connect to existing browser
- **WHEN** the server is started with `--cdp-endpoint http://localhost:9222`
- **THEN** the server connects to the existing browser via CDP
- **AND** does not launch a new browser process

#### Scenario: Browser state persistence
- **WHEN** multiple tool calls are made
- **THEN** the browser, context, and page state persists between calls

#### Scenario: Graceful shutdown
- **WHEN** the server receives a shutdown signal
- **THEN** the server closes the browser gracefully
- **AND** terminates the browser process if owned

### Requirement: CLI Configuration
The server SHALL accept command-line arguments matching playwright-mcp.

#### Scenario: Headless mode
- **WHEN** the server is started with `--headless`
- **THEN** the browser runs without a visible window

#### Scenario: Headed mode (default)
- **WHEN** the server is started without `--headless`
- **THEN** the browser runs with a visible window

#### Scenario: Viewport configuration
- **WHEN** the server is started with `--viewport-size 1280x720`
- **THEN** the browser viewport is set to 1280x720 pixels

#### Scenario: User data directory
- **WHEN** the server is started with `--user-data-dir /path/to/profile`
- **THEN** the browser uses the specified profile directory
- **AND** persists cookies and storage between sessions

#### Scenario: Capability flags
- **WHEN** the server is started with `--caps vision,pdf`
- **THEN** vision and PDF tools are enabled
- **AND** coordinate-based tools are available

### Requirement: Multiple Browser Contexts
The server SHALL support multiple isolated browser contexts for parallel scraping and profile comparison.

#### Scenario: Create named context
- **WHEN** `browser_context_create` is called with `name: "clean"`
- **THEN** a new isolated browser context is created
- **AND** the context has separate cookies, storage, and cache
- **AND** the new context becomes the active context

#### Scenario: Create context with proxy
- **WHEN** `browser_context_create` is called with `name: "uk"` and `proxy: { server: "socks5://uk-proxy:1080" }`
- **THEN** the context routes all traffic through the specified proxy
- **AND** different contexts can use different proxies simultaneously

#### Scenario: Create context with storage state
- **WHEN** `browser_context_create` is called with `name: "logged_in"` and `storageState: "/path/to/state.json"`
- **THEN** the context is initialized with cookies and localStorage from the file

#### Scenario: Switch active context
- **WHEN** `browser_context_switch` is called with `name: "returning"`
- **THEN** that context becomes active for subsequent tool calls
- **AND** all tools operate on the active context's pages

#### Scenario: List contexts
- **WHEN** `browser_context_list` is called
- **THEN** all context names are returned
- **AND** each entry includes: name, page count, current URL, proxy (if set)
- **AND** the active context is marked

#### Scenario: Close context
- **WHEN** `browser_context_close` is called with `name: "temp"`
- **THEN** that context and all its pages are closed
- **AND** if it was active, the "default" context becomes active

#### Scenario: Context isolation
- **WHEN** cookies are set in context "A"
- **THEN** context "B" does NOT see those cookies
- **AND** localStorage is isolated per context
- **AND** cache is isolated per context

#### Scenario: Default context
- **WHEN** no context is explicitly created
- **THEN** a "default" context is used automatically
- **AND** backward compatibility is maintained with single-context workflows

#### Scenario: Parallel operations
- **WHEN** multiple contexts exist
- **THEN** each context can have pages navigated independently
- **AND** snapshots from different contexts use context-prefixed refs (e.g., `clean:e1`, `uk:e2`)

