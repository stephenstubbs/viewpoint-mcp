//! Element reference system for stable element identification
//!
//! This module provides the `ElementRef` type which wraps viewpoint-core's native
//! element references in the format `e{backendNodeId}`.

/// An element reference for targeting elements in tool calls.
///
/// References use viewpoint-core's native format: `e{backendNodeId}` where
/// `backendNodeId` is the CDP backend node identifier.
///
/// In multi-context mode, refs may include a context prefix: `clean:e12345`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementRef {
    /// The viewpoint-core ref string (e.g., "e12345")
    ref_string: String,

    /// Optional context prefix for multi-context mode
    context: Option<String>,
}

impl ElementRef {
    /// Create a new element reference from a viewpoint-core ref string.
    ///
    /// The ref string should be in the format `e{backendNodeId}` as provided
    /// by viewpoint-core's `AriaSnapshot.node_ref` field.
    #[must_use]
    pub fn new(ref_string: impl Into<String>) -> Self {
        Self {
            ref_string: ref_string.into(),
            context: None,
        }
    }

    /// Create a new element reference with context prefix
    #[must_use]
    pub fn with_context(ref_string: impl Into<String>, context: impl Into<String>) -> Self {
        Self {
            ref_string: ref_string.into(),
            context: Some(context.into()),
        }
    }

    /// Get the raw ref string (e.g., "e12345")
    ///
    /// This is the format expected by `page.locator_from_ref()`.
    #[must_use]
    pub fn ref_string(&self) -> &str {
        &self.ref_string
    }

    /// Format the reference as a string for display
    ///
    /// Returns the ref with optional context prefix (e.g., "clean:e12345" or "e12345")
    #[must_use]
    pub fn to_ref_string(&self) -> String {
        match &self.context {
            Some(ctx) => format!("{}:{}", ctx, self.ref_string),
            None => self.ref_string.clone(),
        }
    }

    /// Parse a reference string into an `ElementRef`
    ///
    /// Accepts formats:
    /// - `e{backendNodeId}` (e.g., "e12345")
    /// - `{context}:e{backendNodeId}` (e.g., "clean:e12345")
    ///
    /// # Errors
    ///
    /// Returns an error if the format is invalid
    pub fn parse(s: &str) -> Result<Self, String> {
        // Check for context prefix (e.g., "clean:e12345")
        if let Some((context, rest)) = s.split_once(':') {
            if rest.starts_with('e') && rest.len() > 1 {
                // Validate that the part after 'e' is a valid number
                let id_part = &rest[1..];
                if id_part.chars().all(|c| c.is_ascii_digit()) {
                    return Ok(Self {
                        ref_string: rest.to_string(),
                        context: Some(context.to_string()),
                    });
                }
            }
            return Err(format!(
                "Invalid reference format: '{s}'. Expected format: e{{backendNodeId}} or {{context}}:e{{backendNodeId}}"
            ));
        }

        // No context prefix (e.g., "e12345")
        if s.starts_with('e') && s.len() > 1 {
            let id_part = &s[1..];
            if id_part.chars().all(|c| c.is_ascii_digit()) {
                return Ok(Self {
                    ref_string: s.to_string(),
                    context: None,
                });
            }
        }

        Err(format!(
            "Invalid reference format: '{s}'. Expected format: e{{backendNodeId}} or {{context}}:e{{backendNodeId}}"
        ))
    }

    /// Get the context name if present
    #[must_use]
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
}

impl std::fmt::Display for ElementRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ref_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_ref() {
        let element_ref = ElementRef::parse("e12345").unwrap();
        assert_eq!(element_ref.ref_string(), "e12345");
        assert_eq!(element_ref.context(), None);
        assert_eq!(element_ref.to_ref_string(), "e12345");
    }

    #[test]
    fn test_parse_ref_with_context() {
        let element_ref = ElementRef::parse("clean:e12345").unwrap();
        assert_eq!(element_ref.ref_string(), "e12345");
        assert_eq!(element_ref.context(), Some("clean"));
        assert_eq!(element_ref.to_ref_string(), "clean:e12345");
    }

    #[test]
    fn test_parse_invalid_no_prefix() {
        let result = ElementRef::parse("12345");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_non_numeric() {
        let result = ElementRef::parse("eabc123");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_empty() {
        let result = ElementRef::parse("e");
        assert!(result.is_err());
    }

    #[test]
    fn test_new_and_display() {
        let element_ref = ElementRef::new("e42");
        assert_eq!(format!("{}", element_ref), "e42");
    }

    #[test]
    fn test_with_context() {
        let element_ref = ElementRef::with_context("e42", "main");
        assert_eq!(element_ref.to_ref_string(), "main:e42");
    }
}
