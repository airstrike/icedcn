use iced_widget::Widget;
mod element;
pub mod widget;

use crate::compose::{Compose, Inherited, View};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::widget::Id;
use iced_core::{Length, Padding, Pixels};
use iced_widget::Renderer;
use iced_widget::container::StyleFn;
use icedcn_core::Theme;
use icedcn_core::theme::container::IntoStyle;

/// Create a [`Container`] wrapping the given content.
///
/// ```ignore
/// container(text::h1("Hello")).padding(16).center_x(Fill)
/// ```
pub struct Container<'a, Message> {
    content: View<'a, Message>,
    id: Option<Id>,
    width: Length,
    height: Length,
    padding: Padding,
    align_x: Horizontal,
    align_y: Vertical,
    clip: bool,
    fit_content: bool,
    style: Option<StyleFn<'static, Theme>>,
}

impl<'a, Message: 'a> Container<'a, Message> {
    pub fn new(content: impl Into<View<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            id: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::ZERO,
            align_x: Horizontal::Left,
            align_y: Vertical::Top,
            clip: false,
            fit_content: false,
            style: None,
        }
    }

    /// Set this container's widget [`Id`] — required for
    /// `scroll_into_view` and other Id-based operations.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Caps the [`Container`] width. Folded into `width` as a
    /// [`Length::Bounded`] variant — when chained after `.width(Fill)` or
    /// `.width(Shrink)`, the cap propagates through `Limits` cleanly and
    /// overrides any inherited cross-axis compression from a `Shrink`
    /// ancestor (the iced flex pass-2 collapse-to-zero pathology).
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.width = self.width.max(max_width);
        self
    }

    /// Caps the [`Container`] height. See [`max_width`] for the folding semantics.
    ///
    /// [`max_width`]: Self::max_width
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.height = self.height.max(max_height);
        self
    }

    /// Sets a minimum width on the [`Container`]. Folded into `width` as a
    /// [`Length::Bounded`] variant.
    pub fn min_width(mut self, min_width: impl Into<Pixels>) -> Self {
        self.width = self.width.min(min_width);
        self
    }

    /// Sets a minimum height on the [`Container`]. Folded into `height` as a
    /// [`Length::Bounded`] variant.
    pub fn min_height(mut self, min_height: impl Into<Pixels>) -> Self {
        self.height = self.height.min(min_height);
        self
    }

    pub fn padding(mut self, padding: impl Into<icedcn_core::Padding>) -> Self {
        let padding: icedcn_core::Padding = padding.into();
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Container`] and centers its contents horizontally.
    pub fn center_x(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(Horizontal::Center)
    }

    /// Sets the height of the [`Container`] and centers its contents vertically.
    pub fn center_y(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(Vertical::Center)
    }

    /// Sets the content alignment for the horizontal axis.
    pub fn align_x(mut self, alignment: impl Into<Horizontal>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Sets the content alignment for the vertical axis.
    pub fn align_y(mut self, alignment: impl Into<Vertical>) -> Self {
        self.align_y = alignment.into();
        self
    }

    /// Sets the style of this container: a variant (`container::Card`), a
    /// tweaked variant or `Spec`, or a closure returning a container `Style`.
    pub fn style(mut self, style: impl IntoStyle<'static>) -> Self {
        self.style = Some(style.into_style());
        self
    }

    /// Sets whether the contents of this container should be clipped on
    /// overflow. Mirrors iced's stock `Container::clip`.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Resolve the intrinsic width before wrapping a bubble's contents.
    pub(crate) fn fit_content(mut self) -> Self {
        self.fit_content = true;
        self
    }

    /// Center this container horizontally and vertically inside the
    /// available space. Shortcut for `.center_x(Fill).center_y(Fill)`.
    pub fn center(self, length: impl Into<Length>) -> Self {
        let l = length.into();
        self.center_x(l).center_y(l)
    }
}

impl<'a, Message: 'a> From<Container<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(c: Container<'a, Message>) -> Self {
        Box::new(c).compose()
    }
}

impl<'a, Message: 'a> From<Container<'a, Message>> for View<'a, Message> {
    fn from(widget: Container<'a, Message>) -> Self {
        View::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Container<'a, Message> {
    fn has_card_footer(&self) -> bool {
        self.content.has_card_footer()
    }

    fn inherit(&mut self, from: &Inherited) {
        self.content.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let mut c = widget::Container::new(self.content.compose())
            .width(self.width)
            .height(self.height)
            .padding(self.padding)
            .align_x(self.align_x)
            .align_y(self.align_y)
            .fit_content(self.fit_content)
            .clip(self.clip);

        if let Some(id) = self.id {
            c = c.id(id);
        }
        if let Some(style) = self.style {
            // Bridge: wrapper's `style` is `iced_widget::container::StyleFn`
            // (so existing variants like `theme::container::CardPreview`
            // flow naturally via their existing `From<X> for iced::StyleFn`
            // impls). At construction we wrap that closure to land in the
            // vendored widget's `Style` via the field-identical `From`.
            let bridge: widget::StyleFn<'static, Theme> =
                Box::new(move |t: &Theme| widget::Style::from((style)(t)));
            c = c.class(bridge);
        }

        c.boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{column, container, row};
    use iced_core::Length::Fill;
    use iced_core::widget::Operation;
    use iced_core::{Element, Rectangle};
    use iced_test::simulator::simulator;
    use iced_widget::Space;

    #[test]
    fn intrinsic_surface_resolves_nested_fill_at_its_finite_width() {
        #[derive(Default)]
        struct Bounds(Vec<(Id, Rectangle)>);
        impl Operation for Bounds {
            fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
                visit(self);
            }
            fn container(&mut self, id: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
                if let Some(id) = id {
                    self.0.push((id.clone(), bounds));
                }
            }
        }
        for (maximum, expected) in [(256.0, 108.0), (80.0, 80.0)] {
            let content: Element<'_, (), Theme, Renderer> = container(
                column![
                    row![
                        container(Space::new().width(40).height(12))
                            .width(Fill)
                            .id("name"),
                        container(Space::new().width(20).height(12)).id("value"),
                    ]
                    .spacing(8)
                    .width(Fill)
                ]
                .width(Fill.min(108)),
            )
            .fit_content()
            .max_width(maximum)
            .id("surface")
            .into();
            let mut ui = simulator(content);
            let mut bounds = Bounds::default();
            ui.operate(&mut bounds);
            let find = |name| {
                bounds
                    .0
                    .iter()
                    .find(|(id, _)| *id == Id::new(name))
                    .unwrap()
                    .1
            };
            let surface = find("surface");
            let name = find("name");
            let value = find("value");
            assert_eq!(surface.width, expected);
            assert_eq!(value.x + value.width, surface.x + surface.width);
            assert_eq!(name.x + name.width + 8.0, value.x);
        }
    }
}
