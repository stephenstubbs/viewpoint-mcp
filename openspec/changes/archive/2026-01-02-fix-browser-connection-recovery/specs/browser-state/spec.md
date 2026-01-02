## ADDED Requirements

### Requirement: Connection Loss Recovery

The system SHALL detect browser connection loss and automatically reset state to allow re-initialization.

The recovery mechanism SHALL:
- Detect connection loss from error messages containing "WebSocket connection lost", "ConnectionLost", or similar
- Clear all browser state (contexts, browser reference, initialized flag) without attempting to close dead connections
- Allow the next tool call to trigger fresh browser initialization
- Log connection loss events at WARN level for debugging

#### Scenario: Browser process dies during operation

- **GIVEN** an MCP server with an active browser connection
- **WHEN** the Chromium process is killed externally
- **AND** a tool call is made (e.g., `browser_navigate`)
- **THEN** a `CdpError::ConnectionLost` error is detected
- **AND** the browser state is reset (initialized = false, contexts cleared)
- **AND** a warning is logged about the connection loss

#### Scenario: Automatic re-initialization after connection loss

- **GIVEN** an MCP server whose browser connection was lost
- **AND** the browser state has been reset
- **WHEN** the next tool call is made
- **THEN** `BrowserState::initialize()` launches a fresh browser
- **AND** the tool operation succeeds

#### Scenario: Non-connection errors do not trigger reset

- **GIVEN** an MCP server with an active browser connection
- **WHEN** a tool call fails with a non-connection error (e.g., element not found)
- **THEN** the browser state is NOT reset
- **AND** subsequent tool calls continue using the existing connection
