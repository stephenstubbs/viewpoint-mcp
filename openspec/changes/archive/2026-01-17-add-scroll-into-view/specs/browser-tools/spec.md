## ADDED Requirements

### Requirement: Scroll Into View Tool

The system SHALL provide a tool for scrolling elements into the visible viewport.

#### Scenario: Scroll element into view

- **WHEN** `browser_scroll_into_view` is called with `ref: "e1a2b3"` and `element: "Submit button"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** scrolls the element into the center of the viewport
- **AND** returns success after the scroll completes

#### Scenario: Scroll element not found

- **WHEN** `browser_scroll_into_view` is called with a ref that does not exist
- **THEN** the system returns an error indicating the element was not found

#### Scenario: Scroll with stale ref

- **WHEN** `browser_scroll_into_view` is called with a ref from a previous snapshot
- **AND** the element no longer exists in the DOM
- **THEN** the system returns an error indicating the element may no longer exist
