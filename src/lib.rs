//! Themed widgets for iced applications. Compose controls and layouts with the widget builders
//! and customize their appearance with the shared theme and design tokens.

pub mod card;
#[cfg(feature = "fount")]
pub mod fount;
pub mod separator;

// Re-export `card::card` as `icedcn::card` so `use icedcn::card;` brings
// both the module (for `card::title(...)`, `card::header(...)`, etc.) and
// the convenience constructor (for `card(my_content)`) into scope. Same
// pattern `widget/src/lib.rs` uses for `field` / `checkbox` / `text` / etc.
pub use card::card;
pub use separator::separator;

pub mod core {
    pub use icedcn_core::*;
}
pub use icedcn_core::padding;
pub use icedcn_core::theme;
pub use icedcn_core::{Color, Design, FontFamily, Gap, Padding, Palette, Space, Theme};
pub use icedcn_core::{font_size, height, icon_size, radius, spacing};

// Layout-axis primitive shared across widgets (Field, ButtonGroup, ...).
// Re-exported at the crate root with the bare variants so callers reach
// for `use icedcn::{Vertical, Horizontal};` and pass them directly into
// any `.orientation()` builder, no FQN dance per-widget.
pub use icedcn_core::Orientation;
pub use icedcn_core::Orientation::{Horizontal, Vertical};

// The shared style variants: `.style(Destructive)` on any component.
pub use icedcn_core::theme::{Destructive, Ghost, Invalid, Link, Muted, Outline, Secondary};

#[allow(hidden_glob_reexports)]
pub mod widget {
    pub use icedcn_widget::*;
}

// Re-export the `rich_text!` macro at our crate root so consumers don't
// need a direct `icedcn_widget` dep. Type-level `RichText` / `Span` /
// `span` / `rich_text` (the constructor fn) are available via the
// `widget` glob above. `#[doc(inline)]` keeps the macro visible in
// rustdoc; `#[macro_export]` from `icedcn_widget` makes it available
// at `icedcn_widget::rich_text` and we surface it here.
#[doc(inline)]
pub use icedcn_widget::rich_text;

pub use widget::Size;
pub use widget::Weight;
pub use widget::{Compose, Inherited, Slot};

/// Raw iced widgets with modifications (centering, focus/blur, etc.)
pub mod raw {
    pub use icedcn_widget::button::widget::Button;
}
