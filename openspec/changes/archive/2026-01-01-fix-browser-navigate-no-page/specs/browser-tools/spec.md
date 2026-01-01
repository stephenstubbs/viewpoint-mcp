## MODIFIED Requirements

### Requirement: Navigation Tools
The system SHALL provide tools for page navigation.

#### Scenario: Navigate to URL
- **WHEN** `browser_navigate` is called with `url: "https://example.com"`
- **THEN** the page navigates to the URL
- **AND** waits for the page to load

#### Scenario: Navigate back
- **WHEN** `browser_navigate_back` is called
- **THEN** the page navigates to the previous history entry

#### Scenario: Navigate after all pages closed
- **WHEN** all pages have been closed via `browser_close`
- **AND** `browser_navigate` is called with a URL
- **THEN** a new page is automatically created in the active context
- **AND** the page navigates to the specified URL
- **AND** the tool returns success (not an error)
