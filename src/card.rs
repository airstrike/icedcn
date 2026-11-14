//! Cards with header, content, and footer sections.
//!
//! Container component with header / content / footer slots and three
//! header-internal slots: `title`, `description`, and `action`. The header
//! is a builder — title is required (and auto-styled from a string),
//! description and action are independently optional via fluent methods.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::card;
//! use icedxdy::widget::{button, column};
//!
//! card(column![
//!     card::header("Login to your account")
//!         .description("Enter your email below to login to your account")
//!         .action(card::action(button("Sign Up").style(button::Link).on_press(Message::SignUp))),
//!     card::content(my_form),
//!     card::footer_row([
//!         button("Login").on_press(Message::Submit).into(),
//!         button("Login with Google").style(button::Outline).into(),
//!     ]),
//! ])
//! .width(384.0)
//! ```

use iced_core::Length;
use iced_core::text::IntoFragment;
use iced_widget::Column;
use iced_widget::Row;
use iced_widget::container::Container;
use iced_widget::text::LineHeight;
use iced_widget::{Renderer, Widget};

use crate::Gap;
use crate::Padding;
use crate::Theme;
use crate::core::{Border, Radius};
use crate::radius;
use crate::theme;
use crate::theme::container;
use crate::theme::container::Style;
use crate::widget::rule;
use crate::widget::text;
use crate::widget::{self, Compose, Inherited, Slot};

/// The boxed element a card slot holds.
type Element<'a, Message> = iced_core::Element<'a, Message, Theme, Renderer>;

/// Card root — wraps content with shadcn's card surface (bg-card + ring +
/// rounded corners).
pub fn card<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Element<'a, Message>, Theme> {
    Container::new(content.into())
        .clip(true)
        .style(|t| container::card(t).into())
}

/// Medium-weight card title. A string passed to [`header`] receives this style automatically.
pub fn title<'a>(content: impl IntoFragment<'a>) -> text::Text<'a> {
    text::Text::new(content)
        .size(text::size::BASE)
        .medium()
        .line_height(LineHeight::Relative(1.375))
        .style(theme::text::Foreground)
}

/// Creates muted description text for a card with a 20-pixel line height.
pub fn description<'a>(content: impl IntoFragment<'a>) -> text::Text<'a> {
    text::muted(content).line_height(iced_core::Pixels(20.0))
}

/// Marks content as a card action. Pass it to [`Header::action`] to place it at the top right
/// of the header.
pub fn action<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Slot<'a, Message> {
    content.into()
}

/// Builder for a card header.
///
/// Start with a title and optionally add a description or action. The header stacks title and
/// description and aligns the action to their right.
pub struct Header<'a, Message> {
    title: Slot<'a, Message>,
    inset: f32,
    description: Option<Slot<'a, Message>>,
    action: Option<Slot<'a, Message>>,
}

/// Build a header from a title.
///
/// Plain strings auto-style as [`title`]. Pre-styled `Text` or `Slot`
/// values pass through unchanged — useful when the title needs custom
/// styling (rich text, icon, etc.).
///
/// Add a description and/or an action via the builder methods:
/// ```ignore
/// card::header("Login to your account")
///     .description("Enter your email below to login to your account")
///     .action(card::action(button("Sign Up").style(button::Link).on_press(Message::SignUp)));
/// ```
pub fn header<'a, Message: 'a>(title: impl IntoTitle<'a, Message>) -> Header<'a, Message> {
    Header {
        title: title.into_title(),
        inset: 16.0,
        description: None,
        action: None,
    }
}

impl<'a, Message: 'a> Header<'a, Message> {
    /// Horizontal section padding, shared with the card content and footer.
    pub fn inset(mut self, pixels: f32) -> Self {
        self.inset = pixels.max(0.0);
        self
    }

    /// Set the description slot. Plain strings auto-style as [`description`]
    /// (`text-sm text-muted-foreground`); pre-styled values pass through.
    pub fn description(mut self, description: impl IntoDescription<'a, Message>) -> Self {
        self.description = Some(description.into_description());
        self
    }

    /// Places a link, button, badge, or other action at the top right of the header.
    pub fn action(mut self, action: impl Into<Slot<'a, Message>>) -> Self {
        self.action = Some(action.into());
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Header<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.title.inherit(from);
        if let Some(description) = &mut self.description {
            description.inherit(from);
        }
        if let Some(action) = &mut self.action {
            action.inherit(from);
        }
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message>
    where
        Self: 'b,
    {
        let Header {
            title,
            inset,
            description,
            action,
        } = *self;

        // Stack title above (optional) description in a column with gap-1.
        let mut text_col: Column<Element<'a, Message>> =
            Column::new().spacing(Gap::G1).width(Length::Fill);
        text_col = text_col.push(title.compose());
        if let Some(d) = description {
            text_col = text_col.push(d.compose());
        }

        // The flexible title column pushes the action to the right edge.
        let padded_inner: Element<'a, Message> = match action {
            Some(a) => Row::new()
                .push(text_col.boxed())
                .push(a.compose())
                .align_y(iced_core::Alignment::Start)
                .width(Length::Fill)
                .boxed(),
            None => text_col.boxed(),
        };

        iced_widget::container(padded_inner)
            .padding([0.0, inset])
            .width(Length::Fill)
            .boxed()
    }
}

