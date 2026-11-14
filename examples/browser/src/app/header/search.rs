//! The documentation site's command palette owns one panel and one search field.
use crate::icon;
use iced::widget::text;
use iced::{Center, Fill, Vector};
use icedxdy::core::{Border, Shadow};
use icedxdy::theme::{container::Style, input_group::control, text_input::StyleFn};
use icedxdy::widget::{Input, command, container, input_group};
use icedxdy::{Color, Slot, Theme, radius};

pub(super) const INSET: u16 = 8;
pub(super) const RIM: u16 = 4;

pub(super) fn panel(theme: &Theme) -> Style {
    let rim = if theme.neutral.is_dark {
        theme.muted.base
    } else {
        theme.neutral.border.scale_alpha(0.8)
    };
    Style {
        background: Some(theme.popover.base.into()),
        text_color: Some(theme.popover.foreground),
        // iced draws the border inward. One quad gives the 14px panel and
        // its outer 4px ring the same center at every corner and DPI.
        border: Border::default()
            .rounded(radius::XL + f32::from(RIM))
            .width(f32::from(RIM))
            .color(rim),
        shadow: Shadow {
            color: Color::BLACK.alpha(0.25),
            offset: Vector::new(0., 25.),
            blur_radius: 50.,
        },
        snap: true,
    }
}

pub(super) fn field(input: Input<'_, command::Event>) -> Slot<'_, command::Event> {
    let icon = icon::search().size(16).style(|theme: &Theme| text::Style {
        color: Some(theme.popover.foreground.scale_alpha(0.5).into()),
        ..text::Style::default()
    });
    let group = input_group(input)
        .addon(input_group::addon(icon))
        .style(Box::new(control) as StyleFn<'_, Theme>);
    container(group)
        .width(Fill)
        .height(36)
        .align_y(Center)
        .padding([0, 4])
        .style(|theme| {
            // CSS paints bg-input/50 under the border as well as the field.
            let fill = theme.input.scale_alpha(0.5).over(theme.popover.base);
            Style {
                background: Some(fill.into()),
                border: Border::default()
                    .width(1)
                    .rounded(radius::MD)
                    .color(theme.input.over(fill)),
                snap: true,
                ..Style::default()
            }
        })
        .into()
}
