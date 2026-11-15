//! The widget behind [`Textarea`](super::Textarea): iced's text editor over
//! a [`Content`] kept in the widget's own tree, so the textarea can be
//! uncontrolled like an Input.
//!
//! The editor publishes its edit actions into a local shell; the textarea
//! performs them on its content and forwards everything else the editor
//! asked of the shell (capture, redraws, layout, IME, clipboard).

use super::Textarea;
use crate::core::Renderer as _;
use crate::core::text::Position;
use crate::core::{Border, Color, Point, touch, window};
use crate::operation::focus;
use iced_widget::Renderer;
use iced_widget::text_editor::{Action, Content, Status, TextEditor};
use icedcn_core::Theme;

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
            .padding(Padding {
                top: 9.0,
                right: 11.0,
                bottom: 9.0,
                left: 11.0,
            })
            .height(
                state
                    .height
                    .filter(|_| self.resizable)
                    .map_or(Length::Fit.min(self.min_height), |h| {
                        Length::Fixed(h.max(self.min_height))
                    }),
            );

        if self.disabled {
            editor
        } else {
            editor.on_action(|action| action)
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
            focused: false,
            hovered: false,
            height: None,
            resize: None,
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();

        if let Some(value) = &self.value
            && state.content.text() != *value
        {
            let mut cursor = state.content.cursor();
            state.content = Content::with_text(value);
            let clamp = |mut p: Position| {
                p.line = p.line.min(state.content.line_count().saturating_sub(1));
                if let Some(line) = state.content.line(p.line) {
                    p.index = line.text.floor_char_boundary(p.index.min(line.text.len()));
                }
                p
            };
            cursor.position = clamp(cursor.position);
            cursor.selection = cursor.selection.map(clamp);
            state.content.move_to(cursor);
        }

        if self.disabled && state.focused {
            children.clear();
            state.focused = false;
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

            self.editor(state).update(
                child_tree,
                event,
                child_layout,
                cursor,
                renderer,
                &mut local,
                viewport,
            );

            state.focused = focus::count(|op| {
                self.editor(state)
                    .operate(child_tree, child_layout, viewport, renderer, op)
            })
            .focused
            .is_some();

            if local.is_event_captured() {
                shell.capture_event();
            }

            shell.request_redraw_at(local.redraw_request());
            shell.invalidate(local.invalidation());
            shell.request_input_method(local.input_method());
            shell.clipboard_mut().merge(local.clipboard_mut());
        }

        let state = tree.state.downcast_mut::<State>();
        let mut edited = false;

        for (action, _receipt) in bus.drain() {
            if action.is_edit() && self.value.is_some() && self.on_input.is_none() {
                continue;
            }
            edited |= action.is_edit();
            state.content.perform(action);
        }

        if edited {
            if let Some(on_input) = &self.on_input {
                shell.publish(on_input(state.content.text()));
            }
            shell.invalidate_layout();
            shell.request_redraw();
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
}

struct State {
    content: Content<Renderer>,
    focused: bool,
    hovered: bool,
    height: Option<f32>,
    resize: Option<(Option<touch::Finger>, f32)>,
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
            Event::Touch(FingerPressed { id, position }) => Some(Self::Start {
                pointer: Some(id),
                hit: cursor.position()?,
                y: position.y,
            }),
            Event::Touch(FingerMoved { id, position }) => Some(Self::Move(Some(id), position)),
            Event::Touch(FingerLifted { id, .. } | FingerLost { id, .. }) => {
                Some(Self::End(Some(id)))
            }
            _ => None,
        }
    }
}
