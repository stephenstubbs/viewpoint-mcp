## MODIFIED Requirements

### Requirement: Element Reference System
The system SHALL assign unique references to interactive elements using viewpoint-core's native CDP backend node identifiers.

**Implementation Note**: The spec originally described a hybrid identification strategy (id/data-testid/name prioritization) for stable refs. The current implementation uses viewpoint-core's native `node_ref` field which provides refs in the format `e{backendNodeId}`. This is a simpler approach but refs may change on page refresh. Enhanced stable identification would require viewpoint-core changes.

#### Scenario: Reference format
- **WHEN** refs are generated
- **THEN** the format is `e{backendNodeId}` where backendNodeId is the CDP backend node identifier
- **AND** in multi-context mode, refs are prefixed with context name (e.g., `clean:e12345`)

#### Scenario: Native ref from viewpoint-core
- **WHEN** viewpoint-core's `AriaSnapshot.node_ref` is present
- **THEN** the ref is used directly for element identification
- **AND** the ref can be passed to `page.locator_from_ref()` for reliable element resolution

#### Scenario: Reference validation on use
- **WHEN** a tool is called with ref `e{backendNodeId}`
- **THEN** the system looks up the element by ref in the current snapshot
- **AND** validates the element still exists
- **AND** verifies the role matches expectations

### Requirement: Stale Reference Detection
The system SHALL detect stale references and provide actionable recovery guidance.

#### Scenario: Reference from current snapshot
- **WHEN** a tool is called with a ref from the most recent snapshot
- **THEN** the action proceeds normally

#### Scenario: Reference exists but element changed
- **WHEN** a tool is called with a ref from a previous snapshot
- **AND** an element with matching ref exists but role or name changed significantly
- **THEN** the system returns an error with details:
  - "Element changed since snapshot. Was: button 'Submit', Now: button 'Loading...'"
  - "Take a new snapshot to get current element state."

#### Scenario: Reference no longer exists
- **WHEN** a tool is called with a ref that no longer matches any element
- **THEN** the system returns an error with recovery suggestions:
  - "Element 'Submit button' (ref: e12345) no longer exists."
  - "Take a new snapshot to see current page state."

#### Scenario: Graceful handling of minor changes
- **WHEN** a tool is called with a ref from a previous snapshot
- **AND** the element exists with only minor changes (e.g., text content updated)
- **THEN** the action proceeds with a note:
  - "Note: Element may have changed. Using current state."

#### Scenario: Snapshot history for comparison
- **WHEN** a new snapshot is taken
- **THEN** the previous snapshot is retained for staleness comparison
- **AND** only one previous snapshot is kept (not full history)

### Requirement: Snapshot Text Format
The system SHALL format accessibility snapshots as indented text for LLM readability.

#### Scenario: Format tree structure
- **WHEN** the accessibility tree is formatted
- **THEN** the output uses indentation to show hierarchy
- **AND** each line contains role, name, and ref (if interactive)

#### Scenario: Example output format
- **WHEN** a page with heading, button, and input is captured
- **THEN** the output resembles:
  ```
  - document:
    - heading "Welcome"
    - button "Sign In" [ref=e12345]
    - textbox "Email" [ref=e12346]
  ```

#### Scenario: Truncate long text content
- **WHEN** an element has text content exceeding 100 characters
- **THEN** the text is truncated with ellipsis ("...")
- **AND** the full text is available via element inspection
