# browser-state Spec Delta

## ADDED Requirements

### Requirement: Event-Driven Active Page Tracking

The system SHALL track which page is currently active by subscribing to viewpoint-core's `on_page_activated` events, keeping state synchronized when users interact with tabs directly in the browser UI.

#### Scenario: User clicks tab updates active page index

- **GIVEN** an MCP server with a browser context containing two pages
- **AND** `active_page_index` is 0 (first page)
- **WHEN** the user clicks on the second tab in the browser UI
- **THEN** `active_page_index` is updated to 1
- **AND** the update happens automatically via event subscription

#### Scenario: User clicks tab updates current URL

- **GIVEN** an MCP server with a browser context containing two pages
- **AND** page 0 is at "https://example.com"
- **AND** page 1 is at "https://google.com"
- **WHEN** the user clicks on the second tab in the browser UI
- **THEN** `current_url` is updated to "https://google.com"

#### Scenario: browser_close reports correct URL after user tab switch

- **GIVEN** an MCP server with pages at "https://example.com" and "https://google.com"
- **AND** the user has clicked on the Google tab in the browser UI
- **WHEN** `browser_close` is called
- **THEN** the response message includes "https://google.com"
- **AND** NOT "https://example.com"

#### Scenario: browser_tabs list shows correct active marker after user tab switch

- **GIVEN** an MCP server with two pages open
- **AND** the user has clicked on the second tab in the browser UI
- **WHEN** `browser_tabs list` is called
- **THEN** the second tab shows `[active]` marker
- **AND** the first tab does NOT show `[active]` marker

#### Scenario: Snapshot cache invalidated on user tab switch

- **GIVEN** an MCP server with a cached snapshot for page 0
- **WHEN** the user clicks on a different tab in the browser UI
- **THEN** the snapshot cache is invalidated
- **AND** the next snapshot request captures fresh data from the new active page

### Requirement: Page Activation Event Subscription

The system SHALL subscribe to viewpoint-core's `on_page_activated` event when creating a context state.

#### Scenario: Activation handler registered on context creation

- **GIVEN** a new browser context is being created
- **WHEN** `ContextState::new()` completes
- **THEN** an `on_page_activated` handler is registered with viewpoint-core
- **AND** the handler ID is stored to keep the subscription alive

#### Scenario: Handler finds page index from activated page

- **GIVEN** a context with pages tracked by viewpoint-core
- **WHEN** the `on_page_activated` handler receives a `Page`
- **THEN** it queries `context.pages().await` to get the current page list
- **AND** finds the index of the activated page by matching `target_id`
- **AND** updates `active_page_index` to that index
