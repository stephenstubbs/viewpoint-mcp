## MODIFIED Requirements

### Requirement: JavaScript Evaluation
The system SHALL allow executing JavaScript in the page context.

#### Scenario: Evaluate expression
- **WHEN** `browser_evaluate` is called with `function: "() => document.title"`
- **THEN** the expression is evaluated and result returned

#### Scenario: Evaluate on element
- **WHEN** `browser_evaluate` is called with `ref: "c0p0f0e1"` and `function: "(el) => el.textContent"`
- **THEN** the system calls `page.locator_from_ref("c0p0f0e1")`
- **AND** the function is evaluated with the resolved element
- **AND** the result is properly serialized and returned

#### Scenario: Evaluate on element returns object
- **WHEN** `browser_evaluate` is called with `ref: "c0p0f0e1"` and `function: "(el) => ({ tag: el.tagName, id: el.id })"`
- **THEN** the function is evaluated with the element
- **AND** the object result is properly serialized as JSON

#### Scenario: Evaluate on element returns string
- **WHEN** `browser_evaluate` is called with `ref: "c0p0f0e1"` and `function: "(el) => el.textContent"`
- **THEN** the function is evaluated with the element
- **AND** the string result is returned directly

#### Scenario: Evaluate on element returns null
- **WHEN** `browser_evaluate` is called with `ref: "c0p0f0e1"` and `function: "(el) => el.getAttribute('nonexistent')"`
- **THEN** the function is evaluated with the element
- **AND** null is returned and properly formatted

### Requirement: Page Inspection Tools
The system SHALL provide tools for inspecting page state.

#### Scenario: Capture accessibility snapshot
- **WHEN** `browser_snapshot` is called
- **THEN** the accessibility tree is captured and formatted
- **AND** element refs are assigned for interactive elements

#### Scenario: Take screenshot
- **WHEN** `browser_take_screenshot` is called
- **THEN** a screenshot of the viewport is captured
- **AND** returned as base64-encoded image data

#### Scenario: Screenshot element
- **WHEN** `browser_take_screenshot` is called with `ref: "c0p0f0e1"`
- **THEN** the system calls `page.locator_from_ref("c0p0f0e1")`
- **AND** only the resolved element is captured

#### Scenario: Full page screenshot
- **WHEN** `browser_take_screenshot` is called with `fullPage: true`
- **THEN** the entire scrollable page is captured

#### Scenario: Get console messages
- **WHEN** `browser_console_messages` is called
- **THEN** console messages captured since page load are returned
- **AND** messages are filtered by the specified level

#### Scenario: Console messages include all levels
- **WHEN** `browser_console_messages` is called with `level: "debug"`
- **THEN** messages from console.log, console.info, console.warn, console.error, and console.debug are returned

#### Scenario: Console messages filtered by level
- **WHEN** `browser_console_messages` is called with `level: "error"`
- **THEN** only console.error messages are returned

#### Scenario: Console message buffer limit
- **WHEN** more than 1000 console messages are logged
- **THEN** the oldest messages are evicted to maintain the 1000 message limit

#### Scenario: Get network requests
- **WHEN** `browser_network_requests` is called
- **THEN** network requests since page load are returned

### Requirement: Context Management Tools
The system SHALL provide tools for managing multiple isolated browser contexts.

#### Scenario: Create context
- **WHEN** `browser_context_create` is called with `name: "clean"`
- **THEN** a new isolated browser context is created
- **AND** the context becomes active
- **AND** the response confirms creation with context details

#### Scenario: Create context with proxy
- **WHEN** `browser_context_create` is called with:
  ```json
  {
    "name": "uk_proxy",
    "proxy": {
      "server": "socks5://proxy.example.com:1080",
      "username": "user",
      "password": "pass"
    }
  }
  ```
- **THEN** the context routes all traffic through the proxy
- **AND** proxy authentication is handled automatically

#### Scenario: Create context with storage state
- **WHEN** `browser_context_create` is called with `storageState: "/path/to/cookies.json"`
- **THEN** the context is initialized with saved cookies and localStorage
- **Note**: Implementation pending viewpoint-core storage state API support

#### Scenario: Switch context
- **WHEN** `browser_context_switch` is called with `name: "returning_user"`
- **THEN** that context becomes active
- **AND** subsequent tool calls operate on that context's pages

#### Scenario: List contexts
- **WHEN** `browser_context_list` is called
- **THEN** the response includes all contexts with:
  - `name`: context identifier
  - `isActive`: boolean indicating current context
  - `pageCount`: number of open pages
  - `currentUrl`: URL of active page (if any)
  - `proxy`: proxy server (if configured)

#### Scenario: Close context
- **WHEN** `browser_context_close` is called with `name: "temp"`
- **THEN** that context and all its pages are closed
- **AND** if it was active, "default" becomes active

#### Scenario: Close active context error
- **WHEN** `browser_context_close` is called with the name of the only remaining context
- **THEN** an error is returned: "Cannot close the only remaining context"

#### Scenario: Save context storage state
- **WHEN** `browser_context_save_storage` is called with `name: "logged_in"` and `path: "/path/to/save.json"`
- **THEN** cookies and localStorage are collected from the context
- **AND** the storage state is saved to the specified file path
- **AND** the response confirms successful save

#### Scenario: Save storage state handles page session errors
- **WHEN** `browser_context_save_storage` is called
- **AND** some pages have stale or invalid sessions
- **THEN** the system skips pages with invalid sessions
- **AND** collects storage from valid pages
- **AND** returns success with partial data (or appropriate error if no valid pages)
