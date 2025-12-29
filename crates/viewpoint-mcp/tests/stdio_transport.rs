//! Integration tests for stdio transport with mock MCP client
#![cfg(feature = "integration")]

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

/// Helper to send a JSON-RPC request and get response
fn send_request(stdin: &mut impl Write, stdout: &mut impl BufRead, request: &Value) -> Value {
    let request_str = serde_json::to_string(request).unwrap();
    writeln!(stdin, "{request_str}").unwrap();
    stdin.flush().unwrap();

    let mut response_line = String::new();
    stdout.read_line(&mut response_line).unwrap();
    serde_json::from_str(&response_line).unwrap()
}

#[test]
fn test_stdio_initialize_handshake() {
    // Build the CLI binary first
    let status = Command::new("cargo")
        .args(["build", "-p", "viewpoint-mcp-cli"])
        .status()
        .expect("Failed to build");
    assert!(status.success());

    // Start the server process
    let mut child = Command::new("cargo")
        .args(["run", "-p", "viewpoint-mcp-cli", "--", "--headless"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start server");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    // Send initialize request
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        }
    });

    let response = send_request(&mut stdin, &mut reader, &init_request);

    // Verify response structure
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"]["protocolVersion"].is_string());
    assert!(response["result"]["capabilities"].is_object());
    assert_eq!(response["result"]["serverInfo"]["name"], "viewpoint-mcp");

    // Send initialized notification (no response expected for notifications)
    let initialized = json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    let init_str = serde_json::to_string(&initialized).unwrap();
    writeln!(stdin, "{init_str}").unwrap();
    stdin.flush().unwrap();

    // Close stdin to signal shutdown
    drop(stdin);

    // Wait for clean exit
    let status = child.wait().expect("Failed to wait");
    assert!(status.success());
}

#[test]
fn test_stdio_tools_list() {
    let status = Command::new("cargo")
        .args(["build", "-p", "viewpoint-mcp-cli"])
        .status()
        .expect("Failed to build");
    assert!(status.success());

    let mut child = Command::new("cargo")
        .args(["run", "-p", "viewpoint-mcp-cli", "--", "--headless"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start server");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    // Initialize first
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    });
    let _ = send_request(&mut stdin, &mut reader, &init_request);

    // Request tools list
    let tools_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });

    let response = send_request(&mut stdin, &mut reader, &tools_request);

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 2);
    assert!(response["result"]["tools"].is_array());

    drop(stdin);
    child.wait().expect("Failed to wait");
}

#[test]
fn test_stdio_unknown_method() {
    let status = Command::new("cargo")
        .args(["build", "-p", "viewpoint-mcp-cli"])
        .status()
        .expect("Failed to build");
    assert!(status.success());

    let mut child = Command::new("cargo")
        .args(["run", "-p", "viewpoint-mcp-cli", "--", "--headless"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start server");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    // Send unknown method
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "unknown/method",
        "params": {}
    });

    let response = send_request(&mut stdin, &mut reader, &request);

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32601); // Method not found

    drop(stdin);
    child.wait().expect("Failed to wait");
}

#[test]
fn test_stdio_malformed_json() {
    let status = Command::new("cargo")
        .args(["build", "-p", "viewpoint-mcp-cli"])
        .status()
        .expect("Failed to build");
    assert!(status.success());

    let mut child = Command::new("cargo")
        .args(["run", "-p", "viewpoint-mcp-cli", "--", "--headless"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start server");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    // Send malformed JSON
    writeln!(stdin, "{{ invalid json }}").unwrap();
    stdin.flush().unwrap();

    let mut response_line = String::new();
    reader.read_line(&mut response_line).unwrap();
    let response: Value = serde_json::from_str(&response_line).unwrap();

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32700); // Parse error

    drop(stdin);
    child.wait().expect("Failed to wait");
}
