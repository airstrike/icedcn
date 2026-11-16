//! The widget behind [`Textarea`](super::Textarea): iced's text editor over
//! a [`Content`] kept in the widget's own tree, so the textarea can be
//! uncontrolled like an Input.
//!
//! The editor publishes its edit actions into a local shell; the textarea
//! performs them on its content and forwards everything else the editor
//! asked of the shell (capture, redraws, layout, IME, clipboard).

use super::Textarea;
use crate::core::Renderer as _;
use crate::core::{Border, Color, InputMethod, Point, Vector, clipboard, overlay, touch, window};
use crate::edit::{self, Command};
use crate::operation::focus;
use iced_widget::Renderer;
use iced_widget::text_editor::{Action, Content, Edit, Motion, Status, TextEditor};
use icedcn_core::Theme;

use crate::core::keyboard;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::shell::Bus;
use crate::core::text::parser::PlainText;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::{Event, Layout, Length, Padding, Rectangle, Shell, Size, Widget};
use crate::operation::control::Editable;

impl<Message> Textarea<'_, Message> {
    /// The editor over `content`, built anew for every call: it borrows the
    /// content from the tree.
    fn editor<'b>(
        &'b self,
        state: &'b State,
    ) -> TextEditor<'b, PlainText, Action, Theme, Renderer> {
        let editor = TextEditor::new(&state.content)
            .placeholder(self.placeholder.as_str())
            .size(self.text_size)
            .line_height(if self.text_size >= 16. {
                1.5
            } else {
                20. / 14.
            })
            .padding(PADDING)
            .height(
                state
                    .height
                    .filter(|_| self.resizable)
                    .map_or(Length::Fit.min(self.min_height), |h| {
                        Length::Fixed(h.max(self.min_height))
                    }),
            );

        let editor = if let Some(font) = self.font {
            editor.font(font)
        } else {
            editor
        };

        if self.disabled {
            editor
        } else {
            editor.on_action(|action| action)
        }
    }
}

impl<Message> Textarea<'_, Message> {
    /// Whether edits change the text: enabled, and uncontrolled or listening.
    fn is_editable(&self) -> bool {
        !self.disabled && (self.value.is_none() || self.on_input.is_some())
    }

    fn available(&self, state: &State) -> edit::Available {
        let selected = state.content.selection().is_some();

        edit::Available {
            undo: Some(self.is_editable()),
            redo: None,
            cut: selected && self.is_editable(),
            copy: selected,
            paste: self.is_editable(),
            select_all: !state.content.is_empty(),
        }
    }

    /// Run `command`: the clipboard through the shell, and the edit it makes
    /// as the editor's own action.
    fn command(
        &self,
        state: &State,
        command: Command,
        shell: &mut Shell<'_, Message>,
    ) -> Option<Action> {
        let editable = self.is_editable();

        match command {
            Command::Copy | Command::Cut => {
                let text = state.content.selection()?;
                if command == Command::Cut && !editable {
                    return None;
                }
                shell.write_clipboard(clipboard::Content::Text(text));
                (command == Command::Cut).then_some(Action::Edit(Edit::Backspace))
            }
            // The focused editor pastes the text it reads.
            Command::Paste => {
                if editable {
                    shell.read_clipboard(clipboard::Kind::Text);
                }
                None
            }
            Command::SelectAll => Some(Action::SelectAll),
            Command::Undo => editable.then_some(Action::Edit(Edit::Undo)),
            Command::Redo => editable.then_some(Action::Edit(Edit::Redo)),
        }
    }

    /// Perform the editor's actions on the content and publish an edit.
    fn perform(
        &self,
        state: &mut State,
        actions: impl IntoIterator<Item = Action>,
        shell: &mut Shell<'_, Message>,
    ) {
        let mut edited = false;

        for action in actions {
            if action.is_edit() && self.value.is_some() && self.on_input.is_none() {
                continue;
            }
            // An undo or redo with nothing to restore changes nothing.
            let before = matches!(action, Action::Edit(Edit::Undo | Edit::Redo))
                .then(|| state.content.text());
            let is_edit = action.is_edit();
            state.content.perform(action);
            edited |= is_edit && before.is_none_or(|before| before != state.content.text());
        }

        if edited {
            if let Some(on_input) = &self.on_input {
                shell.publish(on_input(state.content.text()));
            }
            shell.invalidate_layout();
            shell.request_redraw();
        }
    }
}

impl<Message> Meta for Textarea<'_, Message> {}

