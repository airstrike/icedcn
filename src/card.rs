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
//! use icedcn::card;
//! use icedcn::widget::{button, column};
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

use iced_core::Length::Fill;
use iced_core::text::IntoFragment;
use iced_core::{Length, Padding};
use iced_widget::Column;
use iced_widget::Row;
use iced_widget::container::StyleFn;
use iced_widget::{Renderer, Widget};

use crate::Gap;
use crate::Size;
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

/// A Card root that retains its children through the Compose cascade.
pub struct Card<'a, Message> {
    content: Slot<'a, Message>,
    size: Size,
    spacing: Option<f32>,
    gap: Option<f32>,
    inherited: Inherited,
    width: Length,
    height: Length,
    padding: Option<Padding>,
    clip: bool,
    style: StyleFn<'a, Theme>,
}

/// Card root with Nova's surface, section spacing and vertical padding.
pub fn card<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Card<'a, Message> {
    Card {
        content: content.into(),
        size: Size::Default,
        spacing: None,
        gap: None,
        inherited: Inherited::default(),
        width: Length::Shrink,
        height: Length::Shrink,
        padding: None,
        clip: true,
        style: Box::new(|t| container::card(t).into()),
    }
}

impl<'a, Message: 'a> Card<'a, Message> {
    /// Small cards use 12px spacing and 14px titles; other sizes use 16px.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the section gap, root vertical padding and section insets together.
    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        self.spacing = Some(spacing.into().0.max(0.0));
        self
    }

    /// Override only the section gap, keeping the root's section insets.
    pub fn gap(mut self, gap: impl Into<iced_core::Pixels>) -> Self {
        self.gap = Some(gap.into().0.max(0.0));
        self
    }

    /// Set the root width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the root height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Override root padding, for example zero for an edge-to-edge image.
    pub fn padding(mut self, padding: impl Into<crate::Padding>) -> Self {
        self.padding = Some(padding.into().into());
        self
    }

    /// Set overflow clipping (enabled by default).
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Override the stock-compatible container surface.
    pub fn style(mut self, style: impl Fn(&Theme) -> iced_widget::container::Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Card<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message>
    where
        Self: 'b,
    {
        let Card {
            mut content,
            size,
            spacing,
            gap,
            inherited,
            width,
            height,
            padding,
            clip,
            style,
        } = *self;
        let small = size == Size::Sm;
        let spacing = spacing.unwrap_or(if small { 12.0 } else { 16.0 });
        let padding = padding.unwrap_or(Padding {
            top: spacing,
            bottom: if content.has_card_footer() {
                0.0
            } else {
                spacing
            },
            ..Padding::ZERO
        });
        let inherited = inherited
            .with_card(spacing, if small { 14.0 } else { 16.0 })
            .with_card_gap(gap.unwrap_or(spacing));
        // The Card's text-sm is local; enclosing control sizes do not resize it.
        content.inherit(&inherited.with_size(Size::Default));
        iced_widget::container(content.compose())
            .width(width)
            .height(height)
            .padding(padding)
            .clip(clip)
            .class(style)
            .boxed()
    }
}

impl<'a, Message: 'a> From<Card<'a, Message>> for Slot<'a, Message> {
    fn from(card: Card<'a, Message>) -> Self {
        Slot::new(card)
    }
}

impl<'a, Message: 'a> From<Card<'a, Message>> for Element<'a, Message> {
    fn from(card: Card<'a, Message>) -> Self {
        Box::new(card).compose()
    }
}

/// Medium-weight card title. A string passed to [`header`] receives this style automatically.
pub fn title<'a>(content: impl IntoFragment<'a>) -> text::Text<'a> {
    text::Text::new(content)
        .card_title()
        .medium()
        .line_height(text::leading::SNUG)
        .style(theme::text::Foreground)
}

/// Creates muted `text-sm` description text for a card.
pub fn description<'a>(content: impl IntoFragment<'a>) -> text::Text<'a> {
    text::muted(content)
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
    inset: Option<f32>,
    inherited: Inherited,
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
        inset: None,
        inherited: Inherited::default(),
        description: None,
        action: None,
    }
}

