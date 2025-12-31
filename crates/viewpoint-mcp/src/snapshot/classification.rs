//! Element classification for determining which elements receive refs

/// Classification tier for accessibility elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementTier {
    /// Tier 1: Always interactive - always receive refs
    /// Includes: button, link, textbox, checkbox, radio, combobox, slider,
    /// menuitem, menuitemcheckbox, menuitemradio, tab, switch, searchbox, spinbutton
    AlwaysInteractive,

    /// Tier 2: Contextually interactive - receive refs when in interactive containers
    /// Includes: listitem, option, treeitem, row, cell
    ContextuallyInteractive,

    /// Tier 3: Structural/non-interactive - never receive refs
    /// Includes: heading, paragraph, text, separator, img, figure, main, navigation, etc.
    NonInteractive,
}

/// Roles that are always interactive (Tier 1)
const TIER1_ROLES: &[&str] = &[
    "button",
    "link",
    "textbox",
    "checkbox",
    "radio",
    "combobox",
    "slider",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "tab",
    "switch",
    "searchbox",
    "spinbutton",
    "scrollbar",
    "progressbar",
];

/// Roles that are contextually interactive (Tier 2)
const TIER2_ROLES: &[&str] = &[
    "listitem",
    "option",
    "treeitem",
    "row",
    "cell",
    "gridcell",
    "columnheader",
    "rowheader",
];

/// Roles that are structural/non-interactive (Tier 3)
const TIER3_ROLES: &[&str] = &[
    "heading",
    "paragraph",
    "text",
    "separator",
    "img",
    "figure",
    "main",
    "navigation",
    "banner",
    "contentinfo",
    "complementary",
    "region",
    "article",
    "document",
    "group",
    "list",
    "table",
    "tree",
    "grid",
    "menu",
    "menubar",
    "tablist",
    "toolbar",
    "status",
    "alert",
    "log",
    "marquee",
    "timer",
    "none",
    "presentation",
];

/// Classify an element's role into its interaction tier
#[must_use]
pub fn classify_role(role: &str) -> ElementTier {
    let role_lower = role.to_lowercase();

    if TIER1_ROLES.contains(&role_lower.as_str()) {
        ElementTier::AlwaysInteractive
    } else if TIER2_ROLES.contains(&role_lower.as_str()) {
        ElementTier::ContextuallyInteractive
    } else if TIER3_ROLES.contains(&role_lower.as_str()) {
        ElementTier::NonInteractive
    } else {
        // Unknown roles default to non-interactive
        ElementTier::NonInteractive
    }
}

/// Check if a role should receive a ref based on tier and context
///
/// Note: This function is primarily used for testing and documentation.
/// In production, viewpoint-core's `node_ref` field determines which elements
/// receive refs.
#[allow(dead_code)]
#[must_use]
pub fn should_receive_ref(
    role: &str,
    in_interactive_container: bool,
    has_tabindex: bool,
) -> bool {
    // Elements with tabindex >= 0 always receive refs
    if has_tabindex {
        return true;
    }

    match classify_role(role) {
        ElementTier::AlwaysInteractive => true,
        ElementTier::ContextuallyInteractive => in_interactive_container,
        ElementTier::NonInteractive => false,
    }
}

/// Interactive container roles that make Tier 2 children interactive
const INTERACTIVE_CONTAINERS: &[&str] = &[
    "listbox",
    "combobox",
    "tree",
    "grid",
    "menu",
    "menubar",
    "tablist",
    "radiogroup",
];

/// Check if a role represents an interactive container
#[must_use]
pub fn is_interactive_container(role: &str) -> bool {
    INTERACTIVE_CONTAINERS.contains(&role.to_lowercase().as_str())
}
