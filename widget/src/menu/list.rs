use crate::operation::focus;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::time::{Duration, Instant};
use iced_core::widget::operation::{Focusable, Scrollable, Text};
use iced_core::widget::{
    Id, Meta, Operation,
    tree::{self, Tree},
};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

/// The keyboard focus scope of a horizontal menubar or a vertical menu panel.
pub(crate) struct List<'a, Message> {
    pub content: Element<'a, Message, Theme, Renderer>,
    /// None manages initial focus only; the input owns its editing keys.
    pub horizontal: Option<bool>,
    pub autofocus: Initial,
    pub on_move: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}
pub(crate) enum Initial {
    Menu(bool),
    Content {
        keyboard: bool,
    },
    Dropdown {
        keyboard: bool,
        selected: Option<usize>,
        scroll: Id,
    },
    Choice {
        index: usize,
        labels: Vec<String>,
        scroll: Id,
    },
}

impl From<bool> for Initial {
    fn from(autofocus: bool) -> Self {
        Self::Menu(autofocus)
    }
}

impl Initial {
    fn focus(&self, index: usize, visit: &mut impl FnMut(&mut dyn Operation)) {
        match self {
            Self::Dropdown { scroll, .. } | Self::Choice { scroll, .. } => {
                focus::reveal(index, scroll, visit)
            }
            Self::Content { keyboard } => visit(&mut focus::restore(Some(index), *keyboard)),
            Self::Menu(_) => visit(&mut focus::select(Some(index))),
        }
    }
}

impl<Message> Meta for List<'_, Message> {}
#[derive(Default)]
struct State {
    initialized: bool,
    keyboard: bool,
    search: Search,
}

#[derive(Debug, Default)]
pub(crate) struct Search {
    search: String,
    typed: Option<Instant>,
}
impl<Message> Widget<Message, Theme, Renderer> for List<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
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
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a keyboard handler has one child");

        let mut visit = |op: &mut dyn Operation| {
            self.content
                .operate(child_tree, child_layout, viewport, renderer, op)
        };
        let state = tree.state.downcast_mut::<State>();
        if !state.initialized {
            state.initialized = true;
            let index = match &self.autofocus {
                Initial::Content { .. } | Initial::Menu(true) => Some(0),
                Initial::Dropdown {
                    keyboard, selected, ..
                } => selected.or(keyboard.then_some(0)),
                Initial::Choice { index, .. } => Some(*index),
                Initial::Menu(false) => None,
            };
            if let Some(index) = index {
                state.keyboard = true;
                self.autofocus.focus(index, &mut visit);
                shell.request_redraw();
            }
        }
        let vertical = self.horizontal == Some(false);
        if vertical
            && matches!(event, Event::Mouse(mouse::Event::CursorMoved { .. }))
            && let Some(position) = cursor.position_over(child_layout.bounds())
        {
            state.keyboard = false;
            let mut rows = Rows::default();
            visit(&mut rows);
            let hovered = rows
                .items
                .iter()
                .position(|bounds| bounds.contains(position + rows.translation));
            if hovered != rows.focused {
                visit(&mut focus::restore(hovered, false));
                shell.request_redraw();
            }
        }
        if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
            let count = focus::count(&mut visit);
            if count.total > 0 && (vertical || count.focused.is_some()) {
                let last = count.total - 1;
                let next = match (key.as_ref(), self.horizontal) {
                    (Key::Named(Named::Home), Some(_)) => Some(0),
                    (Key::Named(Named::End), Some(_)) => Some(last),
                    (Key::Named(Named::ArrowUp), Some(false)) => {
                        Some(count.focused.map_or(last, |i| i.saturating_sub(1)))
                    }
                    (Key::Named(Named::ArrowDown), Some(false)) => {
                        Some(count.focused.map_or(0, |i| (i + 1).min(last)))
                    }
                    (Key::Named(Named::ArrowLeft), Some(true)) => focus::adjacent(count, true),
                    (Key::Named(Named::ArrowRight), Some(true)) => focus::adjacent(count, false),
                    (Key::Character(c), Some(false))
                        if !modifiers.command() && !modifiers.alt() && !modifiers.control() =>
                    {
                        match &self.autofocus {
                            Initial::Choice { labels, .. } => {
                                state.search.match_prefix(c, count.focused, labels)
                            }
                            Initial::Content { .. }
                            | Initial::Menu(_)
                            | Initial::Dropdown { .. } => {
                                let mut rows = Rows::default();
                                visit(&mut rows);
                                state.search.match_prefix(c, count.focused, &rows.labels)
                            }
                        }
                    }
                    _ => None,
                };
                if let Some(index) = next {
                    state.keyboard = true;
                    self.autofocus.focus(index, &mut visit);
                    if let Some(moved) = &self.on_move {
                        shell.publish(moved(index));
                    }
                    shell.request_redraw();
                    shell.capture_event();
                    return;
                }
            }
        }
        let cursor = if vertical && state.keyboard && matches!(event, Event::Window(_)) {
            mouse::Cursor::Unavailable
        } else {
            cursor
        };
        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
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
            .iter(&tree.children)
            .next()
            .expect("a keyboard handler has one child");

        let cursor =
            if self.horizontal == Some(false) && tree.state.downcast_ref::<State>().keyboard {
                mouse::Cursor::Unavailable
            } else {
                cursor
            };
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
            .iter(&tree.children)
            .next()
            .expect("a keyboard handler has one child");

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
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a keyboard handler has one child");

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
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a keyboard handler has one child");

        self.content.overlay(
            child_tree,
            child_layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}

/// Only enabled item bounds participate, including items inside composed groups.
#[derive(Default)]
struct Rows {
    translation: Vector,
    items: Vec<Rectangle>,
    labels: Vec<String>,
    focused: Option<usize>,
}

impl Operation for Rows {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        _: Rectangle,
        _: Size,
        offset: Vector,
        _: &mut dyn Scrollable,
    ) {
        self.translation = offset;
    }
    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, control: &mut dyn Focusable) {
        if control.is_focused() {
            self.focused = Some(self.items.len());
        }
        self.items.push(bounds);
        self.labels.push(String::new());
    }
    fn text(&mut self, _: Option<&Id>, bounds: Rectangle, text: &mut dyn Text) {
        if let Some(item) = self.items.last()
            && let Some(label) = self.labels.last_mut()
            && label.is_empty()
            && item.contains(bounds.center())
        {
            let content = text.text();
            if content.chars().any(char::is_alphanumeric) {
                label.push_str(&content.to_lowercase());
            }
        }
    }
}

impl Search {
    pub(crate) fn match_prefix(
        &mut self,
        key: &str,
        focused: Option<usize>,
        labels: &[String],
    ) -> Option<usize> {
        let now = Instant::now();
        if self
            .typed
            .is_none_or(|at| now.duration_since(at) > Duration::from_secs(1))
        {
            self.search.clear();
        }
        self.typed = Some(now);
        self.search.push_str(&key.to_lowercase());
        let repeated = self
            .search
            .chars()
            .all(|c| Some(c) == self.search.chars().next());
        let prefix = if repeated {
            key.to_lowercase()
        } else {
            self.search.clone()
        };
        let start = focused.map_or(0, |i| i + usize::from(repeated));
        (0..labels.len())
            .map(|i| (start + i) % labels.len())
            .find(|&i| labels[i].to_lowercase().starts_with(&prefix))
    }
}
