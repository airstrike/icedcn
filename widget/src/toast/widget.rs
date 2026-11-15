//! The widget behind [`Toaster`](super::Toaster): its content in place,
//! its toasts in an overlay, and the clock that dismisses them.

use std::time::Duration;

use iced_core::Length::Fill;
use iced_core::time::Instant;
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme::toast;

use super::{Icons, Inbox, Kind, Position, Raise, Toast};
use crate::button::button;
use crate::column::Column;
use crate::compose::Slot;
use crate::container::Container;
use crate::core::Renderer as _;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::{
    Alignment, Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Vector, Widget,
};
use crate::core::{keyboard, touch, window};
use crate::marker::{self, Press};
use crate::row::Row;
use crate::size;
use crate::text::Text;
use iced_widget::text;

/// Toast width in logical pixels.
const WIDTH: f32 = 356.0;
/// Distance between the stack and the window edges.
const OFFSET: f32 = 24.0;
const MOBILE_OFFSET: f32 = 16.;
/// Vertical gap between expanded toasts.
const GAP: f32 = 14.0;
/// Maximum number of visible toasts.
const VISIBLE: usize = 3;

// These are the only events that can activate a toast trigger or its action.
fn can_activate(event: &Event) -> bool {
    matches!(
        event,
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Keyboard(keyboard::Event::KeyPressed { .. })
    )
}

/// The signal a toast's action records: the toast to dismiss.
#[derive(Debug, Clone, Copy)]
struct Dismiss(u64);

/// Content with a stack of toasts over it.
pub struct Toaster<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    position: Position,
    duration: Duration,
    icons: Icons<'a, Message>,
    /// The cards of the live toasts, rebuilt from the tree in `diff`.
    cards: Vec<Card<'a, Message>>,
}

/// A live toast's card.
struct Card<'a, Message> {
    id: u64,
    position: Position,
    element: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message> Toaster<'a, Message> {
    /// Show the toasts raised in `content` at `position` for `duration`.
    pub fn new(
        content: Element<'a, Message, Theme, Renderer>,
        position: Position,
        duration: Duration,
        icons: Icons<'a, Message>,
    ) -> Self {
        Self {
            content,
            position,
            duration,
            icons,
            cards: Vec::new(),
        }
    }
}

impl<'a, Message: Clone + 'a> Toaster<'a, Message> {
    /// `flex items-center gap-1.5 p-4` around the icon, the title over the
    /// description, and the action.
    fn card(&self, id: u64, toast: &Toast<Message>) -> Element<'a, Message, Theme, Renderer> {
        let mut copy = vec![Slot::from(
            Text::new(toast.title.clone())
                .size(13)
                .line_height(1.5)
                .medium(),
        )];

        if let Some(description) = &toast.description {
            copy.push(Slot::from(
                Text::new(description.clone())
                    .size(13)
                    .line_height(1.4)
                    .style(Box::new(toast::description) as text::StyleFn<'a, Theme>),
            ));
        }

        let mut row: Vec<Slot<'a, Message>> = Vec::new();

        if let Some(icon) = (self.icons)(toast.kind) {
            row.push(icon);
        }

        row.push(Slot::from(
            Column::with_children(copy).spacing(2).width(Fill),
        ));

        if let Some((label, message)) = &toast.action {
            // `h-6 px-2 text-xs font-medium`.
            let action = button(Text::new(label.clone()))
                .size(size::Size::Xs)
                .style(
                    Box::new(|theme: &Theme, status| toast::action(theme, status).into())
                        as iced_widget::button::StyleFn<'a, Theme>,
                )
                .on_press(message.clone());

            let action: Element<'a, Message, Theme, Renderer> =
                marker::marker(Slot::from(action).compose(), Dismiss(id), Press::Captured).into();
            row.push(Slot::from(action));
        }

        Container::new(
            Row::with_children(row)
                .spacing(6)
                .align_y(Alignment::Center)
                .width(Fill),
        )
        .width(Fill)
        .padding(17)
        .style(toast::card)
        .into()
    }
}

impl<Message> Meta for Toaster<'_, Message> {}

