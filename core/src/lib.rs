//! Design tokens, colors, and theme definitions shared by icedcn widgets.

pub mod background;
pub mod border;
mod color;
mod constants;
pub mod design;
mod orientation;
pub mod padding;
mod palette;
pub mod shadow;
pub mod style;
pub mod theme;

pub use background::Background;
pub use border::{Border, Radius, radius};
pub use color::{Color, oklch, oklch_alpha};
pub use constants::*;
pub use design::{Design, FontFamily};
pub use orientation::Orientation;
// Re-export the variants at module scope so a bare `Vertical` /
// `Horizontal` import from `icedcn_core` (or, transitively, from `icedcn`)
// can be passed directly to any `.orientation()` builder via `impl Into`.
pub use orientation::Orientation::{Horizontal, Vertical};
pub use padding::Padding;
pub use palette::{Palette, SidebarPalette};
pub use shadow::Shadow;
pub use theme::Theme;
