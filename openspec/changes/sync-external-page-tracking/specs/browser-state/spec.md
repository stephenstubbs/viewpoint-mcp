## ADDED Requirements

### Requirement: External Page Tracking

The system SHALL track all pages by querying viewpoint-core's `context.pages()` API instead of maintaining duplicate state.

#### Scenario: Page opened via window.open() is tracked

- **GIVEN** an MCP server with an active browser context
- **WHEN** JavaScript executes `window.open()` on a page
- **THEN** `browser_tabs list` shows the new tab
- **AND** `browser_tabs select` can switch to the new tab

#### Scenario: Page opened via target="_blank" link is tracked

- **GIVEN** an MCP server with an active browser context
- **WHEN** a `target="_blank"` link is clicked via `browser_click`
- **THEN** `browser_tabs list` shows the new tab

#### Scenario: Page opened via Ctrl+Click is tracked

- **GIVEN** an MCP server with an active browser context
- **WHEN** `browser_click` is called with `modifiers: ["Control"]` on a link
- **THEN** `browser_tabs list` shows the new tab

#### Scenario: Console capture for external pages

- **GIVEN** an MCP server with an active browser context
- **WHEN** a page is opened externally
- **AND** that page logs to the console
- **THEN** `browser_console_messages` returns those messages when the page is active
