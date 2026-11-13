//! Framed conversational content with optional corner reactions.
use iced_core::Length::{Fill, FillPortion};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Element, Padding};
use iced_widget::{Renderer, Widget, stack};
use icedxdy_core::theme::container::Style;
use icedxdy_core::{Border, Color, Theme, radius};

use crate::{Column, Compose, Inherited, Size, Slot, column, container, row, space};

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

pub struct Bubble<'a, Message> {
    content: Slot<'a, Message>,
    variant: Variant,
    align: Option<Horizontal>,
    reactions: Option<(Slot<'a, Message>, Horizontal, Vertical)>,
}

pub fn bubble<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Bubble<'a, Message> {
    Bubble {
        content: content.into(),
        variant: Variant::Primary,
        align: None,
        reactions: None,
    }
}

/// Adjacent bubbles in one turn use the tighter Nova group spacing.
pub fn group<'a, Message: 'a>(
    items: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Column<'a, Message> {
    column(items).spacing(8).width(Fill)
}

impl<'a, Message: 'a> Bubble<'a, Message> {
    pub fn style(mut self, style: Variant) -> Self {
        self.variant = style;
        self
    }
    pub fn align(mut self, align: Horizontal) -> Self {
        self.align = Some(align);
        self
    }
    pub fn reactions(
        mut self,
        content: impl Into<Slot<'a, Message>>,
        align: Horizontal,
        side: Vertical,
    ) -> Self {
        self.reactions = Some((content.into(), align, side));
        self
    }
}

fn surface(variant: Variant, theme: &Theme) -> Style {
    use Variant::*;
    let primary = theme.primary.base;
    let (bg, fg) = match variant {
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
    Style {
        background: Some(bg.into()),
        text_color: Some(fg),
        border: Border {
            radius: radius::XL.into(),
            width: if variant == Outline { 1. } else { 0. },
            color: theme.neutral.border,
        },
        ..Style::default()
    }
}

impl<'a, Message: 'a> Compose<Message> for Bubble<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
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
                .with_size(Size::Sm)
                .with_line_height(22.75),
        );
        let content = container(self.content)
            .padding(if variant == Ghost { [0, 0] } else { [8, 12] })
            .style(move |theme| surface(variant, theme));
        let body: Slot<'a, Message> = if let Some((mut reaction, edge, side)) = self.reactions {
            reaction.inherit(&Inherited::default().with_size(Size::Sm));
            let chip = container(reaction).padding([2, 6]).style(|theme| Style {
                background: Some(theme.muted.base.into()),
                text_color: Some(theme.neutral.foreground),
                border: Border {
                    radius: 100.0.into(),
                    width: 3.,
                    color: theme.card.base,
                },
                ..Style::default()
            });
            let mut gutter = Padding::ZERO;
            if side == Vertical::Top {
                gutter.top = 16.;
            } else {
                gutter.bottom = 16.;
            }
            let base: Element<'a, Message, Theme, Renderer> =
                container(content).padding(gutter).into();
            let overlay: Element<'a, Message, Theme, Renderer> = container(chip)
                .padding([0, 12])
                .width(Fill)
                .height(Fill)
                .align_x(edge)
                .align_y(side)
                .into();
            stack![base, overlay].boxed().into()
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

impl<'a, Message: 'a> From<Bubble<'a, Message>> for Slot<'a, Message> {
    fn from(bubble: Bubble<'a, Message>) -> Self {
        Slot::new(bubble)
    }
}
impl<'a, Message: 'a> From<Bubble<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(bubble: Bubble<'a, Message>) -> Self {
        Box::new(bubble).compose()
    }
}
