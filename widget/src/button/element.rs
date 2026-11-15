use std::rc::Rc;

use iced_core::Length;
use iced_core::Length::{Fill, Fixed};
use iced_core::Padding;
use iced_core::alignment;
use iced_core::border;
use iced_core::mouse;
use iced_widget::Widget;

use crate::Size;
use crate::compose::{Compose, Inherited, Slot};
use icedcn_core::Theme;
use icedcn_core::theme;
use icedcn_core::theme::button::state::{self, IntoStyle};

use super::widget;

fn default(theme: &Theme, status: widget::Status) -> widget::Style {
    // theme::button::default returns our OKLCH-native Style; convert
    // through iced::button::Style → widget::Style.
    let iced: iced_widget::button::Style = theme::button::default(theme, status.into()).into();
    iced.into()
}

impl widget::Catalog for Theme {
    type Class<'a> = widget::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: widget::Status) -> widget::Style {
        class(self, status)
    }
}

/// A button that wraps our [`Element`] system.
///
/// Holds a [`Size`] which is propagated to its content when converting
/// to an `iced::Element`. This means a `Size::Lg` button automatically
/// gets larger text, larger icons, etc.
pub struct Button<'a, Message, Theme = icedcn_core::Theme, Renderer = iced_widget::Renderer>
where
    Renderer: iced_core::Renderer,
    Theme: iced_widget::button::Catalog,
{
    join: Option<super::group::Join>,
    content: Vec<Slot<'a, Message, Theme, Renderer>>,
    size: Option<Size>,
    weight: Option<crate::Weight>,
    height: Option<Length>,
    padding: Option<Padding>,
    square: bool,
    rounded: bool,
    translate: bool,
    animate_hover: bool,
    inset: Padding,
    interaction: Option<iced_core::mouse::Interaction>,
    width: Option<Length>,
    style: state::StyleFn<'a, Theme>,
    nova: bool,
    invalid: bool,
    expanded: bool,
    anchor: bool,
    icon_start: bool,
    icon_end: bool,
    focus_ring: Option<Ring>,
    on_press: Option<Message>,
    disabled: bool,
}

/// Create a button with default size and style
pub fn button<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Button<'a, Message> {
    Button {
        join: None,
        content: vec![content.into()],
        size: None,
        weight: None,
        height: None,
        padding: None,
        square: false,
        rounded: false,
        translate: true,
        animate_hover: true,
        inset: Padding::ZERO,
        interaction: None,
        width: None,
        style: theme::button::Default.into_style(),
        nova: true,
        invalid: false,
        expanded: false,
        anchor: false,
        icon_start: false,
        icon_end: false,
        focus_ring: None,
        on_press: None,
        disabled: false,
    }
}

