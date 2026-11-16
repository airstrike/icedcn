//! Code, terminal output and diffs with an optional header, line numbers and a copy button.
//!
//! A block composes the same parts as the source component:
//!
//! ```ignore
//! code_block![
//!     code_block::header![
//!         code_block::title("Terminal"),
//!         code_block::actions![code_block::copy_button(OUTPUT)],
//!     ],
//!     OUTPUT,
//! ]
//! ```
//!
//! A string child is the content. [`content`] sets its props, [`actions`] as a
//! direct child floats over the content, and [`footer`] sits below it.
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

#[doc(inline)]
pub use crate::{actions, footer, header, title};
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
use iced_core::{Padding, Pixels};
use iced_widget::{Renderer, Widget, text};
use icedcn_core::Theme;
use icedcn_core::theme::container::Style;

use crate::Size;
use crate::compose::{Compose, Inherited, Slot};
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

/// Build a block from its parts: a header, floating actions, content and a footer.
///
/// ```ignore
/// code_block![code_block::actions![code_block::copy_button(OUTPUT)], OUTPUT]
/// ```
#[macro_export]
macro_rules! code_block {
    ($first:expr $(, $rest:expr)* $(,)?) => {
        $crate::code_block::code_block($first)$(.push($rest))*
    };
}

/// A block holding one part. A string is the content.
pub fn code_block<'a, Message: 'a>(part: impl Into<Part<'a, Message>>) -> CodeBlock<'a, Message> {
    CodeBlock {
        header: None,
        actions: None,
        content: content(""),
        footer: None,
        size: Size::Default,
        style: Spec::default(),
        disabled: false,
    }
    .push(part)
}

/// A direct child of a [`CodeBlock`].
pub enum Part<'a, Message> {
    Header(Header<'a, Message>),
    Actions(Actions<'a, Message>),
    Content(Content<'a, Message>),
    Footer(Footer<'a, Message>),
}

impl<'a, Message> From<Header<'a, Message>> for Part<'a, Message> {
    fn from(header: Header<'a, Message>) -> Self {
        Part::Header(header)
    }
}
impl<'a, Message> From<Actions<'a, Message>> for Part<'a, Message> {
    fn from(actions: Actions<'a, Message>) -> Self {
        Part::Actions(actions)
    }
}
impl<'a, Message> From<Content<'a, Message>> for Part<'a, Message> {
    fn from(content: Content<'a, Message>) -> Self {
        Part::Content(content)
    }
}
impl<'a, Message> From<Footer<'a, Message>> for Part<'a, Message> {
    fn from(footer: Footer<'a, Message>) -> Self {
        Part::Footer(footer)
    }
}
impl<'a, Message> From<&'a str> for Part<'a, Message> {
    fn from(source: &'a str) -> Self {
        Part::Content(content(source))
    }
}
impl<'a, Message> From<&'a String> for Part<'a, Message> {
    fn from(source: &'a String) -> Self {
        Part::Content(content(source))
    }
}
impl<'a, Message> From<String> for Part<'a, Message> {
    fn from(source: String) -> Self {
        Part::Content(content(source))
    }
}
impl<'a, Message> From<Cow<'a, str>> for Part<'a, Message> {
    fn from(source: Cow<'a, str>) -> Self {
        Part::Content(content(source))
    }
}
impl<'a, Message> From<Vec<Line<'a, Message>>> for Part<'a, Message> {
    fn from(lines: Vec<Line<'a, Message>>) -> Self {
        Part::Content(content(lines))
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
    /// Add a part. A later part of the same kind replaces the earlier one.
    pub fn push(mut self, part: impl Into<Part<'a, Message>>) -> Self {
        match part.into() {
            Part::Header(header) => self.header = Some(header),
            Part::Actions(actions) => self.actions = Some(actions),
            Part::Content(content) => self.content = content,
            Part::Footer(footer) => self.footer = Some(footer),
        }
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
        let content_wrap = content.wrap;
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
        // An editor wraps or scrolls its long lines itself.
        let wrap = content_wrap || viewport.editable;
        let content_width = if wrap { Fill } else { Shrink };
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
            .width(content_width);
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
        let body: Slot<'_, Message> = if let Some(actions) = actions {
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
            container(panel).padding(1).width(Fill).clip(true).into(),
            style,
        )
        .boxed()
    }
}

/// What the scrolling viewport needs from the content.
struct Viewport {
    height: Option<Pixels>,
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

impl<'a, Message: 'a> From<CodeBlock<'a, Message>> for Slot<'a, Message> {
    fn from(block: CodeBlock<'a, Message>) -> Self {
        Slot::new(block)
    }
}
impl<'a, Message: 'a> From<CodeBlock<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(block: CodeBlock<'a, Message>) -> Self {
        Box::new(block).compose()
    }
}
