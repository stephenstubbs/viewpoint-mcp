# Change: Add Core Browser Tools

## Why
The MCP server needs browser automation tools matching playwright-mcp for feature parity. These tools enable LLMs to navigate, interact with, and inspect web pages.

## What Changes
- Add navigation tools: `browser_navigate`, `browser_navigate_back`
- Add interaction tools: `browser_click`, `browser_type`, `browser_fill_form`, `browser_hover`, `browser_drag`, `browser_select_option`, `browser_press_key`, `browser_file_upload`
- Add inspection tools: `browser_snapshot`, `browser_take_screenshot`, `browser_console_messages`, `browser_network_requests`
- Add state tools: `browser_evaluate`, `browser_wait_for`, `browser_handle_dialog`
- Add management tools: `browser_close`, `browser_resize`, `browser_tabs`, `browser_install`

## Impact
- Affected specs: `browser-tools` (new capability)
- Affected code: `viewpoint-mcp/src/tools/`
- Dependencies: `mcp-server`, `accessibility-snapshots`