impl<'a, Message: 'a> Header<'a, Message> {
    /// Horizontal section padding, shared with the card content and footer.
    pub fn inset(mut self, pixels: f32) -> Self {
        self.inset = Some(pixels.max(0.0));
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
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message>
    where
        Self: 'b,
    {
        let Header {
            mut title,
            inset,
            inherited,
            mut description,
            mut action,
        } = *self;
        let inset = inset.or(inherited.card_spacing()).unwrap_or(16.0);
        let inherited = inherited.without_card_gap();
        title.inherit(&inherited);
        if let Some(description) = &mut description {
            description.inherit(&inherited);
        }
        if let Some(action) = &mut action {
            action.inherit(&inherited);
        }

        // Stack title above (optional) description in a column with gap-1.
        let mut text_col: Column<Element<'a, Message>> = Column::new().spacing(Gap::G1).width(Fill);
        text_col = text_col.push(title.compose());
        if let Some(d) = description {
            text_col = text_col.push(d.compose());
        }

        // The flexible title column pushes the action to the right edge.
        let padded_inner: Element<'a, Message> = match action {
            Some(a) => Row::new()
                .push(text_col.boxed())
                .push(a.compose())
                .spacing(Gap::G1)
                .align_y(iced_core::Alignment::Start)
                .width(Fill)
                .boxed(),
            None => text_col.boxed(),
        };

        iced_widget::container(padded_inner)
            .padding([0.0, inset])
            .width(Fill)
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

/// Card body with a lazy child and a horizontal inset from its root.
pub struct Content<'a, Message> {
    child: Slot<'a, Message>,
    inherited: Inherited,
    padding: Option<Padding>,
    width: Length,
    height: Length,
}

/// Card content section (`px-(--card-spacing)`).
pub fn content<'a, Message: 'a>(child: impl Into<Slot<'a, Message>>) -> Content<'a, Message> {
    Content {
        child: child.into(),
        inherited: Inherited::default(),
        padding: None,
        width: Length::Fill,
        height: Length::Shrink,
    }
}

impl<'a, Message: 'a> Content<'a, Message> {
    /// Override this section's padding without changing other Card parts.
    pub fn padding(mut self, padding: impl Into<crate::Padding>) -> Self {
        self.padding = Some(padding.into().into());
        self
    }

    /// Set the section width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the section height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Content<'a, Message> {
    fn has_card_footer(&self) -> bool {
        self.child.has_card_footer()
    }

    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message>
    where
        Self: 'b,
    {
        let Content {
            mut child,
            inherited,
            padding,
            width,
            height,
        } = *self;
        let spacing = inherited.card_spacing().unwrap_or(16.0);
        child.inherit(&inherited.without_card_gap());
        iced_widget::container(child.compose())
            .padding(padding.unwrap_or([0.0, spacing].into()))
            .width(width)
            .height(height)
            .boxed()
    }
}

impl<'a, Message: 'a> From<Content<'a, Message>> for Slot<'a, Message> {
    fn from(content: Content<'a, Message>) -> Self {
        Slot::new(content)
    }
}

impl<'a, Message: 'a> From<Content<'a, Message>> for Element<'a, Message> {
    fn from(content: Content<'a, Message>) -> Self {
        Box::new(content).compose()
    }
}

/// A Card footer that receives its inset before composing its controls.
pub struct Footer<'a, Message> {
    child: Slot<'a, Message>,
    spacing: Option<f32>,
    inherited: Inherited,
}

/// Card footer section with a top border and Nova's muted surface.
pub fn footer<'a, Message: 'a>(child: impl Into<Slot<'a, Message>>) -> Footer<'a, Message> {
    Footer {
        child: child.into(),
        spacing: None,
        inherited: Inherited::default(),
    }
}

/// A footer with an explicit inset overriding the root's spacing.
pub fn footer_with_spacing<'a, Message: 'a>(
    child: impl Into<Slot<'a, Message>>,
    spacing: f32,
) -> Footer<'a, Message> {
    Footer {
        spacing: Some(spacing.max(0.0)),
        ..footer(child)
    }
}

impl<'a, Message: 'a> Compose<Message> for Footer<'a, Message> {
    fn has_card_footer(&self) -> bool {
        true
    }

    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message>
    where
        Self: 'b,
    {
        let Footer {
            mut child,
            spacing,
            inherited,
        } = *self;
        let spacing = spacing.or(inherited.card_spacing()).unwrap_or(16.0);
        child.inherit(&inherited.without_card_gap());
        let line: Element<'a, Message> = rule::horizontal(1).style(footer_rule_style()).boxed();
        let body = widget::container(child)
            .padding(spacing)
            .width(Fill)
            .style(footer_style);
        widget::column([line.into(), body.into()])
            .spacing(0)
            .width(Fill)
            .into()
    }
}

impl<'a, Message: 'a> From<Footer<'a, Message>> for Slot<'a, Message> {
    fn from(footer: Footer<'a, Message>) -> Self {
        Slot::new(footer)
    }
}

impl<'a, Message: 'a> From<Footer<'a, Message>> for Element<'a, Message> {
    fn from(footer: Footer<'a, Message>) -> Self {
        Box::new(footer).compose()
    }
}

/// Card footer as a row of Slots with 8px (`gap-2`) spacing.
pub fn footer_row<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Footer<'a, Message> {
    footer(widget::Row::with_children(children).spacing(Gap::G2))
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
