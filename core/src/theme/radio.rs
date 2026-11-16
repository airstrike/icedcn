//! Radio styles for a circular control with a centered selection indicator.

pub use iced_widget::radio::{Status, Style};

use super::Theme;
use crate::Color;

impl iced_widget::radio::Catalog for Theme {
    type Class<'a> = iced_widget::radio::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> iced_widget::radio::Style {
        class(self, status)
    }

    fn selection(&self) -> iced_core::Color {
        self.selection()
    }
}

fn is_dark(theme: &Theme) -> bool {
    theme.neutral.base.l < 0.5
}

fn is_selected(status: Status) -> bool {
    match status {
        Status::Active { is_selected } | Status::Hovered { is_selected } => is_selected,
    }
}

/// Default radio variant: nova `size-4` control with a primary selected fill.
///
/// Light unchecked bg = transparent; dark unchecked bg = `palette.input` at
/// alpha 0.3 (`dark:bg-input/30`). Checked bg = primary; checked dot =
/// primary foreground; border at rest = input, selected = primary.
pub fn default(theme: &Theme, status: Status) -> iced_widget::radio::Style {
    let p = theme;
    let selected = is_selected(status);
    let background = if selected {
        p.primary.base
    } else if is_dark(theme) {
        p.input.scale_alpha(0.3)
    } else {
        Color::TRANSPARENT
    };
    let border = if selected { p.primary.base } else { p.input };

    iced_widget::radio::Style {
        background: background.into(),
        dot_color: p.primary.foreground.into(),
        border_width: 1.0,
        border_color: border.into(),
        text_color: None,
    }
}

/// Disabled radio: the [`default`] look at `disabled:opacity-50`, with the
/// label in `muted-foreground` like a disabled checkbox or switch.
pub fn disabled(theme: &Theme, status: Status) -> iced_widget::radio::Style {
    dimmed(theme, default(theme, status))
}

/// Any radio `style` at `disabled:opacity-50`, its label muted.
pub fn dimmed(theme: &Theme, mut style: iced_widget::radio::Style) -> iced_widget::radio::Style {
    style.background = style.background.scale_alpha(0.5);
    style.dot_color.a *= 0.5;
    style.border_color.a *= 0.5;
    style.text_color = Some(theme.muted.foreground.into());
    style
}