impl<Message> Widget<Message, Theme, Renderer> for Textarea<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let text = self.value.as_deref().unwrap_or(&self.default_value);

        tree::State::new(State {
            content: Content::with_text(text),
            synced: self.value.clone(),
            focused: false,
            hovered: false,
            height: None,
            resize: None,
            caret: Point::ORIGIN,
            menu: edit::Menu::default(),
            autoscroll: edit::Autoscroll::default(),
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();

        let _ = sync(&mut state.content, &mut state.synced, self.value.as_ref());

        if self.disabled && state.focused {
            children.clear();
            state.focused = false;
        }
        if self.disabled {
            state.menu = edit::Menu::default();
        }
        let mut editor = self.editor(state);

        if children.is_empty() {
            children.push(Tree::new(&editor));
        }

        children[0].diff(&mut editor);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fit)
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let Tree {
            state,
            children,
            size,
            ..
        } = tree;
        let state = state.downcast_ref::<State>();

        self.editor(state)
            .layout(&mut children[0], renderer, limits);
        *size = children[0].size;
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
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();
        let hovered = cursor.is_over(bounds);
        if state.hovered != hovered {
            state.hovered = hovered;
            shell.request_redraw();
        }
        if let Some(command) = state.menu.take() {
            let action = self.command(state, command, shell);
            self.perform(state, action, shell);
            shell.capture_event();
            return;
        }
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) if !self.disabled => {
                let press = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
                let Some(position) = cursor
                    .position_over(bounds)
                    .filter(|position| viewport.contains(*position))
                else {
                    // A press elsewhere takes the focus away, as a left press does.
                    if state.focused {
                        let cursor = mouse::Cursor::Unavailable;
                        self.update(tree, &press, layout, cursor, renderer, shell, viewport);
                    }
                    return;
                };
                // Focus and put the caret under the pointer, unless the press
                // lands inside the selection.
                let before = state.content.cursor();
                let release = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
                for event in [press, release] {
                    self.update(tree, &event, layout, cursor, renderer, shell, viewport);
                }
                let state = tree.state.downcast_mut::<State>();
                if edit::is_inside(before, state.content.cursor().position) {
                    state.content.move_to(before);
                }
                state
                    .menu
                    .open(position - Vector::new(bounds.x, bounds.y), false, shell);
                shell.capture_event();
                return;
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                physical_key,
                modifiers,
                ..
            }) if state.focused && !self.disabled => {
                if edit::opens_menu(key, *modifiers) {
                    state.menu.open(state.caret, true, shell);
                    shell.capture_event();
                    return;
                }
                if let Some(command) = Command::from_key(key, *physical_key, *modifiers) {
                    let action = self.command(state, command, shell);
                    self.perform(state, action, shell);
                    shell.capture_event();
                    return;
                }
            }
            _ => {}
        }
        if self.disabled
            || !self.resizable
            || matches!(event, Event::Window(window::Event::Unfocused))
        {
            state.resize = None;
        } else if let Some(gesture) = Gesture::from_event(event, cursor) {
            match gesture {
                Gesture::Start { pointer, hit, y }
                    if state.resize.is_none()
                        && grip(bounds).contains(hit)
                        && viewport.contains(hit) =>
                {
                    state.resize = Some((pointer, bounds.height - y));
                    shell.capture_event();
                    return;
                }
                Gesture::Move(pointer, position)
                    if let Some((owner, offset)) = state.resize
                        && owner == pointer =>
                {
                    state.height = Some((position.y + offset).max(self.min_height));
                    shell.invalidate_layout();
                    shell.request_redraw();
                    shell.capture_event();
                    return;
                }
                Gesture::End(pointer)
                    if state.resize.is_some_and(|(owner, _)| owner == pointer) =>
                {
                    state.resize = None;
                    shell.capture_event();
                    return;
                }
                _ => {}
            }
        }
        let mut bus = Bus::new();

        {
            let Tree {
                state, children, ..
            } = &mut *tree;
            let state = state.downcast_mut::<State>();
            let (child_layout, child_tree) = layout
                .iter_mut(children)
                .next()
                .expect("a textarea has one child");

            let mut local = shell.local(&mut bus);
            let cursor = if self.disabled {
                cursor
            } else {
                state.autoscroll.track(event, cursor, bounds, viewport)
            };

            self.editor(state).update(
                child_tree,
                event,
                child_layout,
                cursor,
                renderer,
                &mut local,
                viewport,
            );

            let focused = focus::count(|op| {
                self.editor(state)
                    .operate(child_tree, child_layout, viewport, renderer, op)
            })
            .focused
            .is_some();
            if state.focused != focused {
                state.focused = focused;
                shell.request_redraw();
            }

            if local.is_event_captured() {
                shell.capture_event();
            }

            if let InputMethod::Enabled { cursor: caret, .. } = local.input_method() {
                state.caret = Point::new(caret.x - bounds.x, caret.y + caret.height - bounds.y);
            }
            shell.request_redraw_at(local.redraw_request());
            shell.invalidate(local.invalidation());
            shell.request_input_method(local.input_method());
            shell.clipboard_mut().merge(local.clipboard_mut());
        }

        let state = tree.state.downcast_mut::<State>();
        self.perform(state, bus.drain().map(|(action, _)| action), shell);

        // The editor only hits the lines it shows: past its top or bottom, a
        // frame moves the selection a line further, which scrolls it, and
        // drags to the pointer's column on the line that comes into view.
        let text = bounds.shrink(PADDING);
        let visible = viewport.intersection(&text).unwrap_or(text);
        if let Some(pointer) = state.autoscroll.tick(event, visible, viewport, shell) {
            let motion = if pointer.y < visible.y {
                Some(Motion::Up)
            } else if pointer.y > visible.y + visible.height {
                Some(Motion::Down)
            } else {
                None
            };
            self.perform(state, motion.map(Action::Select), shell);
            let held = Point::new(
                pointer.x.clamp(visible.x, visible.x + visible.width),
                pointer
                    .y
                    .clamp(visible.y + 1.0, visible.y + visible.height - 1.0),
            );
            let drag = Event::Mouse(mouse::Event::CursorMoved { position: held });
            let cursor = mouse::Cursor::Available(held);
            self.update(tree, &drag, layout, cursor, renderer, shell, viewport);
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
        let state = tree.state.downcast_ref::<State>();
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a textarea has one child");

        let is_hovered = cursor.is_over(layout.bounds());
        let status = if self.disabled {
            Status::Disabled
        } else if state.focused {
            Status::Focused { is_hovered }
        } else if is_hovered {
            Status::Hovered
        } else {
            Status::Active
        };
        let (surface, ring) = self.style.resolve(theme, status);
        if let Some(ring) = ring {
            let r = ring.width;
            let mut radius = surface.border.radius;
            radius.top_left += r;
            radius.top_right += r;
            radius.bottom_left += r;
            radius.bottom_right += r;
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds().expand(r),
                    border: Border {
                        radius,
                        width: r,
                        color: ring.color.into(),
                    },
                    ..Default::default()
                },
                Color::TRANSPARENT,
            );
        }
        self.editor(state).style(move |_, _| surface).draw(
            child_tree,
            renderer,
            theme,
            style,
            child_layout,
            cursor,
            viewport,
        );
        if self.resizable {
            let corner = grip(layout.bounds());
            let color = surface
                .placeholder
                .scale_alpha(if self.disabled { 0.45 } else { 0.85 });
            for n in [3, 6] {
                for i in 0..n {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: corner.x + corner.width - 3. - i as f32,
                                y: corner.y + corner.height - 2. - (n - i) as f32,
                                width: 1.,
                                height: 1.,
                            },
                            ..Default::default()
                        },
                        color,
                    );
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.disabled {
            return if cursor.is_over(layout.bounds()) {
                mouse::Interaction::NotAllowed
            } else {
                mouse::Interaction::default()
            };
        }

        let state = tree.state.downcast_ref::<State>();
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a textarea has one child");

        if self.resizable && (state.resize.is_some() || cursor.is_over(grip(layout.bounds()))) {
            return mouse::Interaction::ResizingVertically;
        }
        self.editor(state)
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
        let state = state.downcast_mut::<State>();
        let (child_layout, child_tree) = layout
            .iter_mut(children)
            .next()
            .expect("a textarea has one child");

        if self.disabled {
            return;
        }
        operation.custom(None, layout.bounds(), &mut Editable);
        self.editor(state)
            .operate(child_tree, child_layout, viewport, renderer, operation);
        state.focused = focus::count(|op| {
            self.editor(state)
                .operate(child_tree, child_layout, viewport, renderer, op)
        })
        .focused
        .is_some();
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        _viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<State>();
        let available = self.available(state);

        state
            .menu
            .overlay(available, layout.bounds(), translation, renderer, window)
            .into_iter()
            .collect()
    }
}

