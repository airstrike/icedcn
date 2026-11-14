//! Palette — a single Hue.
//!
//! The active color context for whatever's currently being styled.
//! `Theme::tinted(tint)` returns one of `Design`'s per-role palettes;
//! all slots resolve in lockstep against that role.
//!
//! The light and dark designs each define OKLCH colors for every role.
//!
//! `SidebarPalette` stays as a side-field on `Design` for now; a sidebar
//! context that swaps the full Design is deferred.

use crate::color::Color;

/// A single Hue — the colors that everything styled under this palette
/// pulls from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    /// Primary fill / body color of the active role.
    pub base: Color,
    /// Text color on top of `base`.
    pub foreground: Color,
    /// Border drawn from this role.
    pub border: Color,
    /// Focus ring color.
    pub ring: Color,
    /// Text color for transparent-fill shapes (outline / ghost / link).
    /// Tinted hues set this to `base`; neutral and surfaces set it to
    /// `foreground` (neutral's `base` is the page bg, not page text).
    pub tint_text: Color,
    /// Whether this palette belongs to a dark theme. Styles use this flag to adjust opacity for
    /// dark surfaces.
    pub is_dark: bool,
}

/// Sidebar-specific color palette.
///
/// Deferred reshape — kept as the legacy shape for Phase 1; a future
/// pass dissolves this into a swapped `Design` via a sidebar context.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarPalette {
    pub background: Color,
    pub foreground: Color,
    pub primary: SidebarPair,
    pub accent: SidebarPair,
    pub border: Color,
    pub ring: Color,
}

/// Local base+foreground pair for `SidebarPalette`. Replaces the old
/// `SemanticColor` used inside the sidebar struct.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarPair {
    pub base: Color,
    pub foreground: Color,
}
