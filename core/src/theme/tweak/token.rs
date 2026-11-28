//! Palette slot tokens — one variant per addressable color slot, plus a
//! `Custom` escape hatch for arbitrary colors.
//!
//! Mirrors the `widget::Size` precedent: named variants + `Custom(_)` +
//! `From<Color>` blanket. See `widget/src/size.rs`.

use crate::{Color, Theme};

/// A semantic color slot on the active theme, or a literal color.
///
/// Resolves to the crate's `Color` (OKLCH) against a given `Theme` at
/// style computation time. Convert to iced types via the existing
/// `From<Color>` impls at the apply site.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Background,
    Foreground,
    Card,
    CardForeground,
    Popover,
    PopoverForeground,
    Primary,
    PrimaryForeground,
    Secondary,
    SecondaryForeground,
    Muted,
    MutedForeground,
    Accent,
    AccentForeground,
    Destructive,
    DestructiveForeground,
    Info,
    InfoForeground,
    Success,
    SuccessForeground,
    Warning,
    WarningForeground,
    Code,
    CodeForeground,
    Border,
    Input,
    Ring,
    Sidebar,
    SidebarForeground,
    SidebarPrimary,
    SidebarPrimaryForeground,
    SidebarAccent,
    SidebarAccentForeground,
    SidebarBorder,
    SidebarRing,
    Custom(Color),
    /// A literal color pair that follows the active light or dark theme.
    Themed {
        light: Color,
        dark: Color,
    },
}

/// Define a role enum named `$name` beside [`Token`], with one variant per
/// theme role, that becomes `$tweak(token)`. Listing the roles here and in
/// `Token` is checked: a role missing from either side fails to compile.
macro_rules! roles {
    ($(#[$doc:meta])* $name:ident => $tweak:path) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            Background,
            Foreground,
            Card,
            CardForeground,
            Popover,
            PopoverForeground,
            Primary,
            PrimaryForeground,
            Secondary,
            SecondaryForeground,
            Muted,
            MutedForeground,
            Accent,
            AccentForeground,
            Destructive,
            DestructiveForeground,
            Info,
            InfoForeground,
            Success,
            SuccessForeground,
            Warning,
            WarningForeground,
            Code,
            CodeForeground,
            Border,
            Input,
            Ring,
            Sidebar,
            SidebarForeground,
            SidebarPrimary,
            SidebarPrimaryForeground,
            SidebarAccent,
            SidebarAccentForeground,
            SidebarBorder,
            SidebarRing,
        }

        impl From<$name> for $crate::theme::tweak::Token {
            fn from(role: $name) -> Self {
                match role {
                    $name::Background => Self::Background, $name::Foreground => Self::Foreground, $name::Card => Self::Card, $name::CardForeground => Self::CardForeground, $name::Popover => Self::Popover, $name::PopoverForeground => Self::PopoverForeground, $name::Primary => Self::Primary, $name::PrimaryForeground => Self::PrimaryForeground, $name::Secondary => Self::Secondary, $name::SecondaryForeground => Self::SecondaryForeground, $name::Muted => Self::Muted, $name::MutedForeground => Self::MutedForeground, $name::Accent => Self::Accent, $name::AccentForeground => Self::AccentForeground, $name::Destructive => Self::Destructive, $name::DestructiveForeground => Self::DestructiveForeground, $name::Info => Self::Info, $name::InfoForeground => Self::InfoForeground, $name::Success => Self::Success, $name::SuccessForeground => Self::SuccessForeground, $name::Warning => Self::Warning, $name::WarningForeground => Self::WarningForeground, $name::Code => Self::Code, $name::CodeForeground => Self::CodeForeground, $name::Border => Self::Border, $name::Input => Self::Input, $name::Ring => Self::Ring, $name::Sidebar => Self::Sidebar, $name::SidebarForeground => Self::SidebarForeground, $name::SidebarPrimary => Self::SidebarPrimary, $name::SidebarPrimaryForeground => Self::SidebarPrimaryForeground, $name::SidebarAccent => Self::SidebarAccent, $name::SidebarAccentForeground => Self::SidebarAccentForeground, $name::SidebarBorder => Self::SidebarBorder, $name::SidebarRing => Self::SidebarRing,
                }
            }
        }

        impl From<$name> for $crate::theme::tweak::Tweak {
            fn from(role: $name) -> Self {
                $tweak(role.into())
            }
        }

        impl<R: Into<$crate::theme::tweak::Tweak>> std::ops::Add<R> for $name {
            type Output = $crate::theme::tweak::TweakSet;
            fn add(self, rhs: R) -> Self::Output {
                $crate::theme::tweak::Tweak::from(self) + rhs
            }
        }

        const _: fn($crate::theme::tweak::Token) = |token| match token {
            $crate::theme::tweak::Token::Background | $crate::theme::tweak::Token::Foreground | $crate::theme::tweak::Token::Card | $crate::theme::tweak::Token::CardForeground | $crate::theme::tweak::Token::Popover | $crate::theme::tweak::Token::PopoverForeground | $crate::theme::tweak::Token::Primary | $crate::theme::tweak::Token::PrimaryForeground | $crate::theme::tweak::Token::Secondary | $crate::theme::tweak::Token::SecondaryForeground | $crate::theme::tweak::Token::Muted | $crate::theme::tweak::Token::MutedForeground | $crate::theme::tweak::Token::Accent | $crate::theme::tweak::Token::AccentForeground | $crate::theme::tweak::Token::Destructive | $crate::theme::tweak::Token::DestructiveForeground | $crate::theme::tweak::Token::Info | $crate::theme::tweak::Token::InfoForeground | $crate::theme::tweak::Token::Success | $crate::theme::tweak::Token::SuccessForeground | $crate::theme::tweak::Token::Warning | $crate::theme::tweak::Token::WarningForeground | $crate::theme::tweak::Token::Code | $crate::theme::tweak::Token::CodeForeground | $crate::theme::tweak::Token::Border | $crate::theme::tweak::Token::Input | $crate::theme::tweak::Token::Ring | $crate::theme::tweak::Token::Sidebar | $crate::theme::tweak::Token::SidebarForeground | $crate::theme::tweak::Token::SidebarPrimary | $crate::theme::tweak::Token::SidebarPrimaryForeground | $crate::theme::tweak::Token::SidebarAccent | $crate::theme::tweak::Token::SidebarAccentForeground | $crate::theme::tweak::Token::SidebarBorder | $crate::theme::tweak::Token::SidebarRing => {}
            $crate::theme::tweak::Token::Custom(_) | $crate::theme::tweak::Token::Themed { .. } => {}
        };
    };
}

