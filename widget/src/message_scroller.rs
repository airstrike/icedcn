//! A transcript viewport with keyed rows, turn anchoring, live-edge following
//! and stable scroll position when earlier history is inserted.
mod viewport;
use crate::{Compose, Inherited, Slot, scroll_area};
use iced_core::Length::Fill;
use iced_core::{Element, Length};
use iced_widget::{Renderer, Widget, keyed_column};
use icedxdy_core::Theme;

/// Where the first layout displays a saved transcript.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    Start,
    End,
    #[default]
    LastAnchor,
}
/// An explicit navigation destination. Item targets use stable application keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Start,
    End,
    Item(u64),
}
/// A new sequence number requests another navigation, including to the same target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Command {
    pub sequence: u64,
    pub target: Target,
}
/// The measured viewport state, with intersecting rows in visual order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Reader {
    pub can_scroll_up: bool,
    pub can_scroll_down: bool,
    pub current_anchor: Option<u64>,
    pub visible: Vec<u64>,
}
/// One transcript row. Its key must be unique within this viewport.
pub struct Item<'a, Message> {
    id: u64,
    anchor: bool,
    content: Slot<'a, Message>,
}
/// A stable application key identifies an item across insertions and wrapping.
pub fn item<'a, Message: 'a>(id: u64, content: impl Into<Slot<'a, Message>>) -> Item<'a, Message> {
    Item {
        id,
        anchor: false,
        content: content.into(),
    }
}
impl<Message> Item<'_, Message> {
    pub fn anchor(mut self, is_anchor: bool) -> Self {
        self.anchor = is_anchor;
        self
    }
}

/// A native vertical scroll region. Following is opt-in and history preservation is on.
pub struct MessageScroller<'a, Message> {
    items: Vec<Item<'a, Message>>,
    height: Length,
    spacing: f32,
    padding: f32,
    options: viewport::Options,
    command: Option<Command>,
    on_reader: Option<Box<dyn Fn(Reader) -> Message + 'a>>,
}
/// Mount keyed rows in a transcript that defaults to the last anchored turn.
pub fn message_scroller<'a, Message>(
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> MessageScroller<'a, Message> {
    MessageScroller {
        items: items.into_iter().collect(),
        height: Length::Fill,
        spacing: 12.0,
        padding: 16.0,
        options: viewport::Options::default(),
        command: None,
        on_reader: None,
    }
}
impl<'a, Message> MessageScroller<'a, Message> {
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
    pub fn spacing(mut self, px: f32) -> Self {
        self.spacing = px.max(0.0);
        self
    }
    pub fn padding(mut self, px: f32) -> Self {
        self.padding = px.max(0.0);
        self
    }
    pub fn default_scroll_position(mut self, position: Position) -> Self {
        self.options.opening = position;
        self
    }
    pub fn auto_scroll(mut self, follow: bool) -> Self {
        self.options.follow = follow;
        self
    }
    pub fn preserve_scroll_on_prepend(mut self, preserve: bool) -> Self {
        self.options.preserve = preserve;
        self
    }
    pub fn scroll_margin(mut self, px: f32) -> Self {
        self.options.margin = px.max(0.0);
        self
    }
    pub fn previous_item_peek(mut self, px: f32) -> Self {
        self.options.peek = px.max(0.0);
        self
    }
    /// Animate explicit navigation to a conversation item, the start, or the end.
    pub fn smooth_commands(mut self, smooth: bool) -> Self {
        self.options.smooth_commands = smooth;
        self
    }
    pub fn command(mut self, command: Option<Command>) -> Self {
        self.command = command;
        self
    }
    pub fn on_reader_change(mut self, callback: impl Fn(Reader) -> Message + 'a) -> Self {
        self.on_reader = Some(Box::new(callback));
        self
    }
}
impl<'a, Message: 'a> Compose<Message> for MessageScroller<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        for item in &mut self.items {
            item.content.inherit(from);
        }
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let ids = self.items.iter().map(|i| (i.id, i.anchor)).collect();
        let content = keyed_column(self.items.into_iter().map(|i| (i.id, i.content.compose())))
            .width(Fill)
            .spacing(self.spacing)
            .padding(self.padding);
        let scroll = scroll_area(content.boxed()).height(self.height);
        viewport::Viewport {
            child: scroll.into(),
            ids,
            options: self.options,
            command: self.command,
            on_reader: self.on_reader,
        }
        .boxed()
    }
}
impl<'a, Message: 'a> From<MessageScroller<'a, Message>> for Slot<'a, Message> {
    fn from(view: MessageScroller<'a, Message>) -> Self {
        Slot::new(view)
    }
}
impl<'a, Message: 'a> From<MessageScroller<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(view: MessageScroller<'a, Message>) -> Self {
        Box::new(view).compose()
    }
}
