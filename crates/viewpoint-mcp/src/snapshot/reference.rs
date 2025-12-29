//! Element reference system for stable element identification

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// An element reference for targeting elements in tool calls
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementRef {
    /// The hash-based identifier (4-6 characters)
    pub hash: String,

    /// Optional context prefix for multi-context mode
    pub context: Option<String>,
}

impl ElementRef {
    /// Create a new element reference
    #[must_use]
    pub fn new(hash: impl Into<String>) -> Self {
        Self {
            hash: hash.into(),
            context: None,
        }
    }

    /// Create a new element reference with context prefix
    #[must_use]
    pub fn with_context(hash: impl Into<String>, context: impl Into<String>) -> Self {
        Self {
            hash: hash.into(),
            context: Some(context.into()),
        }
    }

    /// Format the reference as a string
    #[must_use]
    pub fn to_ref_string(&self) -> String {
        match &self.context {
            Some(ctx) => format!("{}:e{}", ctx, self.hash),
            None => format!("e{}", self.hash),
        }
    }

    /// Parse a reference string into an `ElementRef`
    ///
    /// # Errors
    ///
    /// Returns an error if the format is invalid
    pub fn parse(s: &str) -> Result<Self, String> {
        // Check for context prefix (e.g., "clean:e1a2b")
        if let Some((context, rest)) = s.split_once(':') {
            if let Some(hash) = rest.strip_prefix('e')
                && !hash.is_empty() {
                    return Ok(Self {
                        hash: hash.to_string(),
                        context: Some(context.to_string()),
                    });
                }
            return Err(format!(
                "Invalid reference format: '{s}'. Expected format: e<hash> or <context>:e<hash>"
            ));
        }

        // No context prefix (e.g., "e1a2b")
        if let Some(hash) = s.strip_prefix('e')
            && !hash.is_empty() {
                return Ok(Self {
                    hash: hash.to_string(),
                    context: None,
                });
            }

        Err(format!(
            "Invalid reference format: '{s}'. Expected format: e<hash> or <context>:e<hash>"
        ))
    }
}

impl std::fmt::Display for ElementRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ref_string())
    }
}

/// Generator for stable element references
#[derive(Debug, Default)]
pub struct RefGenerator {
    /// Context name for multi-context mode
    context: Option<String>,
}

impl RefGenerator {
    /// Create a new reference generator
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a generator with a context prefix
    #[must_use]
    pub fn with_context(context: impl Into<String>) -> Self {
        Self {
            context: Some(context.into()),
        }
    }

    /// Generate a reference for an element based on its attributes
    ///
    /// Priority:
    /// 1. `id` attribute (most stable)
    /// 2. `data-testid` or `data-test` attribute
    /// 3. `name` attribute
    /// 4. Fallback: role + name + DOM path
    #[must_use]
    pub fn generate(
        &self,
        id: Option<&str>,
        test_id: Option<&str>,
        name_attr: Option<&str>,
        role: &str,
        accessible_name: Option<&str>,
        dom_path: &str,
    ) -> ElementRef {
        let hash = Self::compute_hash(id, test_id, name_attr, role, accessible_name, dom_path);
        ElementRef {
            hash,
            context: self.context.clone(),
        }
    }

    /// Compute a stable hash from element attributes
    fn compute_hash(
        id: Option<&str>,
        test_id: Option<&str>,
        name_attr: Option<&str>,
        role: &str,
        accessible_name: Option<&str>,
        dom_path: &str,
    ) -> String {
        let mut hasher = DefaultHasher::new();

        // Priority 1: Use id attribute
        if let Some(id) = id
            && !id.is_empty() {
                "id:".hash(&mut hasher);
                id.hash(&mut hasher);
                return Self::format_hash(hasher.finish());
            }

        // Priority 2: Use test id
        if let Some(test_id) = test_id
            && !test_id.is_empty() {
                "testid:".hash(&mut hasher);
                test_id.hash(&mut hasher);
                return Self::format_hash(hasher.finish());
            }

        // Priority 3: Use name attribute
        if let Some(name) = name_attr
            && !name.is_empty() {
                "name:".hash(&mut hasher);
                name.hash(&mut hasher);
                return Self::format_hash(hasher.finish());
            }

        // Priority 4: Fallback to role + accessible name + DOM path
        "fallback:".hash(&mut hasher);
        role.hash(&mut hasher);
        if let Some(name) = accessible_name {
            name.hash(&mut hasher);
        }
        dom_path.hash(&mut hasher);

        Self::format_hash(hasher.finish())
    }

    /// Format a hash as a 4-6 character hex string
    fn format_hash(hash: u64) -> String {
        // Take lower 24 bits for a 6-character hex string
        let short_hash = hash & 0x00FF_FFFF;
        format!("{short_hash:06x}")
    }
}
