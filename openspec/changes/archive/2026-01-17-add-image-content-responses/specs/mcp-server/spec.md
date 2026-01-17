## MODIFIED Requirements

### Requirement: CLI Configuration
The server SHALL accept command-line arguments matching playwright-mcp.

#### Scenario: Headless mode
- **WHEN** the server is started with `--headless`
- **THEN** the browser runs without a visible window

#### Scenario: Headed mode (default)
- **WHEN** the server is started without `--headless`
- **THEN** the browser runs with a visible window

#### Scenario: Viewport configuration
- **WHEN** the server is started with `--viewport-size 1280x720`
- **THEN** the browser viewport is set to 1280x720 pixels

#### Scenario: User data directory
- **WHEN** the server is started with `--user-data-dir /path/to/profile`
- **THEN** the browser uses the specified profile directory
- **AND** persists cookies and storage between sessions

#### Scenario: Capability flags
- **WHEN** the server is started with `--caps vision,pdf`
- **THEN** vision and PDF tools are enabled
- **AND** coordinate-based tools are available

#### Scenario: Screenshot directory default
- **WHEN** the server is started without `--screenshot-dir`
- **THEN** screenshots are saved to `.viewpoint-mcp-screenshots/` in the current working directory
- **AND** the directory is created if it does not exist

#### Scenario: Screenshot directory custom
- **WHEN** the server is started with `--screenshot-dir /path/to/screenshots`
- **THEN** screenshots are saved to the specified directory
- **AND** the directory is created if it does not exist

#### Scenario: Image responses file mode (default)
- **WHEN** the server is started without `--image-responses`
- **OR** the server is started with `--image-responses=file`
- **THEN** screenshots are saved to the screenshot directory
- **AND** the text response includes the relative file path
- **AND** no `ImageContent` is included in the response

#### Scenario: Image responses inline mode
- **WHEN** the server is started with `--image-responses=inline`
- **THEN** screenshots are saved to the screenshot directory
- **AND** the text response includes the relative file path
- **AND** an `ImageContent` with scaled base64 data is included in the response

#### Scenario: Image responses omit mode
- **WHEN** the server is started with `--image-responses=omit`
- **THEN** screenshots are saved to the screenshot directory
- **AND** the text response only confirms the screenshot was taken
- **AND** no file path or `ImageContent` is included in the response
