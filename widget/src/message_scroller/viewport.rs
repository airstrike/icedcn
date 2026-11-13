use super::{Command, Position, Reader, Target};
use crate::scrollable::widget::{
    animate_transcript_offset, set_transcript_offset, transcript_offset,
};
use iced_core::keyboard::{Key, key::Named};
use iced_core::widget::operation::Focusable;
use iced_core::widget::{Meta, Operation, Tree, tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, keyboard, layout,
    mouse, overlay, renderer,
};
use iced_widget::Renderer;
use icedxdy_core::Theme;

#[derive(Debug, Clone, Copy)]
pub(super) struct Options {
    pub opening: Position,
    pub follow: bool,
    pub preserve: bool,
    pub smooth_commands: bool,
    pub margin: f32,
    pub peek: f32,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            opening: Position::LastAnchor,
            follow: false,
            preserve: true,
            smooth_commands: false,
            margin: 16.0,
            peek: 0.0,
        }
    }
}
pub(super) struct Viewport<'a, Message> {
    pub child: Element<'a, Message, Theme, Renderer>,
    pub ids: Vec<(u64, bool)>,
    pub options: Options,
    pub command: Option<Command>,
    pub on_reader: Option<Box<dyn Fn(Reader) -> Message + 'a>>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Row {
    id: u64,
    anchor: bool,
    y: f32,
    height: f32,
}
#[derive(Default)]
struct State {
    rows: Vec<Row>,
    height: f32,
    total: f32,
    last: Option<Command>,
    reported: Option<Reader>,
    measured_at: Option<(f32, f32)>,
    focused: bool,
    released: bool,
    pin: Option<u64>,
}
impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }
    fn focus(&mut self) {
        self.focused = true;
    }
    fn unfocus(&mut self) {
        self.focused = false;
    }
}
impl State {
    fn reader(&self, offset: f32, margin: f32) -> Reader {
        let first = self.rows.partition_point(|r| r.y + r.height <= offset);
        let end = self.rows.partition_point(|r| r.y < offset + self.height);
        let top = self.rows.partition_point(|r| r.y <= offset + margin + 1.0);
        Reader {
            can_scroll_up: offset > 1.0,
            can_scroll_down: offset + self.height < self.total - 1.0,
            current_anchor: self.rows[..top]
                .iter()
                .rev()
                .find(|r| r.anchor)
                .map(|r| r.id),
            visible: self.rows[first..end.max(first)]
                .iter()
                .map(|r| r.id)
                .collect(),
        }
    }
}
impl<Message> Meta for Viewport<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Viewport<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.child));
    }
    fn size(&self) -> Size<Length> {
        self.child.size()
    }
    fn layout(&mut self, tree: &mut Tree, r: &Renderer, limits: &layout::Limits) {
        let native = &mut tree.children[0];
        let old_y = transcript_offset(native);
        let state = tree.state.downcast_mut::<State>();
        let was_at_end = old_y + state.height >= state.total - 1.0;
        self.child.layout(native, r, limits);
        tree.size = native.size;
        native.translation = Vector::ZERO;
        let content = &mut native.children[0];
        let view = native.size.height;
        let rows: Vec<_> = self
            .ids
            .iter()
            .zip(&content.children)
            .map(|(&(id, anchor), leaf)| Row {
                id,
                anchor,
                y: leaf.translation.y,
                height: leaf.size.height,
            })
            .collect();
        let anchor = rows.iter().rfind(|v| v.anchor);
        let anchor_y = |row: &Row| (row.y - self.options.margin - self.options.peek).max(0.0);
        let natural = content.size.height;
        let old_last = state.rows.last().map(|r| r.id);
        let appended = old_last
            .and_then(|id| rows.iter().position(|r| r.id == id))
            .and_then(|i| rows[i + 1..].iter().rfind(|v| v.anchor));
        let old_first = state.rows.first().map(|r| r.id);
        let prepend = old_first
            .and_then(|id| rows.iter().find(|r| r.id == id))
            .filter(|r| rows.first().is_some_and(|f| r.id != f.id));
        let fresh = !rows.is_empty()
            && (state.rows.is_empty()
                || old_last.is_some_and(|id| !rows.iter().any(|r| r.id == id))
                    && old_first.is_some_and(|id| !rows.iter().any(|r| r.id == id)));
        let mut target = None;
        let mut smooth = false;
        if fresh {
            state.pin = None;
            state.released = false;
            state.last = None;
            target = Some(match self.options.opening {
                Position::Start => 0.0,
                Position::End => (natural - view).max(0.0),
                Position::LastAnchor => anchor.map_or((natural - view).max(0.0), |v| {
                    anchor_y(v).min((natural - view).max(0.0))
                }),
            });
        } else if let Some(row) = appended {
            state.pin = Some(row.id);
            state.released = false;
            target = Some(anchor_y(row));
        } else if self.options.preserve
            && let Some(first) = prepend
            && let Some(old) = state.rows.first()
        {
            target = Some(old_y + first.y - old.y);
        } else if self.options.follow && !state.released && was_at_end && natural > state.total {
            target = Some((natural - view).max(0.0));
        } else if let Some(previous) = state
            .rows
            .get(state.rows.partition_point(|r| r.y + r.height <= old_y))
            && let Some(current) = rows.iter().find(|r| r.id == previous.id)
            && current.y != previous.y
        {
            // Wrapping, image loads and edits above a visible row should keep
            // the same part of that row at the viewport edge.
            if prepend.is_none() {
                target = Some(old_y + current.y - previous.y);
            }
        }
        if let Some(cmd) = self.command
            && state.last != Some(cmd)
            && !rows.is_empty()
        {
            let requested = match cmd.target {
                Target::Start => Some(0.0),
                Target::End => Some((natural - view).max(0.0)),
                Target::Item(id) => rows.iter().find(|r| r.id == id).map(anchor_y),
            };
            // A request may precede the transcript. Once mounted, a missing
            // key is unavailable: a later unrelated append must not steal the view.
            state.last = Some(cmd);
            if let Some(y) = requested {
                state.pin = match cmd.target {
                    Target::Item(id) => Some(id),
                    _ => None,
                };
                state.released = cmd.target != Target::End;
                target = Some(y);
                smooth = self.options.smooth_commands;
            }
        }
        let extra = state
            .pin
            .and_then(|id| rows.iter().find(|r| r.id == id))
            .map_or(0.0, |r| anchor_y(r) + view);
        content.size.height = natural.max(extra);
        if state.height != view || state.total != content.size.height || state.rows != rows {
            state.measured_at = None;
        }
        state.total = content.size.height;
        state.height = view;
        state.rows = rows;
        let max = (state.total - view).max(0.0);
        if let Some(y) = target {
            let y = y.clamp(0.0, max);
            if smooth {
                animate_transcript_offset(native, y);
            } else {
                set_transcript_offset(native, y);
            }
        } else if old_y > max {
            set_transcript_offset(native, max);
        }
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        v: &Rectangle,
        r: &Renderer,
        op: &mut dyn Operation,
    ) {
        op.focusable(None, layout.bounds(), tree.state.downcast_mut::<State>());
        if let Some((l, t)) = layout.iter_mut(&mut tree.children).next() {
            self.child.operate(t, l, v, r, op);
        }
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        c: mouse::Cursor,
        r: &Renderer,
        shell: &mut Shell<'_, Message>,
        v: &Rectangle,
    ) {
        let Some((l, t)) = layout.iter_mut(&mut tree.children).next() else {
            return;
        };
        let before = transcript_offset(t);
        self.child.update(t, event, l, c, r, shell, v);
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            state.focused = c.is_over(bounds) && c.is_over(*v) && !shell.is_event_captured();
            if state.focused {
                // A reader can select text at the live edge without losing their place.
                state.released = true;
            }
        }
        if state.focused
            && !shell.is_event_captured()
            && let Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key),
                ..
            }) = event
        {
            let end = (state.total - state.height).max(0.0);
            let target = match key {
                Named::Home => Some(0.0),
                Named::End => Some(end),
                Named::PageUp => Some(before - state.height),
                Named::PageDown => Some(before + state.height),
                Named::ArrowUp => Some(before - 40.0),
                Named::ArrowDown => Some(before + 40.0),
                _ => None,
            };
            if let Some(y) = target {
                let y = y.clamp(0.0, end);
                set_transcript_offset(t, y);
                state.released = y < end - 1.0;
                shell.capture_event();
                shell.request_redraw();
            }
        }
        let after = transcript_offset(t);
        if (after - before).abs() > 0.1 {
            state.released = after + state.height < state.total - 1.0;
        }
        let margin = self.options.margin + self.options.peek;
        if let Some(on_reader) = &self.on_reader
            && state.measured_at != Some((after, margin))
        {
            state.measured_at = Some((after, margin));
            let reader = state.reader(after, margin);
            if state.reported.as_ref() != Some(&reader) {
                state.reported = Some(reader.clone());
                shell.publish(on_reader(reader));
            }
        }
    }
    fn draw(
        &self,
        tree: &Tree,
        r: &mut Renderer,
        theme: &Theme,
        s: &renderer::Style,
        layout: Layout,
        c: mouse::Cursor,
        v: &Rectangle,
    ) {
        if let Some((l, t)) = layout.iter(&tree.children).next() {
            self.child.draw(t, r, theme, s, l, c, v);
        }
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        c: mouse::Cursor,
        v: &Rectangle,
        r: &Renderer,
    ) -> mouse::Interaction {
        layout
            .iter(&tree.children)
            .next()
            .map(|(l, t)| self.child.mouse_interaction(t, l, c, v, r))
            .unwrap_or_default()
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        l: Layout,
        r: &Renderer,
        v: &Rectangle,
        tr: Vector,
        win: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(
            std::slice::from_mut(&mut self.child),
            tree,
            l,
            r,
            v,
            tr,
            win,
        )
    }
}
