# Tasks: Add Core Browser Tools

## 1. Tool Framework
- [x] 1.1 Define Tool trait with name, description, input schema, execute
- [x] 1.2 Implement tool registry for MCP tool listing
- [x] 1.3 Add tool input validation using JSON Schema

## 2. Navigation Tools
- [x] 2.1 Implement `browser_navigate` (goto URL with wait options)
- [x] 2.2 Implement `browser_navigate_back` (history back)

## 3. Interaction Tools (ref-based)
- [x] 3.1 Implement `browser_click` (click element by ref)
- [x] 3.2 Implement `browser_type` (type text into element)
- [x] 3.3 Implement `browser_fill_form` (fill multiple form fields)
- [x] 3.4 Implement `browser_hover` (hover over element)
- [x] 3.5 Implement `browser_drag` (drag from element to element)
- [x] 3.6 Implement `browser_select_option` (select dropdown option)
- [x] 3.7 Implement `browser_press_key` (keyboard key press)
- [x] 3.8 Implement `browser_file_upload` (upload files to input)

## 4. Inspection Tools
- [x] 4.1 Implement `browser_snapshot` (accessibility tree capture)
- [x] 4.2 Implement `browser_take_screenshot` (page or element screenshot)
- [x] 4.3 Implement `browser_console_messages` (get console logs)
- [x] 4.4 Implement `browser_network_requests` (list network requests)

## 5. State Tools
- [x] 5.1 Implement `browser_evaluate` (execute JavaScript)
- [x] 5.2 Implement `browser_wait_for` (wait for text/condition/time)
- [x] 5.3 Implement `browser_handle_dialog` (accept/dismiss alerts)

## 6. Management Tools
- [x] 6.1 Implement `browser_close` (close page/browser)
- [x] 6.2 Implement `browser_resize` (resize viewport)
- [x] 6.3 Implement `browser_tabs` (list/create/close/select tabs)
- [x] 6.4 Implement `browser_install` (install browser if missing)

## 7. Optional Capability Tools
- [x] 7.1 Implement `browser_mouse_click_xy` (vision: click coordinates)
- [x] 7.2 Implement `browser_mouse_move_xy` (vision: move to coordinates)
- [x] 7.3 Implement `browser_mouse_drag_xy` (vision: drag coordinates)
- [x] 7.4 Implement `browser_pdf_save` (pdf: save page as PDF)
- [x] 7.5 Implement capability-based tool filtering (hide vision/pdf tools when not enabled)

## 8. Context Management Tools
- [x] 8.1 Implement `browser_context_create` (create isolated context with optional proxy/storage)
- [x] 8.2 Implement `browser_context_switch` (switch active context)
- [x] 8.3 Implement `browser_context_list` (list all contexts with details)
- [x] 8.4 Implement `browser_context_close` (close named context)
- [x] 8.5 Implement `browser_context_save_storage` (export cookies/localStorage)

## 9. Testing
- [x] 9.1 Add unit tests for each tool's input validation
- [x] 9.2 Add integration tests for navigation tools
- [x] 9.3 Add integration tests for interaction tools
- [x] 9.4 Add integration tests for inspection tools
- [x] 9.5 Add integration tests for context management tools
- [x] 9.6 Add integration test for parallel context operations (ad comparison scenario)