impl<'a, Message: Clone + Send + 'static> Widget<Message, Theme, Renderer>
    for Toaster<'a, Message>
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Message>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Message> {
            live: Vec::new(),
            inbox: Inbox(Vec::new()),
            next_id: 0,
            hover: None,
            swipe: None,
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        let state = tree.state.downcast_ref::<State<Message>>();

        self.cards = state
            .live
            .iter()
            .map(|live| Card {
                id: live.id,
                position: live.toast.position.unwrap_or(self.position),
                element: self.card(live.id, &live.toast),
            })
            .collect();

        let mut children: Vec<&mut Element<'a, Message, Theme, Renderer>> =
            std::iter::once(&mut self.content)
                .chain(self.cards.iter_mut().map(|card| &mut card.element))
                .collect();

        tree.diff_children_custom(
            &mut children,
            |tree, element| tree.diff(&mut **element),
            |element| Tree::new(&**element),
        );
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State<Message>>();
        let (child_layout, child_tree) = layout
            .iter_mut(&mut children[..1])
            .next()
            .expect("a toaster has its content");

        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        let mut toasts = std::mem::take(&mut state.inbox.0);
        if can_activate(event) {
            let mut raised = marker::drain::<Raise<Message>>();
            self.content
                .operate(child_tree, child_layout, viewport, renderer, &mut raised);
            toasts.extend(raised.into_signals().into_iter().map(|Raise(toast)| toast));
        }

        if !toasts.is_empty() {
            for toast in toasts {
                state.raise(toast);
            }

            shell.invalidate_widgets();
            shell.request_redraw();
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let before = state.live.len();
            let duration = self.duration;
            let hover = state.hover.map(|(pos, _)| pos);

            for live in &mut state.live {
                live.shown_at.get_or_insert(*now);
            }

            state.live.retain(|live| {
                Some(live.toast.position.unwrap_or(self.position)) == hover
                    || live.expires_at(duration).is_none_or(|end| *now < end)
            });

            if state.live.len() != before {
                shell.invalidate_widgets();
            }

            if let Some(next) = state
                .live
                .iter()
                .filter(|live| Some(live.toast.position.unwrap_or(self.position)) != hover)
                .filter_map(|live| live.expires_at(duration))
                .min()
            {
                shell.request_redraw_at(window::RedrawRequest::At(next));
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let (child_layout, child_tree) = layout
            .iter(&tree.children[..1])
            .next()
            .expect("a toaster has its content");

        self.content.draw(
            child_tree,
            renderer,
            theme,
            style,
            child_layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (child_layout, child_tree) = layout
            .iter(&tree.children[..1])
            .next()
            .expect("a toaster has its content");

        self.content
            .mouse_interaction(child_tree, child_layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State<Message>>();
        operation.custom(None, layout.bounds(), &mut state.inbox);

        let (child_layout, child_tree) = layout
            .iter_mut(&mut children[..1])
            .next()
            .expect("a toaster has its content");

        self.content
            .operate(child_tree, child_layout, viewport, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let Tree {
            state, children, ..
        } = tree;
        let (content_tree, card_trees) = children.split_at_mut(1);

        let (child_layout, content_tree) = layout
            .iter_mut(content_tree)
            .next()
            .expect("a toaster has its content");

        let mut overlays = self.content.overlay(
            content_tree,
            child_layout,
            renderer,
            viewport,
            translation,
            window,
        );

        if self.cards.is_empty() {
            return overlays;
        }

        let state = state.downcast_mut::<State<Message>>();
        let (edge, width) = if window.width <= 600. {
            (MOBILE_OFFSET, (window.width - 2. * MOBILE_OFFSET).max(0.))
        } else {
            (OFFSET, WIDTH)
        };
        let limits = layout::Limits::new(Size::new(width, 0.), Size::new(width, window.height));

        // Per position, the newest toasts sit nearest their edge; older
        // ones past the visible count wait off screen.
        let mut stacks: Vec<(Position, f32, usize, f32)> = Vec::new();
        let mut entries = Vec::new();

        for (card, tree) in self.cards.iter_mut().zip(card_trees.iter_mut()).rev() {
            let index = match stacks.iter().position(|(at, ..)| *at == card.position) {
                Some(index) => index,
                None => {
                    stacks.push((card.position, 0.0, 0, 0.));
                    stacks.len() - 1
                }
            };
            let (_, offset, count, front_height) = &mut stacks[index];

            if *count == VISIBLE {
                continue;
            }

            card.element.layout(tree, renderer, &limits);
            let size = tree.size;

            if *count == 0 {
                *front_height = size.height;
            }
            let behind = *count > 0 && !state.hover.is_some_and(|(pos, _)| pos == card.position);
            let scale = if behind {
                1. - 0.05 * *count as f32
            } else {
                1.
            };
            let display = if behind {
                Size::new(width * scale, *front_height * scale)
            } else {
                size
            };
            let x = match card.position {
                Position::TopLeft | Position::BottomLeft => edge,
                Position::TopCenter | Position::BottomCenter => (window.width - width) / 2.,
                Position::TopRight | Position::BottomRight => window.width - edge - width,
            } + (width - display.width) / 2.;
            let inset = if behind {
                GAP * *count as f32 + (*front_height - display.height) / 2.
            } else {
                *offset
            };
            let y = match card.position {
                Position::TopLeft | Position::TopCenter | Position::TopRight => edge + inset,
                Position::BottomLeft | Position::BottomCenter | Position::BottomRight => {
                    window.height - edge - inset - display.height
                }
            };
            *offset += size.height + GAP;
            *count += 1;
            let shift = state
                .swipe
                .as_ref()
                .filter(|s| s.id == card.id)
                .map_or(Vector::ZERO, |s| s.offset);
            entries.push(Entry {
                id: card.id,
                position: card.position,
                behind,
                element: &mut card.element,
                tree,
                layout: Layout::new(display).move_to(Point::new(x, y) + shift),
            });
        }

        overlays.push(overlay::Element::new(Box::new(Stack {
            entries,
            state,
            window,
            default_position: self.position,
        })));

        overlays
    }
}

/// The toaster's tree state.
struct State<Message> {
    live: Vec<Live<Message>>,
    inbox: Inbox<Message>,
    next_id: u64,
    hover: Option<(Position, Instant)>,
    swipe: Option<Swipe>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Contact {
    Press,
    Move,
    Release,
    Lost,
}

impl Contact {
    fn from_event(
        event: &Event,
        cursor: mouse::Cursor,
    ) -> Option<(Option<touch::Finger>, Point, Self)> {
        match event {
            Event::Touch(touch::Event::FingerPressed { id, position }) => {
                Some((Some(*id), *position, Contact::Press))
            }
            Event::Touch(touch::Event::FingerMoved { id, position }) => {
                Some((Some(*id), *position, Contact::Move))
            }
            Event::Touch(touch::Event::FingerLifted { id, position }) => {
                Some((Some(*id), *position, Contact::Release))
            }
            Event::Touch(touch::Event::FingerLost { id, position }) => {
                Some((Some(*id), *position, Contact::Lost))
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                cursor.position().map(|p| (None, p, Contact::Press))
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some((None, *position, Contact::Move))
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                cursor.position().map(|p| (None, p, Contact::Release))
            }
            _ => None,
        }
    }
}

/// The pointer which pressed a notification owns that drag until release.
struct Swipe {
    id: u64,
    finger: Option<touch::Finger>,
    position: Position,
    start: Point,
    offset: Vector,
}

impl Swipe {
    fn offset(&self, point: Point) -> Vector {
        use Position::*;
        let delta = point - self.start;
        let y = match self.position {
            TopLeft | TopCenter | TopRight => delta.y.min(0.),
            _ => delta.y.max(0.),
        };
        let x = match self.position {
            TopLeft | BottomLeft => delta.x.min(0.),
            TopRight | BottomRight => delta.x.max(0.),
            _ => 0.,
        };
        if x.abs() > y.abs() {
            Vector::new(x, 0.)
        } else {
            Vector::new(0., y)
        }
    }
}

/// A toast on screen, timed from its first frame.
struct Live<Message> {
    id: u64,
    toast: Toast<Message>,
    shown_at: Option<Instant>,
}

impl<Message> State<Message> {
    fn hover(&mut self, position: Option<Position>, now: Instant, default: Position) -> bool {
        if self.hover.map(|(p, _)| p) == position {
            return false;
        }
        if let Some((previous, paused)) = self.hover.take() {
            for live in &mut self.live {
                if live.toast.position.unwrap_or(default) == previous
                    && let Some(started) = &mut live.shown_at
                {
                    *started += now.saturating_duration_since(paused.max(*started));
                }
            }
        }
        self.hover = position.map(|p| (p, now));
        true
    }
    fn raise(&mut self, toast: Toast<Message>) {
        if let Some(key) = &toast.key
            && let Some(live) = self
                .live
                .iter_mut()
                .find(|live| live.toast.key.as_ref() == Some(key))
        {
            live.toast = toast;
            live.shown_at = None;
        } else {
            self.live.push(Live {
                id: self.next_id,
                toast,
                shown_at: None,
            });
            self.next_id += 1;
        }
    }
}

impl<Message> Live<Message> {
    fn expires_at(&self, duration: Duration) -> Option<Instant> {
        if self.toast.kind == Kind::Loading {
            return None;
        }
        self.shown_at
            .map(|start| start + self.toast.duration.unwrap_or(duration))
    }
}

/// A laid-out card in the overlay.
struct Entry<'a, 'b, Message> {
    id: u64,
    position: Position,
    behind: bool,
    element: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut Tree,
    layout: Layout,
}

/// The toasts over the window.
struct Stack<'a, 'b, Message> {
    entries: Vec<Entry<'a, 'b, Message>>,
    state: &'b mut State<Message>,
    window: Size,
    default_position: Position,
}

impl<Message: Clone + Send + 'static> overlay::Overlay<Message, Theme, Renderer>
    for Stack<'_, '_, Message>
{
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        let viewport = Rectangle::with_size(self.window);

        for entry in self.entries.iter().rev() {
            if entry.behind {
                let frame = toast::card(theme);
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: entry.layout.bounds(),
                        border: frame.border.into(),
                        shadow: frame.shadow.into(),
                        ..Default::default()
                    },
                    iced_core::Color::from(theme.popover.base),
                );
                continue;
            }
            renderer.with_layer(viewport, |renderer| {
                entry.element.draw(
                    entry.tree,
                    renderer,
                    theme,
                    style,
                    entry.layout,
                    cursor,
                    &viewport,
                );
            });
        }
    }

    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn Operation) {
        let viewport = Rectangle::with_size(self.window);

        for entry in self.entries.iter_mut().filter(|entry| !entry.behind) {
            entry
                .element
                .operate(entry.tree, entry.layout, &viewport, renderer, operation);
        }
    }

    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        let viewport = Rectangle::with_size(self.window);
        let mut dismissed = Vec::new();
        let pointed = self
            .entries
            .iter()
            .find(|entry| cursor.is_over(entry.layout.bounds()))
            .map(|entry| entry.position);
        // Keep a stack expanded over the gaps between its cards.
        let hovered = pointed.or_else(|| {
            let (pos, _) = self.state.hover?;
            let bounds = self
                .entries
                .iter()
                .filter(|e| e.position == pos)
                .map(|e| e.layout.bounds())
                .reduce(|a, b| a.union(&b))?;
            cursor.is_over(bounds).then_some(pos)
        });
        let now = match event {
            Event::Window(window::Event::RedrawRequested(at)) => *at,
            _ => Instant::now(),
        };
        if self.state.hover(hovered, now, self.default_position) {
            shell.invalidate_overlay();
            shell.request_redraw();
        }

        for entry in self.entries.iter_mut().filter(|entry| !entry.behind) {
            entry.element.update(
                entry.tree,
                event,
                entry.layout,
                cursor,
                renderer,
                shell,
                &viewport,
            );

            if can_activate(event) {
                let mut drain = marker::drain::<Dismiss>();
                entry
                    .element
                    .operate(entry.tree, entry.layout, &viewport, renderer, &mut drain);
                dismissed.extend(drain.into_signals().into_iter().map(|Dismiss(id)| id));
            }
        }

        if let Some((finger, point, contact)) = Contact::from_event(event, cursor) {
            if contact == Contact::Press && self.state.swipe.is_none() && !shell.is_event_captured()
            {
                if let Some(card) = self
                    .entries
                    .iter()
                    .find(|e| !e.behind && e.layout.bounds().contains(point))
                {
                    self.state.swipe = Some(Swipe {
                        id: card.id,
                        finger,
                        position: card.position,
                        start: point,
                        offset: Vector::ZERO,
                    });
                    shell.capture_event();
                }
            } else if let Some(swipe) = &mut self.state.swipe
                && swipe.finger == finger
            {
                swipe.offset = swipe.offset(point);
                if matches!(contact, Contact::Lost | Contact::Release) {
                    if contact == Contact::Release
                        && swipe.offset.x.abs().max(swipe.offset.y.abs()) > 45.
                    {
                        dismissed.push(swipe.id);
                    }
                    self.state.swipe = None;
                }
                shell.capture_event();
                shell.invalidate_overlay();
                shell.request_redraw();
            }
        }

        if !dismissed.is_empty() {
            self.state.live.retain(|live| !dismissed.contains(&live.id));
            if self.state.live.is_empty() {
                self.state.hover = None;
            }
            shell.invalidate_widgets();
        }
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        let viewport = Rectangle::with_size(self.window);

        self.entries
            .iter()
            .filter(|entry| !entry.behind)
            .map(|entry| {
                entry.element.mouse_interaction(
                    entry.tree,
                    entry.layout,
                    cursor,
                    &viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Inbox, Instant, Kind, Position, State};
    use crate::core::widget::Id;
    use crate::toast::toast;
    use std::time::Duration;

    #[test]
    fn only_the_hovered_stack_resumes_with_its_remaining_time() {
        let mut state: State<()> = State {
            live: Vec::new(),
            inbox: Inbox(Vec::new()),
            next_id: 0,
            hover: None,
            swipe: None,
        };
        let now = Instant::now();
        let second = Duration::from_secs(1);
        let default = Position::TopCenter;
        state.raise(toast("Hovered"));
        state.raise(toast("Elsewhere").position(Position::BottomLeft));
        for live in &mut state.live {
            live.shown_at = Some(now);
        }
        state.hover(Some(default), now + second, default);
        state.raise(toast("New while hovering"));
        state.live[2].shown_at = Some(now + second * 3);
        state.hover(None, now + second * 8, default);
        let deadlines: Vec<_> = state
            .live
            .iter()
            .map(|live| live.expires_at(second * 4))
            .collect();
        assert_eq!(
            deadlines,
            [
                Some(now + second * 11),
                Some(now + second * 4),
                Some(now + second * 12)
            ]
        );
    }

    #[test]
    fn resolving_a_promise_replaces_only_its_own_loading_card() {
        let mut state: State<()> = State {
            live: Vec::new(),
            inbox: Inbox(Vec::new()),
            next_id: 0,
            hover: None,
            swipe: None,
        };
        let first = Id::unique();
        let second = Id::unique();
        let mut a = toast("First").kind(Kind::Loading);
        a.key = Some(first.clone());
        let mut b = toast("Second").kind(Kind::Loading);
        b.key = Some(second);
        state.raise(a);
        state.raise(b);
        let now = Instant::now();
        state.live[0].shown_at = Some(now);
        assert_eq!(state.live[0].expires_at(Duration::ZERO), None);
        let mut failure = toast("Failed").kind(Kind::Error);
        failure.key = Some(first);
        state.raise(failure);
        assert_eq!(state.live.len(), 2);
        assert_eq!(state.live[0].id, 0);
        assert_eq!(state.live[0].toast.title, "Failed");
        assert_eq!(state.live[0].shown_at, None);
        assert_eq!(state.live[1].toast.kind, Kind::Loading);
        state.live[0].shown_at = Some(now);
        assert_eq!(
            state.live[0].expires_at(Duration::from_secs(4)),
            Some(now + Duration::from_secs(4))
        );
    }
}
