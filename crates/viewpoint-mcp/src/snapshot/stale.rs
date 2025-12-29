//! Stale reference detection and recovery guidance

use std::collections::HashMap;

use super::element::SnapshotElement;
use super::reference::ElementRef;

/// Error type for stale reference detection
#[derive(Debug, Clone)]
pub enum StaleRefError {
    /// Element no longer exists
    ElementRemoved {
        /// The ref that was not found
        ref_string: String,
        /// The original element description
        original_description: String,
        /// Similar elements that might be what the user meant
        similar_elements: Vec<SimilarElement>,
    },

    /// Element exists but has changed significantly
    ElementChanged {
        /// The ref that matched
        ref_string: String,
        /// What the element was before
        was: String,
        /// What the element is now
        now: String,
    },

    /// Element has minor changes (action can proceed with warning)
    MinorChange {
        /// The ref that matched
        ref_string: String,
        /// Description of the change
        change_description: String,
    },
}

impl std::fmt::Display for StaleRefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ElementRemoved {
                ref_string,
                original_description,
                similar_elements,
            } => {
                writeln!(
                    f,
                    "Element '{original_description}' (ref: {ref_string}) no longer exists."
                )?;
                if !similar_elements.is_empty() {
                    writeln!(f, "Similar elements on page:")?;
                    for elem in similar_elements.iter().take(3) {
                        writeln!(f, "  - {} [ref={}]", elem.description, elem.ref_string)?;
                    }
                }
                write!(f, "Take a new snapshot to see current page state.")
            }
            Self::ElementChanged {
                ref_string: _,
                was,
                now,
            } => {
                writeln!(f, "Element changed since snapshot.")?;
                writeln!(f, "Was: {was}")?;
                writeln!(f, "Now: {now}")?;
                write!(f, "Take a new snapshot to get current element state.")
            }
            Self::MinorChange {
                ref_string: _,
                change_description,
            } => {
                write!(
                    f,
                    "Note: Element may have changed ({change_description}). Using current state."
                )
            }
        }
    }
}

/// A similar element suggestion for recovery
#[derive(Debug, Clone)]
pub struct SimilarElement {
    /// The ref string for this element
    pub ref_string: String,
    /// A human-readable description
    pub description: String,
    /// Similarity score (0.0-1.0)
    pub similarity: f64,
}

/// Stored snapshot info for comparison
#[derive(Debug, Clone)]
pub struct SnapshotInfo {
    /// Map from ref hash to element info
    pub elements: HashMap<String, StoredElementInfo>,
}

/// Stored element info for staleness comparison
#[derive(Debug, Clone)]
pub struct StoredElementInfo {
    /// The element's role
    pub role: String,
    /// The element's accessible name
    pub name: Option<String>,
    /// Description for error messages
    pub description: String,
}

/// Detector for stale references
#[derive(Debug, Default)]
pub struct StaleRefDetector {
    /// Previous snapshot (for comparison)
    previous: Option<SnapshotInfo>,
    /// Current snapshot
    current: Option<SnapshotInfo>,
}

impl StaleRefDetector {
    /// Create a new stale ref detector
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update with a new snapshot, moving current to previous
    pub fn update(&mut self, snapshot: &SnapshotElement) {
        self.previous = self.current.take();
        self.current = Some(Self::extract_info(snapshot));
    }

    /// Check if a ref is valid and not stale
    ///
    /// Returns `Ok(())` if the ref is valid, or an appropriate error
    pub fn validate_ref(&self, element_ref: &ElementRef) -> Result<(), StaleRefError> {
        let Some(current) = &self.current else {
            return Ok(()); // No snapshot to validate against
        };

        let hash = &element_ref.hash;

        // Check if element exists in current snapshot
        if let Some(current_info) = current.elements.get(hash) {
            // Element exists - check if it changed from previous
            if let Some(previous) = &self.previous
                && let Some(previous_info) = previous.elements.get(hash) {
                    // Compare for significant changes
                    if current_info.role != previous_info.role {
                        return Err(StaleRefError::ElementChanged {
                            ref_string: element_ref.to_ref_string(),
                            was: previous_info.description.clone(),
                            now: current_info.description.clone(),
                        });
                    }

                    // Check for minor name changes
                    if current_info.name != previous_info.name {
                        return Err(StaleRefError::MinorChange {
                            ref_string: element_ref.to_ref_string(),
                            change_description: format!(
                                "name changed from {:?} to {:?}",
                                previous_info.name, current_info.name
                            ),
                        });
                    }
                }

            Ok(())
        } else {
            // Element not found - find similar elements
            let similar = Self::find_similar_elements(hash);

            // Get original description from previous snapshot if available
            let original_description = self
                .previous
                .as_ref()
                .and_then(|p| p.elements.get(hash)).map_or_else(|| format!("element {}", element_ref.to_ref_string()), |info| info.description.clone());

            Err(StaleRefError::ElementRemoved {
                ref_string: element_ref.to_ref_string(),
                original_description,
                similar_elements: similar,
            })
        }
    }

    /// Find elements similar to a missing ref
    fn find_similar_elements(_target_hash: &str) -> Vec<SimilarElement> {
        // For now, return empty - could implement fuzzy matching later
        Vec::new()
    }

    /// Extract element info from a snapshot tree
    fn extract_info(root: &SnapshotElement) -> SnapshotInfo {
        let mut elements = HashMap::new();
        Self::collect_elements(&mut elements, root);
        SnapshotInfo { elements }
    }

    /// Recursively collect elements with refs
    fn collect_elements(map: &mut HashMap<String, StoredElementInfo>, element: &SnapshotElement) {
        if let Some(element_ref) = &element.element_ref {
            let description = format!(
                "{} {}",
                element.role,
                element.name.as_deref().unwrap_or("")
            )
            .trim()
            .to_string();

            map.insert(
                element_ref.hash.clone(),
                StoredElementInfo {
                    role: element.role.clone(),
                    name: element.name.clone(),
                    description,
                },
            );
        }

        for child in &element.children {
            Self::collect_elements(map, child);
        }
    }
}
