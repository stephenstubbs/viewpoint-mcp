## MODIFIED Requirements

### Requirement: MCP Protocol Support
The server SHALL implement the Model Context Protocol (MCP) for tool-based LLM interaction.

#### Scenario: Initialize connection
- **WHEN** an MCP client connects
- **THEN** the server responds to the `initialize` request with server capabilities
- **AND** the server reports available tools

#### Scenario: List tools
- **WHEN** the client sends `tools/list`
- **THEN** the server returns all registered tool definitions with names, descriptions, and input schemas
- **AND** core tools (27) are always registered
- **AND** optional tools are registered based on enabled capabilities

#### Scenario: Call tool
- **WHEN** the client sends `tools/call` with a valid tool name and arguments
- **THEN** the server executes the tool and returns the result

#### Scenario: Unknown tool
- **WHEN** the client sends `tools/call` with an unknown tool name
- **THEN** the server returns an MCP error response with code `-32601` (method not found)

#### Scenario: Tool registration at startup
- **WHEN** the MCP server is created
- **THEN** all core browser tools are registered with the tool registry
- **AND** optional tools (vision, pdf) are registered only if their capability is enabled