pub(crate) use roles;

impl From<Color> for Token {
    fn from(c: Color) -> Self {
        Token::Custom(c)
    }
}

impl Token {
    /// Use one color in light mode and another in dark mode.
    pub fn themed(light: impl Into<Color>, dark: impl Into<Color>) -> Self {
        Self::Themed {
            light: light.into(),
            dark: dark.into(),
        }
    }

    /// Resolve this token to a concrete color against the given theme.
    pub fn resolve(self, theme: &Theme) -> Color {
        match self {
            Token::Background => theme.neutral.base,
            Token::Foreground => theme.neutral.foreground,
            Token::Card => theme.card.base,
            Token::CardForeground => theme.card.foreground,
            Token::Popover => theme.popover.base,
            Token::PopoverForeground => theme.popover.foreground,
            Token::Primary => theme.primary.base,
            Token::PrimaryForeground => theme.primary.foreground,
            Token::Secondary => theme.secondary.base,
            Token::SecondaryForeground => theme.secondary.foreground,
            Token::Muted => theme.muted.base,
            Token::MutedForeground => theme.muted.foreground,
            Token::Accent => theme.accent.base,
            Token::AccentForeground => theme.accent.foreground,
            Token::Destructive => theme.destructive.base,
            Token::DestructiveForeground => theme.destructive.foreground,
            Token::Info => theme.info.base,
            Token::InfoForeground => theme.info.foreground,
            Token::Success => theme.success.base,
            Token::SuccessForeground => theme.success.foreground,
            Token::Warning => theme.warning.base,
            Token::WarningForeground => theme.warning.foreground,
            Token::Code => theme.code.base,
            Token::CodeForeground => theme.code.foreground,
            Token::Border => theme.neutral.border,
            Token::Input => theme.input,
            Token::Ring => theme.neutral.ring,
            Token::Sidebar => theme.sidebar.background,
            Token::SidebarForeground => theme.sidebar.foreground,
            Token::SidebarPrimary => theme.sidebar.primary.base,
            Token::SidebarPrimaryForeground => theme.sidebar.primary.foreground,
            Token::SidebarAccent => theme.sidebar.accent.base,
            Token::SidebarAccentForeground => theme.sidebar.accent.foreground,
            Token::SidebarBorder => theme.sidebar.border,
            Token::SidebarRing => theme.sidebar.ring,
            Token::Custom(c) => c,
            Token::Themed { light, dark } => {
                if theme.neutral.is_dark {
                    dark
                } else {
                    light
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themed_token_resolves_literal_colors_in_both_modes() {
        let light = iced_core::color!(0xffedd5);
        let dark = iced_core::color!(0x9a3412);
        let token = Token::themed(light, dark);
        assert_eq!(token.resolve(&Theme::light()), Color::from(light));
        assert_eq!(token.resolve(&Theme::dark()), Color::from(dark));
    }
}
