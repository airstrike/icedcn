//! Calendar day styling: the cell behind a day and its day button.
//!
//! A day's paint comes from the DayPicker modifiers Nova styles. The cell
//! carries the range ends' muted fill and its `after:` block, today's fill and
//! the disabled `opacity-50`; the day button paints on top of it.

use iced_core::{Background, Border};
use iced_widget::button;

use super::Theme;
use super::button::state::{IntoStyle, Status, Style, StyleFn};
use crate::{Color, Radius};

/// `--cell-radius: var(--radius-md)`.
const CELL: f32 = 8.0;
/// The day button's own `rounded-lg`.
const BUTTON: f32 = 10.0;

/// The modifiers of one day, as DayPicker sets them.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Day {
    pub selected: bool,
    pub range_start: bool,
    pub range_end: bool,
    pub range_middle: bool,
    pub today: bool,
    pub outside: bool,
    pub disabled: bool,
    /// A booked day: struck through, its cell kept at `opacity-100`.
    pub struck: bool,
    /// The first day of its week (`:first-child`).
    pub first: bool,
    /// The last day of its week (`:last-child`).
    pub last: bool,
    /// The week runs right to left; corners and the `after:` block mirror.
    pub rtl: bool,
}

impl Day {
    /// Selected without a range modifier (`data-selected-single`).
    pub fn single(&self) -> bool {
        self.selected && !self.range_start && !self.range_end && !self.range_middle
    }

    /// A range end isolates its cell (`isolate z-0`), so every other day
    /// button paints above it and its focus ring.
    pub fn layered(&self) -> bool {
        self.range_start || self.range_end
    }

    fn opacity(&self) -> f32 {
        if self.disabled && !self.struck {
            0.5
        } else {
            1.0
        }
    }

    /// The muted fill behind the day button and its corner radii, if any.
    pub fn cell(&self, theme: &Theme) -> Option<(Color, Radius)> {
        if !(self.range_start || self.range_end || self.today) {
            return None;
        }
        // `data-[selected=true]:rounded-none` on today outranks the range ends.
        let corner = if self.range_middle || self.today && self.selected {
            0.0
        } else {
            CELL
        };
        let mut radius = Radius::from(corner);
        if self.layered() {
            // A 16px `after:` block squares the half it covers. When a day both
            // starts and ends the range, `left-0` wins over `right-0`.
            let left = self.range_end != self.rtl;
            radius = if left {
                radius.left(0.0)
            } else {
                radius.right(0.0)
            };
        }
        Some((theme.muted.base.scale_alpha(self.opacity()), radius))
    }

    /// The day number's color at rest, cell opacity included.
    pub fn foreground(&self, theme: &Theme) -> Color {
        self.ink(theme).scale_alpha(self.opacity())
    }

    fn ink(&self, theme: &Theme) -> Color {
        if self.range_start || self.range_end || self.single() {
            theme.primary.foreground
        } else if self.range_middle || !(self.outside || self.disabled) {
            theme.neutral.foreground
        } else {
            theme.muted.foreground
        }
    }

    /// The day button for the given interaction state.
    pub fn button(&self, theme: &Theme, status: Status) -> Style {
        let ends = self.range_start || self.range_end;
        let mut background = if ends || self.single() {
            Some(theme.primary.base)
        } else if self.range_middle {
            Some(theme.muted.base)
        } else {
            None
        };
        let mut text = self.ink(theme);
        if status.hovered && !status.disabled {
            if theme.neutral.is_dark {
                // `dark:hover:bg-muted/50` and `dark:hover:text-foreground`
                // outrank every selection color.
                background = Some(theme.muted.base.scale_alpha(0.5));
                text = theme.neutral.foreground;
            } else if background.is_none() {
                background = Some(theme.muted.base);
                text = theme.neutral.foreground;
            }
        }
        let corner = if ends {
            CELL
        } else if self.range_middle {
            0.0
        } else {
            BUTTON
        };
        let mut radius = Radius::from(corner);
        if self.selected {
            let (left, right) = if self.rtl {
                (self.last, self.first)
            } else {
                (self.first, self.last)
            };
            if left {
                radius = radius.left(CELL);
            }
            if right {
                radius = radius.right(CELL);
            }
        }
        let opacity = self.opacity() * if status.disabled { 0.5 } else { 1.0 };
        let ring = (status.focused && !status.disabled).then(|| Border {
            color: theme.neutral.ring.scale_alpha(0.5 * opacity).into(),
            width: 3.0,
            ..Border::default()
        });
        Style {
            paint: button::Style {
                background: background.map(|c| Background::Color(c.scale_alpha(opacity).into())),
                text_color: text.scale_alpha(opacity).into(),
                border: Border {
                    radius: radius.into(),
                    ..Border::default()
                },
                snap: true,
                ..button::Style::default()
            },
            ring,
            ..Style::default()
        }
    }
}

