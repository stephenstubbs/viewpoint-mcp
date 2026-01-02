# Tasks

## 1. Add Connection Recovery to BrowserState

- [x] 1.1 Add `reset_on_connection_loss()` method to clear state without closing connections
- [x] 1.2 Add `handle_potential_connection_loss(error_msg)` method to detect and reset on connection errors
- [x] 1.3 Add tracing/logging for connection loss detection and recovery

## 2. Integrate Recovery into Tool Execution

- [x] 2.1 Update tool trait or common error handling to check for connection loss
- [x] 2.2 Ensure recovery is triggered before returning error to MCP client

## 3. Testing

- [x] 3.1 Add unit test for `reset_on_connection_loss()` 
- [x] 3.2 Add unit test for `handle_potential_connection_loss()` with various error messages
- [x] 3.3 Manual verification: kill browser process and confirm next tool call succeeds

## 4. Validation

- [x] 4.1 Run `cargo test --workspace`
- [x] 4.2 Run `cargo clippy --workspace`
