//! Blue preview palette for conversation widgets.
use super::Backdrop;
use icedcn::core::{Border, Radius, oklch};
use icedcn::theme::container::Style;
use icedcn::{Color, Palette, Theme};

pub(crate) fn theme(outer: &Theme) -> Theme {
    let mut design = outer.design().clone();
    let dark = design.neutral.is_dark;
    let primary = if dark {
        oklch(0.424, 0.199, 265.638)
    } else {
        oklch(0.488, 0.243, 264.376)
    };
    design.primary = tint(primary, oklch(0.97, 0.014, 254.604), dark);
    design.secondary = if dark {
        tint(oklch(0.274, 0.006, 286.033), oklch(0.985, 0.0, 0.0), dark)
    } else {
        tint(
            oklch(0.967, 0.001, 286.375),
            oklch(0.21, 0.006, 285.885),
            dark,
        )
    };
    Theme::new(design)
}

fn tint(base: Color, foreground: Color, is_dark: bool) -> Palette {
    Palette {
        base,
        foreground,
        border: base,
        ring: base.alpha(0.5),
        tint_text: base,
        is_dark,
    }
}

fn canvas(theme: &Theme) -> Style {
    let surface = if theme.neutral.is_dark {
        theme.neutral.base
    } else {
        oklch(0.98, 0.0, 0.0)
    };
    Style {
        background: Some(surface.into()),
        border: Border {
            radius: Radius {
                top_left: 25.0,
                top_right: 25.0,
                ..Radius::default()
            },
            ..Border::default()
        },
        ..Style::default()
    }
}

fn border(theme: &Theme) -> Style {
    Style {
        background: Some(theme.neutral.base.into()),
        border: Border {
            width: 1.0,
            color: theme.neutral.border,
            radius: 26.0.into(),
        },
        ..Style::default()
    }
}

pub(crate) fn backdrop() -> Backdrop {
    Backdrop {
        padding: [64, 40].into(),
        surface: canvas,
        border,
        ..Backdrop::default()
    }
}
