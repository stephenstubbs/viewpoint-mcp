//! Element reference system for stable element identification
//!
//! This module provides the [`ElementRef`] type which wraps viewpoint-core's native
//! element references.
//!
//! # Reference Format (viewpoint-core 0.3.1+)
//!
//! Refs now use the format `c{ctx}p{page}f{frame}e{counter}` where:
//! - `c{ctx}` - Context index (0-based)
//! - `p{page}` - Page index within the context
//! - `f{frame}` - Frame index (0 = main frame)
//! - `e{counter}` - Element counter within the snapshot
//!
//! Example: `c0p0f0e1`, `c0p0f1e3`, `c1p0f0e1`
//!
//! # Examples
//!
//! ```
//! use viewpoint_mcp::snapshot::ElementRef;
//!
//! // Parse a reference from a snapshot
//! let element_ref = ElementRef::parse("c0p0f0e1").unwrap();
//! assert_eq!(element_ref.ref_string(), "c0p0f0e1");
//!
//! // Create a reference with context name for display
//! let element_ref = ElementRef::with_context("c0p0f0e1", "my-context");
//! assert_eq!(element_ref.context(), Some("my-context"));
//! ```

/// An element reference for targeting elements in tool calls.
///
/// References use viewpoint-core's native format: `c{ctx}p{page}f{frame}e{counter}`
/// which includes context, page, and frame scoping to prevent cross-context misuse.
///
/// # Examples
///
/// ```
/// use viewpoint_mcp::snapshot::ElementRef;
///
/// // Create from a ref string
/// let element_ref = ElementRef::new("c0p0f0e5");
///
/// // Get the ref string for use with page.locator_from_ref()
/// assert_eq!(element_ref.ref_string(), "c0p0f0e5");
///
/// // Display format
/// assert_eq!(element_ref.to_string(), "c0p0f0e5");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementRef {
    /// The viewpoint-core ref string (e.g., "c0p0f0e1")
    ref_string: String,

    /// Optional MCP-level context name for display purposes
    /// Note: The viewpoint ref already includes context index (c{n})
    context_name: Option<String>,
}

impl ElementRef {
    /// Create a new element reference from a viewpoint-core ref string.
    ///
    /// The ref string should be in the format `c{ctx}p{page}f{frame}e{counter}`
    /// as provided by viewpoint-core's `AriaSnapshot.node_ref` field.
    #[must_use]
    pub fn new(ref_string: impl Into<String>) -> Self {
        Self {
            ref_string: ref_string.into(),
            context_name: None,
        }
    }

    /// Create a new element reference with an MCP context name for display
    #[must_use]
    pub fn with_context(ref_string: impl Into<String>, context_name: impl Into<String>) -> Self {
        Self {
            ref_string: ref_string.into(),
            context_name: Some(context_name.into()),
        }
    }

    /// Get the raw ref string (e.g., "c0p0f0e1")
    ///
    /// This is the format expected by `page.locator_from_ref()`.
    #[must_use]
    pub fn ref_string(&self) -> &str {
        &self.ref_string
    }

    /// Format the reference as a string for display
    ///
    /// Returns the ref string (context is embedded in the ref itself now)
    #[must_use]
    pub fn to_ref_string(&self) -> String {
        self.ref_string.clone()
    }

    /// Parse a reference string into an `ElementRef`
    ///
    /// Accepts format: `c{ctx}p{page}f{frame}e{counter}` (e.g., "c0p0f0e1")
    ///
    /// # Errors
    ///
    /// Returns an error if the format is invalid
    pub fn parse(s: &str) -> Result<Self, String> {
        // Format: c{ctx}p{page}f{frame}e{counter}
        if s.starts_with('c') && s.contains('p') && s.contains('f') && s.contains('e') {
            // Basic validation - ensure it has the right structure
            let valid = s[1..].chars().next().is_some_and(|c| c.is_ascii_digit());
            if valid {
                return Ok(Self {
                    ref_string: s.to_string(),
                    context_name: None,
                });
            }
        }

        Err(format!(
            "Invalid reference format: '{s}'. Expected format: c{{ctx}}p{{page}}f{{frame}}e{{counter}} (e.g., c0p0f0e1)"
        ))
    }

    /// Get the MCP context name if set (for display purposes)
    #[must_use]
    pub fn context(&self) -> Option<&str> {
        self.context_name.as_deref()
    }
}

impl std::fmt::Display for ElementRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ref_string())
    }
}
