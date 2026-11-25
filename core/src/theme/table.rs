//! Table styling

pub use iced_widget::table::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    let p = theme;

    Style {
        separator_x: p.neutral.border.into(),
        separator_y: p.neutral.border.into(),
    }
}

/// Style for the enhanced table. The sticky header uses the background color, and the row
/// separators and outer border share the border color.
#[cfg(feature = "sweeten")]
pub mod sweeten_table {
    use sweeten::widget::table::{Catalog, Style, StyleFn};

    use super::Theme;

    impl Catalog for Theme {
        type Class<'a> = StyleFn<'a, Self>;

        fn default<'a>() -> Self::Class<'a> {
            Box::new(default)
        }

        fn style(&self, class: &Self::Class<'_>) -> Style {
            class(self)
        }
    }

    pub fn default(theme: &Theme) -> Style {
        let p = theme;
        let separator = p.neutral.border;

        Style {
            separator_x: separator.into(),
            separator_y: separator.into(),
            border: separator.into(),
            sticky_background: p.neutral.base.into(),
            header_underline: None,
        }
    }
}
