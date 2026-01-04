//! Console message capture and storage.
//!
//! This module provides per-page console message buffering for the MCP server.

use std::collections::VecDeque;
use std::sync::Arc;

use serde::Serialize;
use tokio::sync::RwLock;
use viewpoint_core::ConsoleMessage as VpConsoleMessage;
use viewpoint_core::ConsoleMessageType as VpConsoleMessageType;

/// Maximum number of console messages to store per page.
const CONSOLE_BUFFER_MAX: usize = 1000;

/// Console message level for filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConsoleLevel {
    /// Debug level (lowest priority).
    Debug = 0,
    /// Info/log level.
    Info = 1,
    /// Warning level.
    Warning = 2,
    /// Error level (highest priority).
    Error = 3,
}

impl ConsoleLevel {
    /// Check if a message type matches this level filter.
    ///
    /// Returns true if the message type should be included when filtering at this level.
    /// Each level includes itself and all more severe levels.
    pub fn includes(&self, msg_type: &StoredConsoleMessageType) -> bool {
        let msg_level = match msg_type {
            StoredConsoleMessageType::Debug => ConsoleLevel::Debug,
            StoredConsoleMessageType::Log | StoredConsoleMessageType::Info => ConsoleLevel::Info,
            StoredConsoleMessageType::Warning => ConsoleLevel::Warning,
            StoredConsoleMessageType::Error | StoredConsoleMessageType::Assert => {
                ConsoleLevel::Error
            }
            // Other types are treated as info level
            _ => ConsoleLevel::Info,
        };
        msg_level >= *self
    }
}

/// Stored console message type (serializable version).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StoredConsoleMessageType {
    Log,
    Debug,
    Info,
    Error,
    Warning,
    Dir,
    DirXml,
    Table,
    Trace,
    Clear,
    Count,
    Assert,
    Profile,
    ProfileEnd,
    StartGroup,
    EndGroup,
    TimeEnd,
}

impl From<VpConsoleMessageType> for StoredConsoleMessageType {
    fn from(t: VpConsoleMessageType) -> Self {
        match t {
            VpConsoleMessageType::Log => Self::Log,
            VpConsoleMessageType::Debug => Self::Debug,
            VpConsoleMessageType::Info => Self::Info,
            VpConsoleMessageType::Error => Self::Error,
            VpConsoleMessageType::Warning => Self::Warning,
            VpConsoleMessageType::Dir => Self::Dir,
            VpConsoleMessageType::DirXml => Self::DirXml,
            VpConsoleMessageType::Table => Self::Table,
            VpConsoleMessageType::Trace => Self::Trace,
            VpConsoleMessageType::Clear => Self::Clear,
            VpConsoleMessageType::Count => Self::Count,
            VpConsoleMessageType::Assert => Self::Assert,
            VpConsoleMessageType::Profile => Self::Profile,
            VpConsoleMessageType::ProfileEnd => Self::ProfileEnd,
            VpConsoleMessageType::StartGroup => Self::StartGroup,
            VpConsoleMessageType::EndGroup => Self::EndGroup,
            VpConsoleMessageType::TimeEnd => Self::TimeEnd,
        }
    }
}

impl std::fmt::Display for StoredConsoleMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Log => "log",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Dir => "dir",
            Self::DirXml => "dirxml",
            Self::Table => "table",
            Self::Trace => "trace",
            Self::Clear => "clear",
            Self::Count => "count",
            Self::Assert => "assert",
            Self::Profile => "profile",
            Self::ProfileEnd => "profileEnd",
            Self::StartGroup => "startGroup",
            Self::EndGroup => "endGroup",
            Self::TimeEnd => "timeEnd",
        };
        write!(f, "{s}")
    }
}

/// A stored console message (serializable, without CDP connection references).
#[derive(Debug, Clone, Serialize)]
pub struct StoredConsoleMessage {
    /// Message type.
    #[serde(rename = "type")]
    pub message_type: StoredConsoleMessageType,
    /// Message text.
    pub text: String,
    /// Timestamp (milliseconds since epoch).
    pub timestamp: f64,
    /// Source URL if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Line number if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
}

impl StoredConsoleMessage {
    /// Create a stored message from a viewpoint console message.
    pub fn from_viewpoint(msg: &VpConsoleMessage) -> Self {
        let location = msg.location();
        Self {
            message_type: msg.type_().into(),
            text: msg.text(),
            timestamp: msg.timestamp(),
            url: location.as_ref().map(|l| l.url.clone()),
            line_number: location.map(|l| l.line_number),
        }
    }
}

/// Per-page console message buffer.
#[derive(Debug, Default)]
pub struct ConsoleBuffer {
    messages: VecDeque<StoredConsoleMessage>,
}

impl ConsoleBuffer {
    /// Create a new empty buffer.
    pub fn new() -> Self {
        Self {
            messages: VecDeque::with_capacity(CONSOLE_BUFFER_MAX),
        }
    }

    /// Add a message to the buffer, evicting oldest if at capacity.
    pub fn push(&mut self, message: StoredConsoleMessage) {
        if self.messages.len() >= CONSOLE_BUFFER_MAX {
            self.messages.pop_front();
        }
        self.messages.push_back(message);
    }

    /// Get all messages matching the given level filter.
    pub fn get_messages(&self, level: ConsoleLevel) -> Vec<&StoredConsoleMessage> {
        self.messages
            .iter()
            .filter(|m| level.includes(&m.message_type))
            .collect()
    }

    /// Get all messages.
    pub fn all_messages(&self) -> &VecDeque<StoredConsoleMessage> {
        &self.messages
    }

    /// Clear all messages.
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Get the number of stored messages.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

/// Shared console buffer that can be passed to async handlers.
pub type SharedConsoleBuffer = Arc<RwLock<ConsoleBuffer>>;

/// Create a new shared console buffer.
pub fn new_shared_buffer() -> SharedConsoleBuffer {
    Arc::new(RwLock::new(ConsoleBuffer::new()))
}
