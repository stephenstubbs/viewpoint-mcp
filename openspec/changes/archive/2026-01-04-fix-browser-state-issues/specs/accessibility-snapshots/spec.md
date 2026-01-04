## MODIFIED Requirements

### Requirement: Accessibility Tree Capture
The system SHALL capture the accessibility tree from the current page for LLM consumption.

#### Scenario: Capture full page accessibility tree
- **WHEN** `browser_snapshot` tool is called
- **THEN** the system retrieves the accessibility tree from the page
- **AND** returns a formatted text representation

#### Scenario: Handle empty page
- **WHEN** `browser_snapshot` is called on a blank page
- **THEN** the system returns a minimal tree with just the document node

#### Scenario: Handle frames
- **WHEN** the page contains iframes
- **THEN** the accessibility tree includes frame contents
- **AND** frame boundaries are clearly marked

#### Scenario: Handle null accessibility tree response
- **WHEN** `browser_snapshot` is called
- **AND** the browser returns a null or empty accessibility tree (e.g., minimal HTML without semantic elements)
- **THEN** the system SHALL NOT fail with a deserialization error
- **AND** the system returns a minimal document node representation
- **AND** an informational message indicates the page has no accessible content
