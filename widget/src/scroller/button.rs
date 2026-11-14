//! The end control stays mounted while a change in scroll availability animates.
use crate::button;
use crate::{Slot, animation::cubic_bezier, container};
use iced_core::animation::Easing;
use iced_core::time::Instant;
use iced_core::{Animation, Element, Length::Fill, Padding, alignment::Vertical::Bottom};
use iced_widget::button::{Status, StyleFn};
use iced_widget::transition::Program;
use iced_widget::{Renderer, Widget, space, transition};
use icedxdy_core::Theme;
use icedxdy_core::theme::button::secondary;

struct Visibility(Animation<f32>);

impl Program for Visibility {
    type Value = bool;

    fn go(&mut self, show: bool, now: Instant) {
        let target = f32::from(show);
        if self.0.value() == target {
            return;
        }
        let current = self.0.interpolate(now);
        self.0 = if show {
            Animation::new(current)
                .quick()
                .easing(Easing::Custom(|x| cubic_bezier(0.23, 1., 0.32, 1., x)))
        } else {
            Animation::new(current)
                .slow()
                .easing(Easing::Custom(|x| cubic_bezier(0.7, 0., 0.84, 0., x)))
        }
        .go(target, now);
    }

    fn is_animating(&self, now: Instant) -> bool {
        self.0.is_animating(now)
    }
}

/// The end-of-transcript button layer, for use above a scroller in an iced stack.
/// `visible` is the reader's `can_scroll_down`. The icon receives its animated size.
pub(super) fn end<'a, Message: Clone + 'a, Icon: Into<Slot<'a, Message>>>(
    visible: bool,
    icon: impl Fn(f32) -> Icon + 'a,
    press: Message,
) -> Element<'a, Message, Theme, Renderer> {
    transition(
        visible,
        || Visibility(Animation::new(0.)),
        move |visibility, now| -> Element<'a, Message, Theme, Renderer> {
            let opacity = visibility.0.interpolate(now);
            if opacity <= 0. {
                return space().width(Fill).height(Fill).boxed();
            }
            let size = 28. * (0.95 + 0.05 * opacity);
            let style: StyleFn<'static, Theme> = Box::new(move |theme, status| {
                let mut style = secondary(theme, status);
                let fill = if matches!(status, Status::Hovered | Status::Pressed) {
                    theme.muted.base
                } else {
                    theme.neutral.base
                };
                style.background = Some(fill.scale_alpha(opacity).into());
                style.border.color = theme.neutral.border.scale_alpha(opacity);
                style.border.width = 1.;
                style.text_color = theme.neutral.foreground.scale_alpha(opacity);
                style.into()
            });
            let arrow = button(icon(16. * size / 28.))
                .rounded()
                .height(size)
                .square()
                .translate(false)
                .style(style)
                .disabled(!visible)
                .on_press(press.clone());
            container(arrow)
                .center_x(Fill)
                .align_y(Bottom)
                .height(Fill)
                .padding(Padding {
                    bottom: 16. - 28. * (1. - opacity) + (28. - size) / 2.,
                    ..Padding::ZERO
                })
                .into()
        },
    )
    .boxed()
}
