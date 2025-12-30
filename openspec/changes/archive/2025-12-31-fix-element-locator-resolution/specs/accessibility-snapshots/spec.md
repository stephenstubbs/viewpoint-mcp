# accessibility-snapshots Spec Delta

## MODIFIED Requirements

### Requirement: Element Lookup by Reference
The system SHALL resolve element references to DOM elements for tool actions using viewpoint's native ref resolution API.

#### Scenario: Resolve valid reference to locator
- **WHEN** a tool is called with `ref=e1a2b3`
- **AND** the reference exists in the current page
- **THEN** the system uses `page.locator_from_ref(ref)` to get a `Locator`
- **AND** the `Locator` provides auto-waiting capabilities for reliable interactions

#### Scenario: Handle invalid reference format
- **WHEN** a tool is called with a malformed `ref` value
- **THEN** the system returns an error with the expected format

#### Scenario: Handle stale reference
- **WHEN** a tool is called with a `ref` for an element no longer in the DOM
- **THEN** the system returns an error indicating the element no longer exists
- **AND** suggests taking a new snapshot