impl<'a, Message: Clone + 'a> Button<'a, Message> {
    /// Append inline content. The size variant supplies the gap between children.
    pub fn push(mut self, content: impl Into<Slot<'a, Message>>) -> Self {
        self.content.push(content.into());
        self
    }

    /// Use reduced leading padding for an icon at the start of the button.
    pub fn icon_start(mut self) -> Self {
        self.icon_start = true;
        self
    }

    /// Use reduced trailing padding for an icon at the end of the button.
    pub fn icon_end(mut self) -> Self {
        self.icon_end = true;
        self
    }

    /// Apply the invalid border and ring. This does not disable the action.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Apply the expanded appearance of a popup trigger.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Mark a popup trigger, which does not translate while pressed.
    pub fn has_popup(mut self, has_popup: bool) -> Self {
        self.translate = !has_popup;
        self
    }

    /// Style an anchor action (including primary hover) with a pointer cursor.
    /// The caller supplies its destination message with `on_press`.
    pub fn anchor(mut self) -> Self {
        self.anchor = true;
        self.interaction = Some(mouse::Interaction::Pointer);
        self
    }

    /// Set the size variant
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Override the control height while keeping the variant's text size.
    /// Use `Shrink` to fit the content and padding. Square buttons retain
    /// the variant's side length unless given a fixed height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Override the space around non-square button content.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Make the button square (equal width and height).
    pub fn square(mut self) -> Self {
        self.square = true;
        self
    }

    /// Make the button fully rounded (pill/circle shape).
    pub fn rounded(mut self) -> Self {
        self.rounded = true;
        self
    }

    /// Whether hover colors interpolate over 150 ms. Defaults to `true`.
    pub fn animate_hover(mut self, animate: bool) -> Self {
        self.animate_hover = animate;
        self
    }

    /// Whether to translate down 1px on press. Defaults to `true`.
    ///
    /// Set to `false` for sidebar menu items or other elements styled
    /// as buttons that shouldn't have a press-down effect.
    pub fn translate(mut self, translate: bool) -> Self {
        self.translate = translate;
        self
    }

    /// Set the mouse interaction (cursor) when hovering.
    pub fn interaction(mut self, interaction: iced_core::mouse::Interaction) -> Self {
        self.interaction = Some(interaction);
        self
    }

    /// Set a transparent inset between the button's bounds (hit-test +
    /// hover region) and the painted bg/border. Used by sidebar items so
    /// adjacent buttons in a `spacing(0)` column hover continuously while
    /// the visible fills sit apart by `inset.top + inset.bottom`.
    pub fn inset(mut self, inset: impl Into<Padding>) -> Self {
        self.inset = inset.into();
        self
    }

    /// Set the layout width. Defaults to the vendored widget's intrinsic
    /// shrink-to-content sizing; pass `iced::Fill` to stretch (used by
    /// sidebar nav items so the hover/active surface spans the row).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Set the font weight
    pub fn weight(mut self, weight: impl Into<crate::Weight>) -> Self {
        self.weight = Some(weight.into());
        self
    }

    /// Set the style
    pub fn style<S: IntoStyle<'a>>(mut self, style: S) -> Self {
        self.nova = style.uses_border_box();
        self.style = style.into_style();
        self
    }

    /// Paint an exterior ring when the button receives keyboard focus.
    pub fn focus_ring(
        mut self,
        width: impl Into<iced_core::Pixels>,
        color: fn(&Theme) -> icedcn_core::Color,
    ) -> Self {
        self.focus_ring = Some((width.into().0, color));
        self
    }

    /// The fixed height override, or the size variant's nominal height.
    pub fn size_height(&self) -> f32 {
        match self.height {
            Some(Fixed(height)) => height,
            _ => self.size.unwrap_or_default().height(),
        }
    }

    /// Adjust the resolved [`iced_widget::button::Style`] after applying the user’s style. A
    /// button group uses this to join the corners of adjacent buttons.
    pub fn map_style<F>(mut self, transform: F) -> Self
    where
        F: Fn(iced_widget::button::Style) -> iced_widget::button::Style + 'a,
    {
        let prev = std::mem::replace(&mut self.style, Box::new(|_, _| state::Style::default()));
        self.style = Box::new(move |t, s| {
            let mut resolved = prev(t, s);
            resolved.paint = transform(resolved.paint);
            resolved
        });
        self
    }

    /// Set the message published when the button is pressed. A button
    /// without one stays enabled and publishes nothing; use
    /// [`disabled`](Self::disabled) to disable it.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Set the message published when the button is pressed, if `Some`.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Disable the button: it ignores input, is skipped by focus traversal
    /// and paints with the variant's disabled style (nova `disabled:opacity-50`).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>>
    for iced_core::Element<'a, Message, Theme, iced_widget::Renderer>
{
    fn from(mut btn: Button<'a, Message>) -> Self {
        let size = btn.size.unwrap_or_default();
        let h = btn.size_height();
        let line_height = match size {
            Size::Xs => 16.0,
            Size::Sm => 22.4,
            _ => 20.0,
        };
        let mut inherited = Inherited::default()
            .with_size(size)
            .with_weight(btn.weight.unwrap_or(crate::Weight::Medium))
            .with_selectable(false)
            .with_disabled(btn.disabled);
        if btn.nova {
            inherited = inherited.with_line_height(line_height);
        }
        for child in &mut btn.content {
            child.inherit(&inherited);
        }
        let content = if btn.content.len() == 1 {
            btn.content.pop().expect("Button content").compose()
        } else {
            iced_widget::Row::with_children(btn.content.into_iter().map(Slot::compose))
                .spacing(size.gap())
                .align_y(alignment::Vertical::Center)
                .boxed()
        };
        let mut padding = btn.padding.unwrap_or_else(|| size.padding());
        if btn.nova {
            // CSS height includes the border. Vertical layout centers the line box;
            // horizontal padding reserves the transparent 1px border on each edge.
            if btn.padding.is_none() {
                padding.top = 1.0;
                padding.bottom = 1.0;
            }
            let edge = if matches!(size, Size::Xs | Size::Sm) {
                6.0
            } else {
                8.0
            };
            if btn.icon_start {
                padding.left = edge;
            }
            if btn.icon_end {
                padding.right = edge;
            }
            padding.left += 1.0;
            padding.right += 1.0;
        }
        if let Some(join) = btn.join {
            padding = join.padding(padding, 1.);
        }
        let mut iced_btn = if btn.square {
            super::widget::Button::new(iced_widget::center(content))
                .width(h)
                .height(h)
                .padding(0)
        } else {
            // Stock callbacks retain their original explicit Fill behavior.
            let inner = if !btn.nova && matches!(btn.width, Some(Fill)) {
                iced_widget::container(content)
                    .width(Fill)
                    .align_x(alignment::Horizontal::Center)
                    .boxed()
            } else {
                content
            };
            widget::Button::new(inner)
                .height(btn.height.unwrap_or_else(|| h.into()))
                .padding(padding)
        };

        iced_btn = iced_btn
            .translate(btn.translate)
            .animate_hover(btn.animate_hover)
            .inset(btn.inset)
            .border_box(btn.nova)
            .join(btn.join);
        if let Some(interaction) = btn.interaction {
            iced_btn = iced_btn.interaction(interaction);
        }
        // `square` already set both width and height to the size's height;
        // skip the user width override in that case so the square invariant
        // holds.
        if let Some(width) = btn.width
            && !btn.square
        {
            iced_btn = iced_btn.width(width);
        }

        let rounded = btn.rounded;
        let status = move |s| state::Status {
            hovered: matches!(
                s,
                widget::Status::Hovered
                    | widget::Status::Pressed
                    | widget::Status::Focused { is_hovered: true }
            ),
            pressed: s == widget::Status::Pressed,
            focused: matches!(s, widget::Status::Focused { .. }),
            disabled: s == widget::Status::Disabled,
            invalid: btn.invalid,
            expanded: btn.expanded,
            anchor: btn.anchor,
            small: matches!(size, Size::Xs | Size::Sm),
        };
        let resolve: Resolver<'a> = Rc::from(btn.style);
        let decorate = resolve.clone();
        let bridge: widget::StyleFn<'a, Theme> = Box::new(move |t, s| {
            let resolved = resolve(t, status(s));
            let mut style: widget::Style = resolved.paint.into();
            style.ring = resolved.ring;
            if rounded {
                style.border.radius = (u32::MAX as f32).into();
            }
            if let Some((width, color)) = btn.focus_ring
                && matches!(s, widget::Status::Focused { .. })
            {
                style.ring = Some(border::width(width).color(color(t)));
            }
            if let Some(ring) = &mut style.ring {
                let extent = ring.width + resolved.ring_offset;
                let radius = style.border.radius;
                ring.radius = border::Radius {
                    top_left: radius.top_left + extent,
                    top_right: radius.top_right + extent,
                    bottom_left: radius.bottom_left + extent,
                    bottom_right: radius.bottom_right + extent,
                };
            }
            style
        });
        iced_btn = iced_btn.style(bridge).decoration(move |t, s| {
            let resolved = decorate(t, status(s));
            (resolved.underline, resolved.ring_offset)
        });

        if let Some(on_press) = btn.on_press {
            iced_btn = iced_btn.on_press(on_press);
        }

        iced_btn.disabled(btn.disabled).into()
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Button<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Button<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.join = self.join.or(from.join());
        self.weight = self.weight.or(from.weight());
        self.size = self.size.or(from.size());
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, iced_widget::Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_widget::button;

    fn stock(_: &Theme, _: button::Status) -> button::Style {
        button::Style::default()
    }

    #[test]
    fn shrink_height_fits_content_and_padding_without_changing_fixed_defaults() {
        use iced_core::Length::Shrink;
        use iced_core::renderer::Headless;
        use iced_core::widget::Tree;
        use iced_core::{Element, Pixels, Size, Widget as _, layout};
        use iced_test::futures::futures::executor::block_on;
        use iced_widget::{Renderer, Space};

        let renderer =
            block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
        let content = || {
            crate::column![
                Space::new().width(80).height(13),
                Space::new().width(60).height(12),
            ]
            .spacing(5)
        };
        let cases = [
            (super::button(content()).height(Shrink).padding(10), 50.0),
            (super::button(content()), 32.0),
            (super::button(content()).height(48), 48.0),
            (super::button(content()).height(Pixels(46.0)), 46.0),
            (super::button(content()).height(40).square(), 40.0),
            (super::button(content()).height(Shrink).square(), 32.0),
        ];
        for (button, expected) in cases {
            let square = button.square;
            let mut element: Element<'_, (), Theme, Renderer> = button.into();
            let mut tree = Tree::new(&element);
            tree.diff(&mut element);
            element.layout(
                &mut tree,
                &renderer,
                &layout::Limits::new(Size::ZERO, Size::new(400.0, 400.0)),
            );
            assert_eq!(tree.size.height, expected);
            if square {
                assert_eq!(tree.size.width, expected);
            }
        }
    }

    #[test]
    fn existing_style_forms_compile_and_stock_fields_stay_exact() {
        let callback: button::StyleFn<'_, Theme> = Box::new(stock);
        let buttons: [Button<'_, ()>; 6] = [
            super::button("stock").style(callback),
            super::button("function").style(stock),
            super::button("closure").style(|_: &Theme, _: button::Status| button::Style::default()),
            super::button("boxed").style(Box::new(|_: &Theme, _: button::Status| {
                button::Style::default()
            })),
            super::button("variant").style(theme::button::Outline),
            super::button("spec").style(theme::button::Outline + icedcn_core::radius(4)),
        ];
        for b in &buttons[..4] {
            assert!(!b.nova);
            let resolved = (b.style)(
                &Theme::light(),
                state::Status {
                    focused: true,
                    hovered: true,
                    ..state::Status::default()
                },
            );
            assert_eq!(resolved.paint, button::Style::default());
            assert_eq!(resolved.ring, None);
        }
    }

    #[test]
    fn group_mapping_keeps_underlines_rings_and_mapped_corners() {
        let b: Button<'_, ()> =
            super::button("Link")
                .style(theme::button::Link)
                .map_style(|mut style| {
                    style.border.radius.top_left = 0.0;
                    style
                });
        let status = state::Status {
            hovered: true,
            focused: true,
            ..state::Status::default()
        };
        let resolved = (b.style)(&Theme::light(), status);
        assert!(resolved.underline);
        assert_eq!(resolved.ring.expect("focus ring").width, 3.0);
        assert_eq!(resolved.paint.border.radius.top_left, 0.0);
        assert_eq!(resolved.paint.border.radius.top_right, 10.0);
    }
}

type Ring = (f32, fn(&Theme) -> icedcn_core::Color);

type Resolver<'a> = Rc<dyn Fn(&Theme, state::Status) -> state::Style + 'a>;
