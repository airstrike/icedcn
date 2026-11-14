//! A transcript viewport with keyed rows, turn anchoring, live-edge following
//! and stable scroll position when earlier history is inserted.
mod button;
mod viewport;
use crate::{Compose, Inherited, Slot, scroll_area};
use iced_core::Length::Fill;
use iced_core::widget::Id;
pub use iced_core::widget::operation::Animation;
use iced_core::{Element, Length};
use iced_runtime::{Action, Task, task};
use iced_widget::{Renderer, Widget, column};
use icedcn_core::Theme;

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
/// Scroll an identified viewport to an edge or a stable item key.
/// Return this task from `update` each time the user requests a jump.
pub fn scroll_to<Message>(
    id: impl Into<Id>,
    target: Target,
    animation: Animation,
) -> Task<Message> {
    task::effect(Action::widget(viewport::navigate(
        id.into(),
        target,
        animation,
    )))
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
pub struct Scroller<'a, Message> {
    items: Vec<Item<'a, Message>>,
    height: Length,
    spacing: f32,
    padding: f32,
    options: viewport::Options,
    end: Option<Element<'a, Message, Theme, Renderer>>,
    id: Option<Id>,
    on_reader: Option<Box<dyn Fn(Reader) -> Message + 'a>>,
}
/// Mount keyed rows in a transcript that defaults to the last anchored turn.
pub fn scroller<'a, Message>(
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> Scroller<'a, Message> {
    Scroller {
        items: items.into_iter().collect(),
        height: Length::Fill,
        spacing: 12.0,
        padding: 16.0,
        options: viewport::Options::default(),
        end: None,
        id: None,
        on_reader: None,
    }
}
impl<'a, Message> Scroller<'a, Message> {
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
    /// Fade unread content into the supplied background at the bottom edge.
    pub fn fade_to(mut self, background: fn(&Theme) -> iced_core::Color) -> Self {
        self.options.fade = Some(background);
        self
    }
    /// Show a floating end button while the reader can scroll down.
    /// Supply the icon and handle its press in `update` with [`scroll_to`].
    pub fn end_button<Icon: Into<Slot<'a, Message>>>(
        mut self,
        visible: bool,
        icon: impl Fn(f32) -> Icon + 'a,
        press: Message,
    ) -> Self
    where
        Message: Clone + 'a,
    {
        self.end = Some(button::end(visible, icon, press));
        self
    }
    /// Identify this scroller for [`scroll_to`] and focus operations.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn on_reader_change(mut self, on_change: impl Fn(Reader) -> Message + 'a) -> Self {
        self.on_reader = Some(Box::new(on_change));
        self
    }
}
impl<'a, Message: 'a> Compose<Message> for Scroller<'a, Message> {
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
        let content = column(self.items.into_iter().map(|i| i.content.compose()))
            .width(Fill)
            .spacing(self.spacing)
            .padding(self.padding);
        let scroll = scroll_area(content.boxed()).width(Fill).height(self.height);
        let viewport = viewport::Viewport {
            child: scroll.into(),
            ids,
            options: self.options,
            id: self.id,
            on_reader: self.on_reader,
        }
        .boxed();
        if let Some(end) = self.end {
            let layers = iced_widget::stack([viewport, end])
                .width(Fill)
                .height(self.height);
            iced_widget::container(layers).clip(true).boxed()
        } else {
            viewport
        }
    }
}
impl<'a, Message: 'a> From<Scroller<'a, Message>> for Slot<'a, Message> {
    fn from(view: Scroller<'a, Message>) -> Self {
        Slot::new(view)
    }
}
impl<'a, Message: 'a> From<Scroller<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(view: Scroller<'a, Message>) -> Self {
        Box::new(view).compose()
    }
}
