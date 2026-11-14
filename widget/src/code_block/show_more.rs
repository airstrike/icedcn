use std::f32::consts::PI;

use iced_core::animation::Easing;
use iced_core::time::Duration;
use iced_core::{Animation, Length::Fill, Never, Pixels, never};
use iced_widget::{Widget, transition};

use super::{CodeBlock, glyph::Glyph};
use crate::animation::cubic_bezier;
use crate::collapsible::Collapsible;
use crate::{Size, Slot, button, collapsible, column, container, row};

/// Show a clipped preview with a footer that expands the complete code block.
/// The block's own height or maximum height applies when expanded. Use the
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
            block.height = None;
            block = block.max_height(height).collapsed(true);
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
        let toggle = handle.trigger(Slot::from(toggle).map(never));
        let toggle = container(toggle).center_x(Fill);
        if let Some(footer) = block.footer.take() {
            block.footer(column![footer, toggle].spacing(8)).into()
        } else {
            block.footer(toggle).into()
        }
    })
    .width(Fill)
}