/// Replace `content` with a controlled `value` that changed since `synced`, keeping
/// the caret where it was. Returns whether the content was replaced.
///
/// iced can rebuild the view before the app handles an edit, so a value seen
/// before is stale and must not replace the edited content.
pub(crate) fn sync(
    content: &mut Content<Renderer>,
    synced: &mut Option<String>,
    value: Option<&String>,
) -> bool {
    let Some(value) = value else {
        return false;
    };
    if synced.as_ref() == Some(value) {
        return false;
    }
    synced.replace(value.clone());
    if content.text() == *value {
        return false;
    }
    let mut cursor = content.cursor();
    *content = Content::with_text(value);
    for position in std::iter::once(&mut cursor.position).chain(cursor.selection.iter_mut()) {
        position.line = position.line.min(content.line_count().saturating_sub(1));
        if let Some(line) = content.line(position.line) {
            position.index = line
                .text
                .floor_char_boundary(position.index.min(line.text.len()));
        }
    }
    content.move_to(cursor);
    true
}

/// The space between the textarea's border and its text.
const PADDING: Padding = Padding {
    top: 9.0,
    right: 11.0,
    bottom: 9.0,
    left: 11.0,
};

struct State {
    content: Content<Renderer>,
    /// The controlled value the content last synced to; see [`sync`].
    synced: Option<String>,
    focused: bool,
    hovered: bool,
    height: Option<f32>,
    resize: Option<(Option<touch::Finger>, f32)>,
    /// Below the caret, relative to the textarea: where Shift+F10 opens the menu.
    caret: Point,
    menu: edit::Menu,
    autoscroll: edit::Autoscroll,
}

