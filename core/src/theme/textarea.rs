//! Multiline field styles. The editor shares input colors and border states.

use iced_widget::text_editor::{Catalog, Status, Style, StyleFn};

use super::Theme;
use super::text_input;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn input_status(status: Status) -> text_input::Status {
    match status {
        Status::Active => text_input::Status::Active,
        Status::Hovered => text_input::Status::Hovered,
        Status::Focused { is_hovered } => text_input::Status::Focused { is_hovered },
        Status::Disabled => text_input::Status::Disabled,
    }
}

fn project(style: text_input::Style) -> Style {
    Style {
        background: style.background.into(),
        border: style.border.into(),
        placeholder: style.placeholder.into(),
        value: style.value.into(),
        selection: style.selection.into(),
    }
}

/// The default textarea.
pub fn default(theme: &Theme, status: Status) -> Style {
    project(text_input::default(theme, input_status(status)))
}

/// An invalid textarea: a destructive border.
pub fn invalid(theme: &Theme, status: Status) -> Style {
    project(text_input::invalid(theme, input_status(status)))
}

/// Default textarea style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Invalid textarea style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invalid;

impl<'a> From<Default> for StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(default)
    }
}

impl<'a> From<Invalid> for StyleFn<'a, Theme> {
    fn from(_: Invalid) -> Self {
        Box::new(invalid)
    }
}

/// Built-in textarea treatments can draw an outer ring. An iced editor style
/// controls the editor surface directly.
pub enum Appearance<'a> {
    Default,
    Invalid,
    Custom(StyleFn<'a, Theme>),
}

impl<'a> Appearance<'a> {
    /// Resolve the editor surface and any surrounding ring together.
    pub fn resolve(&self, theme: &Theme, status: Status) -> (Style, Option<text_input::Ring>) {
        let field = match self {
            Self::Default => text_input::default(theme, input_status(status)),
            Self::Invalid => text_input::invalid(theme, input_status(status)),
            Self::Custom(style) => return (style(theme, status), None),
        };
        (project(field), field.ring)
    }
}

impl From<Default> for Appearance<'_> {
    fn from(_: Default) -> Self {
        Self::Default
    }
}

impl From<Invalid> for Appearance<'_> {
    fn from(_: Invalid) -> Self {
        Self::Invalid
    }
}

impl<'a, F> From<F> for Appearance<'a>
where
    F: Fn(&Theme, Status) -> Style + 'a,
{
    fn from(style: F) -> Self {
        Self::Custom(Box::new(style))
    }
}