impl<'a> IntoStyle<'a> for Day {
    fn into_style(self) -> StyleFn<'a> {
        Box::new(move |theme, status| self.button(theme, status))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corners(radius: Radius) -> [f32; 4] {
        [
            radius.top_left,
            radius.top_right,
            radius.bottom_right,
            radius.bottom_left,
        ]
    }

    #[test]
    fn range_cells_square_the_side_their_after_block_covers() {
        let theme = Theme::light();
        let start = Day {
            selected: true,
            range_start: true,
            ..Day::default()
        };
        let end = Day {
            range_start: false,
            range_end: true,
            ..start
        };
        let both = Day {
            range_end: true,
            ..start
        };
        let cell = |day: Day| corners(day.cell(&theme).expect("muted cell").1);
        assert_eq!(cell(start), [8., 0., 0., 8.]);
        assert_eq!(cell(end), [0., 8., 8., 0.]);
        assert_eq!(cell(both), [0., 8., 8., 0.], "left-0 wins over right-0");
        assert_eq!(
            cell(Day { rtl: true, ..both }),
            [8., 0., 0., 8.],
            "a right-to-left week mirrors the block"
        );
        let today = Day {
            today: true,
            ..Day::default()
        };
        assert_eq!(cell(today), [8.; 4]);
        assert_eq!(cell(Day { today: true, ..end }), [0.; 4]);
        assert_eq!(
            cell(Day {
                today: true,
                selected: true,
                ..Day::default()
            }),
            [0.; 4]
        );
        assert!(Day::default().cell(&theme).is_none());
        let middle = Day {
            selected: true,
            range_middle: true,
            ..Day::default()
        };
        assert!(middle.cell(&theme).is_none());
    }

    #[test]
    fn day_buttons_round_only_the_outer_corners_of_a_selected_week_edge() {
        let theme = Theme::light();
        let at = |day: Day| {
            corners(
                day.button(&theme, Status::default())
                    .paint
                    .border
                    .radius
                    .into(),
            )
        };
        let middle = Day {
            selected: true,
            range_middle: true,
            ..Day::default()
        };
        assert_eq!(at(middle), [0.; 4]);
        assert_eq!(
            at(Day {
                first: true,
                ..middle
            }),
            [8., 0., 0., 8.]
        );
        assert_eq!(
            at(Day {
                last: true,
                ..middle
            }),
            [0., 8., 8., 0.]
        );
        let single = Day {
            selected: true,
            first: true,
            ..Day::default()
        };
        assert_eq!(at(single), [8., 10., 10., 8.]);
        assert_eq!(at(Day::default()), [10.; 4]);
        assert_eq!(
            at(Day {
                first: true,
                ..Day::default()
            }),
            [10.; 4],
            "an unselected edge keeps the button radius"
        );
    }

    #[test]
    fn dark_hover_outranks_selection_and_disabled_cells_fade_twice() {
        let dark = Theme::dark();
        let hovered = Status {
            hovered: true,
            ..Status::default()
        };
        let start = Day {
            selected: true,
            range_start: true,
            ..Day::default()
        };
        let paint = start.button(&dark, hovered).paint;
        assert_eq!(
            paint.background,
            Some(Background::Color(dark.muted.base.scale_alpha(0.5).into()))
        );
        assert_eq!(paint.text_color, dark.neutral.foreground.into());
        let light = Theme::light();
        let paint = start.button(&light, hovered).paint;
        assert_eq!(
            paint.background,
            Some(Background::Color(light.primary.base.into()))
        );
        let outside = Day {
            outside: true,
            ..Day::default()
        };
        assert_eq!(
            outside.button(&light, hovered).paint.text_color,
            light.neutral.foreground.into(),
            "hover:text-foreground"
        );
        let inside = Day {
            selected: true,
            range_middle: true,
            outside: true,
            ..Day::default()
        };
        assert_eq!(
            inside.button(&light, Status::default()).paint.text_color,
            light.neutral.foreground.into(),
            "a selected outside day keeps the range's text color"
        );
        let disabled = Status {
            disabled: true,
            ..Status::default()
        };
        let faded = Day {
            disabled: true,
            ..Day::default()
        };
        let text = faded.button(&light, disabled).paint.text_color;
        assert!((text.a - 0.25 * light.muted.foreground.a).abs() < 1e-6);
        let booked = Day {
            struck: true,
            ..faded
        };
        let text = booked.button(&light, disabled).paint.text_color;
        assert!((text.a - 0.5 * light.muted.foreground.a).abs() < 1e-6);
    }
}
