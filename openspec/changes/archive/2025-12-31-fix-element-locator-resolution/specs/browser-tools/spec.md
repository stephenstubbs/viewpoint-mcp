# browser-tools Spec Delta

## MODIFIED Requirements

### Requirement: Element Interaction Tools
The system SHALL provide tools for interacting with page elements using accessibility refs.

#### Scenario: Click element
- **WHEN** `browser_click` is called with `ref: "e1a2b3"` and `element: "Sign In button"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** clicks the resolved DOM element

#### Scenario: Type text
- **WHEN** `browser_type` is called with `ref: "e1a2b3"`, `element: "Email input"`, and `text: "user@example.com"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** types the text into the resolved DOM element

#### Scenario: Hover element
- **WHEN** `browser_hover` is called with `ref: "e1a2b3"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** hovers the mouse over the resolved DOM element

#### Scenario: Drag and drop
- **WHEN** `browser_drag` is called with `startRef` and `endRef`
- **THEN** the system calls `page.locator_from_ref()` for both refs
- **AND** a drag operation is performed from start to end element

#### Scenario: Select dropdown option
- **WHEN** `browser_select_option` is called with `ref: "e1a2b3"` and `values: ["option1"]`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** selects the specified option in the resolved dropdown

#### Scenario: Fill form fields
- **WHEN** `browser_fill_form` is called with an array of field objects containing refs
- **THEN** the system calls `page.locator_from_ref()` for each field's ref
- **AND** each field is filled with its specified value

### Requirement: JavaScript Evaluation
The system SHALL allow executing JavaScript in the page context.

#### Scenario: Evaluate on element
- **WHEN** `browser_evaluate` is called with `ref: "e1a2b3"` and `function: "(el) => el.textContent"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** the function is evaluated with the resolved element

### Requirement: Page Inspection Tools
The system SHALL provide tools for inspecting page state.

#### Scenario: Screenshot element
- **WHEN** `browser_take_screenshot` is called with `ref: "e1a2b3"`
- **THEN** the system calls `page.locator_from_ref("e1a2b3")`
- **AND** only the resolved element is captured
