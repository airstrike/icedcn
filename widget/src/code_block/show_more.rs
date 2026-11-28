use std::f32::consts::PI;

use iced_core::Length::Fill;
use iced_core::alignment::Horizontal;
use iced_core::animation::Easing;
use iced_core::time::Duration;
use iced_core::{Animation, Never, Pixels, never};
use iced_widget::{Widget, transition};

use super::glyph::Glyph;
use super::{CodeBlock, footer};
use crate::animation::cubic_bezier;
use crate::collapsible::Collapsible;
use crate::{Size, View, button, collapsible, row};

/// Clip the content to `height` with a centered Show more button in the footer.
/// Expanded, the content's own height or maximum height applies. Use the
/// returned [`Collapsible`] to control the open state or listen for changes.
pub fn show_more<'a, Message: 'static>(
    height: impl Into<Pixels>,
    block: impl Fn() -> CodeBlock<'a, Message> + 'a,
) -> Collapsible<'a, Message> {
    let height = height.into().0;
    collapsible(move |handle| {
        let open = handle.is_open();
        let mut block = block();
        if !open {
            block.content.height = None;
            block.content.max_height = Some(Pixels(height));
            block.content.collapsed = true;
        }
        let chevron = transition(
            open,
            move || {
                Animation::new(open)
                    .duration(Duration::from_millis(150))
                    .easing(Easing::Custom(|x| cubic_bezier(0.4, 0., 0.2, 1., x)))
            },
            |angle, now| {
                Glyph::Chevron(angle.interpolate(0., PI, now))
                    .view::<Never>()
                    .compose()
            },
        )
        .boxed();
        let toggle = button(row![chevron, if open { "Show less" } else { "Show more" }].spacing(4))
            .style(button::Ghost)
            .size(Size::Xs);
        let toggle = handle.trigger(View::from(toggle).map(never));
        let footer = match block.footer.take() {
            Some(footer) => footer.push(toggle),
            None => footer(toggle).align_x(Horizontal::Center),
        };
        block.footer(footer).into()
    })
    .width(Fill)
}
