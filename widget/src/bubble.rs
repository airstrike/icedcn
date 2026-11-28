//! Framed conversational content with optional corner reactions.
use crate::text;
use iced_core::Length::{Fill, FillPortion};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Element, Vector};
use iced_widget::{Renderer, Widget, float, stack};
use icedcn_core::{Border, Color, Padding, Radius, Theme, radius};

use crate::button::widget::Button;
use crate::container::widget::Container;
use crate::{Column, Compose, Inherited, Size, View, button, column, container, row, space};

pub use crate::button::widget::Status;
pub use icedcn_core::theme::container::Style;

/// The seven Nova bubble surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}
pub use Variant::{Destructive, Ghost, Muted, Outline, Secondary, Tinted};

/// Draw-time [`crate::Motion::theme`] for a muted bubble on a solid card.
/// The surface fades with alpha. Its text fades toward the card color so
/// overlapping text and fill have the same result as fading a single layer.
pub fn fade_muted_on_card(theme: &Theme, opacity: f32) -> Theme {
    let mut design = theme.design().clone();
    design.muted.base = design.muted.base.scale_alpha(opacity);
    let card: iced_core::Color = design.card.base.into();
    let ink: iced_core::Color = design.neutral.foreground.into();
    let channel = |card: f32, ink: f32| card + (ink - card) * opacity;
    design.neutral.foreground = Color::from_srgb(iced_core::Color::from_rgb(
        channel(card.r, ink.r),
        channel(card.g, ink.g),
        channel(card.b, ink.b),
    ));
    Theme::new(design)
}

/// Conversational content with optional overlapping reactions and surface actions.
pub struct Bubble<'a, Message> {
    content: View<'a, Message>,
    variant: Variant,
    align: Option<Horizontal>,
    padding: Option<Padding>,
    radius: Option<Radius>,
    reactions: Option<(View<'a, Message>, Horizontal, Vertical)>,
    reaction_padding: Padding,
    interactive: bool,
    on_press: Option<Message>,
    disabled: bool,
    paint: Option<Paint<'a>>,
}

/// Frame content in a primary, start-aligned bubble.
pub fn bubble<'a, Message: 'a>(content: impl Into<View<'a, Message>>) -> Bubble<'a, Message> {
    Bubble {
        content: content.into(),
        variant: Variant::Primary,
        align: None,
        padding: None,
        radius: None,
        reactions: None,
        reaction_padding: [2, 6].into(),
        interactive: false,
        on_press: None,
        disabled: false,
        paint: None,
    }
}

/// Adjacent bubbles in one turn use the tighter Nova group spacing.
pub fn group<'a, Message: 'a>(
    items: impl IntoIterator<Item = View<'a, Message>>,
) -> Column<'a, Message> {
    column(items).spacing(8).width(Fill)
}

impl<'a, Message: 'a> Bubble<'a, Message> {
    /// Select a surface variant, retaining its hover and focus treatment.
    pub fn style(mut self, style: Variant) -> Self {
        self.variant = style;
        self
    }
    /// Align the bubble within its row; otherwise inherit the Message alignment.
    pub fn align(mut self, align: Horizontal) -> Self {
        self.align = Some(align);
        self
    }
    /// Override the space inside the bubble's one-pixel border.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }
    /// Override the corner radius of the bubble surface.
    pub fn radius(mut self, radius: impl Into<Radius>) -> Self {
        self.radius = Some(radius.into());
        self
    }
    /// Attach content to a bubble edge, overlapping it by one quarter of its height.
    pub fn reactions(
        mut self,
        content: impl Into<View<'a, Message>>,
        align: Horizontal,
        side: Vertical,
    ) -> Self {
        self.reactions = Some((content.into(), align, side));
        self
    }

    /// Override reaction padding. Use zero when the reaction contains a button.
    pub fn reaction_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.reaction_padding = padding.into();
        self
    }

    /// Make the entire surface focusable and pressable, including without a handler.
    /// This is useful when a toast or other marker observes the activation.
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    /// Make the surface interactive and publish a message when activated.
    pub fn on_press(mut self, message: Message) -> Self {
        self.interactive = true;
        self.on_press = Some(message);
        self
    }

    /// Suppress pointer/keyboard interaction and disable descendants.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled |= disabled;
        self
    }

    /// Adjust resolved surface paint without losing the selected variant's geometry.
    pub fn map_style(mut self, paint: impl Fn(&Theme, Status, Style) -> Style + 'a) -> Self {
        let previous = self.paint.take();
        self.paint = Some(Box::new(move |theme, status, style| {
            let style = match &previous {
                Some(f) => f(theme, status, style),
                None => style,
            };
            paint(theme, status, style)
        }));
        self
    }
}

