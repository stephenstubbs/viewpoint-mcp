//! Accessibility snapshot system for LLM-based browser automation
//!
//! This module provides accessibility tree capture and element reference management,
//! enabling LLMs to interact with web pages using structured data rather than vision.

mod capture;
mod classification;
mod element;
mod error;
mod format;
mod reference;
mod stale;

#[cfg(test)]
mod tests;

pub use capture::{AccessibilitySnapshot, SnapshotOptions};
pub use classification::{ElementTier, classify_role};
pub use element::SnapshotElement;
pub use error::{SnapshotError, SnapshotResult};
pub use format::SnapshotFormatter;
pub use reference::ElementRef;
pub use stale::{StaleRefDetector, StaleRefError};