fn grip(bounds: Rectangle) -> Rectangle {
    Rectangle {
        x: bounds.x + bounds.width - 12.,
        y: bounds.y + bounds.height - 12.,
        width: 12.,
        height: 12.,
    }
}

// A mouse drag has one pointer; touch releases must match the initiating finger.
enum Gesture {
    Start {
        pointer: Option<touch::Finger>,
        hit: Point,
        y: f32,
    },
    Move(Option<touch::Finger>, Point),
    End(Option<touch::Finger>),
}

impl Gesture {
    fn from_event(event: &Event, cursor: mouse::Cursor) -> Option<Self> {
        use mouse::Button::Left;
        use mouse::Event::{ButtonPressed, ButtonReleased, CursorMoved};
        use touch::Event::{FingerLifted, FingerLost, FingerMoved, FingerPressed};
        match *event {
            // The scroller translates the cursor into child coordinates, while
            // event positions stay in the window. Keep each drag in one space.
            Event::Mouse(ButtonPressed(Left)) => {
                let hit = cursor.position()?;
                Some(Self::Start {
                    pointer: None,
                    hit,
                    y: hit.y,
                })
            }
            Event::Mouse(CursorMoved { .. }) => {
                Some(Self::Move(None, cursor.observe().position()?))
            }
            Event::Mouse(ButtonReleased(Left)) => Some(Self::End(None)),
            Event::Touch(FingerPressed { id, position, .. }) => Some(Self::Start {
                pointer: Some(id),
                hit: cursor.position()?,
                y: position.y,
            }),
            Event::Touch(FingerMoved { id, position, .. }) => Some(Self::Move(Some(id), position)),
            Event::Touch(FingerLifted { id, .. } | FingerLost { id, .. }) => {
                Some(Self::End(Some(id)))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::textarea::textarea;
    use iced_core::keyboard::key::Named;
    use iced_core::renderer::Headless;
    use iced_core::shell::Waker;
    use iced_core::time::Instant;
    use iced_test::futures::futures::executor::block_on;
    use iced_test::simulator;

    #[test]
    fn focus_and_escape_blur_redraw_without_editing_or_moving_the_pointer() {
        let renderer =
            block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
        let mut textarea: Textarea<'_, ()> = textarea("").resizable(false);
        let mut tree = Tree::new(&textarea);
        tree.diff(&mut textarea);
        textarea.layout(
            &mut tree,
            &renderer,
            &layout::Limits::new(Size::ZERO, Size::new(200.0, 100.0)),
        );
        let layout = Layout::new(tree.size);
        let position = Point::new(12.0, 10.0);
        let cursor = tree.state.downcast_ref::<State>().content.cursor();
        let events = [
            (
                Event::Window(window::Event::RedrawRequested(Instant::now())),
                false,
            ),
            (
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
                true,
            ),
            (simulator::press_key(Named::Escape, None), false),
        ];

        for (index, (event, focused)) in events.into_iter().enumerate() {
            let mut bus = Bus::new();
            let mut shell = Shell::new(&window::Headless, Waker::noop(), &mut bus);
            textarea.update(
                &mut tree,
                &event,
                layout,
                mouse::Cursor::Available(position),
                &renderer,
                &mut shell,
                &layout.bounds(),
            );
            let state = tree.state.downcast_ref::<State>();
            assert_eq!(state.focused, focused);
            assert_eq!(state.content.cursor(), cursor, "the caret did not move");
            assert!(
                shell.is_empty(),
                "focus changes publish no application message"
            );
            if index > 0 {
                assert_eq!(shell.redraw_request(), window::RedrawRequest::NextFrame);
            }
        }
    }
}