fn surface(variant: Variant, radius: Radius, theme: &Theme, status: Status) -> Style {
    use Variant::*;
    let primary = theme.primary.base;
    let (mut bg, fg) = match variant {
        Primary => (primary, theme.primary.foreground),
        Secondary => (theme.secondary.base, theme.secondary.foreground),
        Muted => (theme.muted.base, theme.neutral.foreground),
        Tinted => (
            Color::new(
                if theme.neutral.is_dark { 0.3 } else { 0.93 },
                primary.c * 0.4,
                primary.h,
            ),
            theme.neutral.foreground,
        ),
        Outline => (theme.neutral.base, theme.neutral.foreground),
        Ghost => (Color::TRANSPARENT, theme.neutral.foreground),
        Destructive => (
            theme
                .destructive
                .base
                .alpha(if theme.neutral.is_dark { 0.2 } else { 0.1 }),
            theme.destructive.base,
        ),
    };
    let hovered = matches!(
        status,
        Status::Hovered | Status::Pressed | Status::Focused { is_hovered: true }
    );
    if hovered {
        bg = match variant {
            Primary => primary.scale_alpha(0.8),
            Secondary | Muted => bg.blend(theme.neutral.foreground, 0.05),
            Tinted => Color::new(
                if theme.neutral.is_dark { 0.35 } else { 0.88 },
                primary.c * 0.5,
                primary.h,
            ),
            Outline if theme.neutral.is_dark => theme.input.scale_alpha(0.3),
            Ghost if theme.neutral.is_dark => theme.muted.base.scale_alpha(0.5),
            Outline | Ghost => theme.muted.base,
            Destructive => theme
                .destructive
                .base
                .scale_alpha(if theme.neutral.is_dark { 0.3 } else { 0.2 }),
        };
    }
    let focused = matches!(status, Status::Focused { .. });
    Style {
        background: Some(bg.into()),
        text_color: Some(fg),
        border: Border {
            radius,
            // CSS paints the surface under a transparent border. Iced cuts
            // the stroke out of the fill, so only paint a visible stroke.
            width: if focused || variant == Outline {
                1.
            } else {
                0.
            },
            color: if focused {
                theme.neutral.ring
            } else if variant == Outline {
                theme.neutral.border
            } else {
                Color::TRANSPARENT
            },
        },
        ..Style::default()
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Bubble<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
        self.align = self.align.or(from.conversation_alignment());
        self.content.inherit(from);
        if let Some((slot, ..)) = &mut self.reactions {
            slot.inherit(from);
        }
    }

    fn compose<'b>(mut self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let variant = self.variant;
        let align = self.align.unwrap_or(Horizontal::Left);
        self.content.inherit(
            &Inherited::default()
                .with_size(text::size::SM)
                .with_line_height(text::leading::RELAXED)
                .with_disabled(self.disabled),
        );
        if self.interactive {
            self.content
                .inherit(&Inherited::default().with_selectable(false));
        }
        let padding = self.padding.unwrap_or_else(|| {
            if variant == Ghost {
                Padding::ZERO
            } else {
                [10, 12].into()
            }
        });
        let radius = self
            .radius
            .unwrap_or_else(|| if variant == Ghost { 0.0 } else { radius::XL3 }.into());
        // iced draws its border inside the quad. CSS reserves a pixel on
        // each edge, including the transparent border on the ghost variant.
        let paint = self.paint.take();
        let resolve = move |theme: &Theme, status| {
            let style = surface(variant, radius, theme, status);
            match &paint {
                Some(f) => f(theme, status, style),
                None => style,
            }
        };
        let padding = padding + Padding::all(1.0);
        let content: Element<'a, Message, Theme, Renderer> = if self.interactive {
            let style: button::widget::StyleFn<'a, Theme> = Box::new(move |theme, status| {
                let style = resolve(theme, status);
                let ring = matches!(status, Status::Focused { .. }).then(|| {
                    let mut ring = iced_core::Border::default();
                    ring.width = 3.0;
                    ring.color = theme.neutral.ring.scale_alpha(0.5).into();
                    let r: iced_core::border::Radius = style.border.radius.into();
                    ring.radius = iced_core::border::Radius {
                        top_left: r.top_left + 3.0,
                        top_right: r.top_right + 3.0,
                        bottom_left: r.bottom_left + 3.0,
                        bottom_right: r.bottom_right + 3.0,
                    };
                    ring
                });
                button::widget::Style {
                    background: style.background.map(Into::into),
                    text_color: style.text_color.unwrap_or(theme.neutral.foreground).into(),
                    border: style.border.into(),
                    shadow: style.shadow.into(),
                    snap: style.snap,
                    ring,
                }
            });
            Button::new(self.content.compose())
                .padding(padding)
                .translate(false)
                .disabled(self.disabled)
                .on_press_maybe(self.on_press)
                .style(style)
                .boxed()
        } else {
            let style: container::widget::StyleFn<'a, Theme> = Box::new(move |theme| {
                let paint: iced_widget::container::Style = resolve(theme, Status::Active).into();
                paint.into()
            });
            Container::new(self.content.compose())
                .fit_content(true)
                .padding(padding)
                .class(style)
                .boxed()
        };
        let body: View<'a, Message> = if let Some((mut reaction, edge, side)) = self.reactions {
            reaction.inherit(
                &Inherited::default()
                    .with_size(Size::Sm)
                    .with_line_height(iced_core::Pixels(20.0))
                    .with_disabled(self.disabled),
            );
            let chip = container(reaction)
                .fit_content()
                .padding(self.reaction_padding)
                .style(|theme: &Theme| Style {
                    background: Some(theme.muted.base.into()),
                    text_color: Some(theme.neutral.foreground),
                    border: Border {
                        radius: 100.0.into(),
                        ..Border::default()
                    },
                    ..Style::default()
                });
            // The outer three pixels paint CSS's external ring. They do not
            // participate in either the 75% chip translation or the 12px inset.
            let ring: Element<'a, Message, Theme, Renderer> = container(chip)
                .padding(3)
                .style(|theme: &Theme| Style {
                    background: Some(theme.card.base.into()),
                    border: Border {
                        radius: 100.0.into(),
                        ..Border::default()
                    },
                    ..Style::default()
                })
                .into();
            let chip = float(ring)
                .translate(move |bounds, _| {
                    let distance = (bounds.height - 6.0) * 0.75 + 3.0;
                    Vector::new(
                        0.0,
                        if side == Vertical::Top {
                            -distance
                        } else {
                            distance
                        },
                    )
                })
                .boxed();
            let overlay: Element<'a, Message, Theme, Renderer> = container(chip)
                .padding([0, 9])
                .width(Fill)
                .height(Fill)
                .align_x(edge)
                .align_y(side)
                .into();
            stack![content, overlay].boxed().into()
        } else {
            content.into()
        };
        let limit = if variant == Ghost {
            Fill
        } else {
            FillPortion(4)
        };
        let body = container(body).width(limit).align_x(align);
        if variant == Ghost {
            return body.into();
        }
        let gutter = space().width(FillPortion(1));
        if align == Horizontal::Right {
            row![gutter, body].width(Fill).into()
        } else {
            row![body, gutter].width(Fill).into()
        }
    }
}

impl<'a, Message: Clone + 'a> From<Bubble<'a, Message>> for View<'a, Message> {
    fn from(bubble: Bubble<'a, Message>) -> Self {
        View::new(bubble)
    }
}
impl<'a, Message: Clone + 'a> From<Bubble<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(bubble: Bubble<'a, Message>) -> Self {
        Box::new(bubble).compose()
    }
}

type Paint<'a> = Box<dyn Fn(&Theme, Status, Style) -> Style + 'a>;
