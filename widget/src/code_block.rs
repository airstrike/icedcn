//! Code, terminal output and diffs with an optional header, line numbers and a copy button.
//!
//! A block composes the same parts as the source component:
//!
//! ```ignore
//! code_block(OUTPUT)
//!     .header(code_block::header("Terminal").actions(code_block::copy_button(OUTPUT)))
//! ```
//!
//! A string is the content; [`content`] sets its props. [`CodeBlock::actions`]
//! floats them over the content, and [`CodeBlock::footer`] sits below it.
mod actions;
pub mod ansi;
mod content;
mod copy;
mod editor;
mod floating;
mod footer;
mod frame;
mod glyph;
mod header;
mod lines;
mod listing;
mod selection;
mod show_more;
#[cfg(feature = "highlighter")]
mod syntax;
mod viewport;

pub use crate::__code_block_actions as actions;
pub use actions::{Actions, actions};
pub use content::{Content, Line, LineRange, content, line};
pub use copy::{CopyButton, copy_button};
pub use footer::{Footer, footer};
pub use header::{Header, HeaderItem, Title, TitleItem, header, title};
pub use icedcn_core::theme::code_block::{Default, Destructive, Spec};
pub use show_more::show_more;

use std::borrow::Cow;

use iced_core::Element;
use iced_core::Length::{Fill, Shrink};
use iced_core::alignment::Horizontal;
use iced_core::text::{LineHeight, Wrapping};
use iced_core::widget::Id;
use iced_core::{Length, Padding, Pixels};
use iced_widget::{Renderer, Widget, text};
use icedcn_core::Theme;
use icedcn_core::theme::container::Style;

use crate::Size;
use crate::compose::{Compose, Inherited, View};
use crate::scrollable::{Direction, scrollbar};
use crate::{column, container, scrollable};

/// The state of a line: its tint and its `+` or `−` marker.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum LineStyle {
    #[default]
    Default,
    Highlighted,
    Added,
    Removed,
}

pub use LineStyle::{Added, Highlighted, Removed};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Placement {
    Header,
    Floating,
}

/// A block showing `content`: a string, a list of [`line`]s, or [`content`] with its props.
pub fn code_block<'a, Message: 'a>(
    content: impl Into<Content<'a, Message>>,
) -> CodeBlock<'a, Message> {
    CodeBlock {
        header: None,
        actions: None,
        content: content.into(),
        footer: None,
        size: Size::Default,
        style: Spec::default(),
        disabled: false,
    }
}

/// The surface that holds the header, the content and the footer.
pub struct CodeBlock<'a, Message: 'a> {
    header: Option<Header<'a, Message>>,
    actions: Option<Actions<'a, Message>>,
    content: Content<'a, Message>,
    footer: Option<Footer<'a, Message>>,
    size: Size,
    style: Spec,
    disabled: bool,
}

impl<'a, Message: 'a> CodeBlock<'a, Message> {
    /// The row above the content: a [`header`] with a title and actions.
    pub fn header(mut self, header: impl Into<Header<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Actions that float over the top end of the content and show on hover or focus.
    pub fn actions(mut self, actions: impl Into<Actions<'a, Message>>) -> Self {
        self.actions = Some(actions.into());
        self
    }

    /// The row below the content.
    pub fn footer(mut self, footer: impl Into<Footer<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// The tone of the block: the default panel, [`Destructive`], or either with tweaks.
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = style.into();
        self
    }

    /// `Size::Sm` has tighter padding and 12px code, for tool output.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    fn is_compact(&self) -> bool {
        matches!(self.size, Size::Sm | Size::Xs)
    }
}

/// The header ink: muted, or the panel's own ink on a custom surface.
fn header_ink(spec: &Spec, theme: &Theme) -> Style {
    let ink = spec.resolve(theme).text_color;
    Style {
        text_color: Some(match ink {
            Some(ink) if ink != theme.neutral.foreground => ink,
            _ => theme.muted.foreground,
        }),
        ..Style::default()
    }
}