impl<'a, Message: 'a> From<Header<'a, Message>> for Element<'a, Message> {
    fn from(header: Header<'a, Message>) -> Self {
        Box::new(header).compose()
    }
}

impl<'a, Message: 'a> From<Header<'a, Message>> for Slot<'a, Message> {
    fn from(h: Header<'a, Message>) -> Self {
        Slot::new(h)
    }
}

/// Convert a value into a card title. Plain strings auto-style as [`title`];
/// pre-styled `Text` and `Slot` values pass through. Mirrors
/// `field::IntoLabel` — name lives under the `card` module so it's read as
/// `card::IntoTitle` (no composite name).
pub trait IntoTitle<'a, Message> {
    fn into_title(self) -> Slot<'a, Message>;
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for &'a str {
    fn into_title(self) -> Slot<'a, Message> {
        title(self).into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for String {
    fn into_title(self) -> Slot<'a, Message> {
        title(self).into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for text::Text<'a> {
    fn into_title(self) -> Slot<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for Slot<'a, Message> {
    fn into_title(self) -> Slot<'a, Message> {
        self
    }
}

/// Convert a value into a card description. Plain strings auto-style as
/// [`description`]; pre-styled values pass through.
pub trait IntoDescription<'a, Message> {
    fn into_description(self) -> Slot<'a, Message>;
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for &'a str {
    fn into_description(self) -> Slot<'a, Message> {
        description(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for String {
    fn into_description(self) -> Slot<'a, Message> {
        description(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for text::Text<'a> {
    fn into_description(self) -> Slot<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for Slot<'a, Message> {
    fn into_description(self) -> Slot<'a, Message> {
        self
    }
}

/// Card header with title only (no description, no action). Convenience for
/// the most common shape.
///
/// Equivalent to `card::header(title)` — both flow through the same
/// rendering path. Kept as a named export so call sites that don't need
/// the builder shape stay terse.
pub fn header_simple<'a, Message: 'a>(title: impl IntoTitle<'a, Message>) -> Header<'a, Message> {
    header(title)
}

/// Card content section (`px-4` only).
pub fn content<'a, Message: 'a>(
    child: impl Into<Element<'a, Message>>,
) -> Container<'a, Element<'a, Message>, Theme> {
    Container::new(child.into())
        .padding(Padding::X4)
        .width(Length::Fill)
}

/// Card footer section. Caller composes the inner shape (row of buttons,
/// stacked column, etc.); `footer` applies nova's muted footer surface.
pub fn footer<'a, Message: 'a>(child: impl Into<Slot<'a, Message>>) -> Slot<'a, Message> {
    footer_with_spacing(child, 16.0)
}

/// A footer with a custom card spacing value.
pub fn footer_with_spacing<'a, Message: 'a>(
    child: impl Into<Slot<'a, Message>>,
    spacing: f32,
) -> Slot<'a, Message> {
    let line: Element<'a, Message> = rule::horizontal(1).style(footer_rule_style()).boxed();

    let body = widget::container(child)
        .padding(spacing.max(0.0))
        .width(Length::Fill)
        .style(footer_style);

    widget::column([line.into(), body.into()])
        .spacing(0)
        .width(Length::Fill)
        .into()
}

/// Card footer as a row of children with 8px (`gap-2`) spacing.
pub fn footer_row<'a, Message: 'a>(
    children: impl IntoIterator<Item = Element<'a, Message>>,
) -> Slot<'a, Message> {
    let row = widget::Row::with_children(children.into_iter().map(Slot::from)).spacing(Gap::G2);
    footer(row)
}

fn footer_style(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.alpha(0.50).into()),
        border: Border {
            radius: Radius {
                bottom_right: radius::XL,
                bottom_left: radius::XL,
                ..Radius::default()
            },
            ..Border::default()
        },
        ..Style::default()
    }
}

fn footer_rule_style() -> rule::StyleFn<'static, Theme> {
    Box::new(|theme| rule::Style {
        background: theme.neutral.border.into(),
        ..rule::Style::default()
    })
}