impl<'a, Message: 'a> Compose<Message> for CodeBlock<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        // The block sets its own type; only the disabled state cascades in.
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let compact = self.is_compact();
        let Self {
            header,
            mut actions,
            content,
            footer,
            style,
            disabled,
            ..
        } = *self;
        let size = if compact { 12. } else { 14. };
        let padding = if compact { 10. } else { 12. };
        let disabled = Inherited::default().with_disabled(disabled);
        let mut content = content;
        content.inherit(&disabled);
        let wrap = content.wrap;
        let mut panel = column![].width(Fill);
        let has_header = header.is_some();
        if let Some(mut header) = header {
            header.inherit(&disabled);
            let spec = style.clone();
            panel = panel.push(
                container(header.view(compact, style.clone()))
                    .style(move |theme: &Theme| header_ink(&spec, theme))
                    .padding(Padding {
                        top: padding,
                        left: padding,
                        right: padding,
                        bottom: 0.,
                    })
                    .width(Fill)
                    .into(),
            );
        }
        if let Some(actions) = &mut actions {
            actions.inherit(&disabled.with_code_actions(Placement::Floating));
        }
        let (body, viewport) = content.view(compact, disabled.is_disabled());
        // A filling content fills the whole block, so every wrapper fills too.
        let height = if viewport.height.is_some_and(|height| height.is_fill()) {
            Fill
        } else {
            Shrink
        };
        // An editor fills the viewport, so a click right of its text lands in it.
        let content_width = if wrap || viewport.editable {
            Fill
        } else {
            Shrink
        };
        let body = container(body)
            .padding(Padding {
                top: if has_header {
                    if compact { 6. } else { 8. }
                } else {
                    padding
                },
                bottom: if footer.is_some() { 8. } else { padding },
                left: 0.,
                right: 0.,
            })
            .width(content_width)
            .height(height);
        let direction = if wrap {
            Direction::Vertical(scrollbar())
        } else {
            Direction::Both {
                vertical: scrollbar(),
                horizontal: scrollbar(),
            }
        };
        let mut view = scrollable(body).direction(direction).width(Fill);
        if let Some(height) = viewport.height {
            view = view.height(height);
        } else if let Some(max) = viewport.max_height {
            view = view.height(Shrink.max(max));
        }
        let body: Element<'_, Message, Theme, Renderer> = viewport::Viewport::new(
            view.into(),
            viewport.id,
            viewport.focusable && !disabled.is_disabled(),
            viewport.collapsed,
            wrap || viewport.height.is_some() || viewport.max_height.is_some(),
        )
        .boxed();
        let body: View<'_, Message> = if let Some(actions) = actions {
            let top: Element<'_, Message, Theme, Renderer> = container(actions)
                .width(Fill)
                .align_x(Horizontal::Right)
                .padding(if compact { 6 } else { 8 })
                .into();
            floating::Hover::new(body, top).boxed().into()
        } else {
            body.into()
        };
        panel = panel.push(body);
        if let Some(mut footer) = footer {
            footer.inherit(&disabled.with_size(Size::Custom(size)));
            panel = panel.push(
                container(footer)
                    .style(|theme: &Theme| Style {
                        text_color: Some(theme.muted.foreground),
                        ..Style::default()
                    })
                    .padding(Padding {
                        bottom: padding,
                        left: padding,
                        right: padding,
                        top: 0.,
                    })
                    .width(Fill)
                    .into(),
            );
        }
        frame::Frame::new(
            container(panel.height(height))
                .padding(1)
                .width(Fill)
                .height(height)
                .clip(true)
                .into(),
            style,
        )
        .boxed()
    }
}

/// What the scrolling viewport needs from the content.
struct Viewport {
    height: Option<Length>,
    max_height: Option<Pixels>,
    collapsed: bool,
    id: Option<Id>,
    focusable: bool,
    editable: bool,
}

fn plain<'a>(source: Cow<'a, str>, layout: listing::Layout) -> iced_widget::Text<'a, Theme> {
    text(source)
        .size(layout.size())
        .font(layout.font)
        .line_height(LineHeight::Absolute(layout.line_height().into()))
        .wrapping(if layout.wrap {
            Wrapping::WordOrGlyph
        } else {
            Wrapping::None
        })
        .selectable(true)
}

fn line_style(theme: &Theme, style: LineStyle) -> Style {
    let color = match style {
        LineStyle::Default => return Style::default(),
        LineStyle::Highlighted | LineStyle::Added => theme.muted.base,
        LineStyle::Removed => theme.destructive.base.alpha(0.1),
    };
    Style {
        background: Some(color.into()),
        ..Style::default()
    }
}

impl<'a, Message: 'a> From<CodeBlock<'a, Message>> for View<'a, Message> {
    fn from(block: CodeBlock<'a, Message>) -> Self {
        View::new(block)
    }
}
impl<'a, Message: 'a> From<CodeBlock<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(block: CodeBlock<'a, Message>) -> Self {
        Box::new(block).compose()
    }
}
