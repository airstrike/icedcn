use std::collections::{HashMap, HashSet};

use super::{Command, Position, Reader, Target};
use crate::scrollable::widget::{
    animate_transcript_offset, set_transcript_offset, transcript_offset,
};
use iced_core::keyboard::{Key, key::Named};
use iced_core::widget::operation::Focusable;
use iced_core::widget::{Meta, Operation, Tree, tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, keyboard, layout,
    mouse, overlay, renderer, touch,
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
impl Row {
    fn offset(&self, inset: f32) -> f32 {
        (self.y - inset).max(0.0)
    }
}
impl Target {
    fn offset(self, rows: &[Row], end: f32, margin: f32) -> Option<f32> {
        match self {
            Target::Start => Some(0.0),
            Target::End => Some(end),
            Target::Item(id) => rows.iter().find(|r| r.id == id).map(|r| r.offset(margin)),
        }
    }
}
#[derive(Default)]
struct State {
    keys: Vec<u64>,
    rows: Vec<Row>,
    height: f32,
    total: f32,
    last: Option<Command>,
    reported: Option<Reader>,
    measured_at: Option<(f32, f32)>,
    focused: bool,
    released: bool,
    // Item jumps and automatic turn placement reserve different top insets.
    pin: Option<(u64, f32)>,
    smooth: Option<Target>,
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
        let first = self
            .rows
            .partition_point(|r| r.y + r.height <= offset + margin);
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
        let keys = &mut tree.state.downcast_mut::<State>().keys;
        if keys.iter().copied().ne(self.ids.iter().map(|&(id, _)| id)) {
            if let Some(column) = tree
                .children
                .first_mut()
                .and_then(|scroll| scroll.children.first_mut())
            {
                // Move mounted widget state by message key before the column diffs its rows.
                let mut old: HashMap<_, _> =
                    keys.drain(..).zip(column.children.drain(..)).collect();
                column.children.extend(
                    self.ids
                        .iter()
                        .map(|(id, _)| old.remove(id).unwrap_or_else(Tree::empty)),
                );
            }
            keys.clear();
            keys.extend(self.ids.iter().map(|&(id, _)| id));
        }
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
        let natural = content.size.height;
        let end = (natural - view).max(0.0);
        let inset = self.options.margin + self.options.peek;
        let previous_first = state.rows.first().map(|r| r.id);
        let first_index = previous_first.and_then(|id| rows.iter().position(|r| r.id == id));
        let previous_last = state.rows.last().map(|r| r.id);
        let last_index = previous_last.and_then(|id| rows.iter().position(|r| r.id == id));
        let appended = last_index.map_or(&[][..], |i| &rows[i + 1..]);
        let mut new_anchors = appended.iter().filter(|row| row.anchor);
        let new_anchor = new_anchors.next().or_else(|| {
            if rows.len() != state.rows.len() {
                return None;
            }
            // A mounted placeholder can acquire a new key and become a turn.
            rows.iter().zip(&state.rows).find_map(|(row, old)| {
                if !row.anchor || (row.id == old.id && old.anchor) {
                    return None;
                }
                let newly_anchored = row.id == old.id
                    || !state
                        .rows
                        .iter()
                        .any(|before| before.id == row.id && before.anchor);
                newly_anchored.then_some(row)
            })
        });
        let prepended = first_index.is_some_and(|i| i > 0);
        let following = self.options.follow && !state.released && was_at_end;
        let fresh = !rows.is_empty()
            && (state.rows.is_empty()
                || first_index.is_none() && last_index.is_none() && {
                    let ids = rows.iter().map(|row| row.id).collect::<HashSet<_>>();
                    !state.rows.iter().any(|old| ids.contains(&old.id))
                });
        let old_visible = state
            .rows
            .get(state.rows.partition_point(|r| r.y + r.height <= old_y));
        let preserve_visible = || {
            let old = old_visible?;
            let current = rows.iter().find(|r| r.id == old.id)?;
            (current.y != old.y).then_some(old_y + current.y - old.y)
        };
        let requested = |dest: Target| dest.offset(&rows, end, self.options.margin);
        let geometry_changed = state.rows != rows || state.height != view;
        let mut target = None;
        let mut smooth = false;
        if fresh {
            state.pin = None;
            state.smooth = None;
            state.released = false;
            state.last = None;
            target = Some(match self.options.opening {
                Position::Start => 0.0,
                Position::End => end,
                Position::LastAnchor => rows
                    .iter()
                    .rfind(|r| r.anchor)
                    .filter(|r| natural - r.y > view)
                    .map_or(end, |r| r.offset(inset)),
            });
        } else if prepended && self.options.preserve {
            // Restore the first visible key even if rows above it also rewrapped.
            if state.smooth.is_none() {
                target = preserve_visible();
            }
        } else if let Some(row) = new_anchor {
            state.smooth = None;
            state.released = false;
            if following && new_anchors.next().is_some() {
                state.pin = None;
                target = Some(end);
            } else {
                state.pin = Some((row.id, inset));
                target = Some(row.offset(inset));
            }
        } else if following
            && (natural > state.total || view != state.height && state.pin.is_none())
        {
            target = Some(end);
        } else if !prepended {
            target = preserve_visible();
        }
        if let Some(cmd) = self.command
            && state.last != Some(cmd)
            && !rows.is_empty()
        {
            let requested = requested(cmd.target);
            // A request may precede the transcript. Once mounted, a missing
            // key is unavailable: a later unrelated append must not steal the view.
            state.last = Some(cmd);
            if let Some(y) = requested {
                state.pin = match cmd.target {
                    Target::Item(id) => Some((id, self.options.margin)),
                    _ => None,
                };
                state.released = cmd.target != Target::End;
                target = Some(y);
                smooth = self.options.smooth_commands;
                state.smooth = smooth.then_some(cmd.target);
            }
        }
        if geometry_changed && let Some(dest) = state.smooth {
            // A prepend or streaming chunk can arrive before a smooth jump settles.
            target = requested(dest);
            smooth = target.is_some();
        }
        let extra = state
            .pin
            .and_then(|(id, inset)| {
                rows.iter()
                    .find(|r| r.id == id)
                    .map(|r| r.offset(inset) + view)
            })
            .unwrap_or(0.0);
        content.size.height = natural.max(extra);
        if geometry_changed || state.total != content.size.height {
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
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();
        let inside = c.is_over(bounds) && c.is_over(*v);
        let pointer_intent = match event {
            Event::Mouse(
                mouse::Event::ButtonPressed(mouse::Button::Left)
                | mouse::Event::WheelScrolled { .. },
            ) => inside,
            Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                bounds.contains(*position) && v.contains(*position)
            }
            _ => false,
        };
        if pointer_intent {
            if state.smooth.take().is_some() {
                set_transcript_offset(t, before);
            }
            state.released = true;
        }
        self.child.update(t, event, l, c, r, shell, v);
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            state.focused = inside && !shell.is_event_captured();
        }
        if state.focused
            && !shell.is_event_captured()
            && let Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key),
                modifiers,
                ..
            }) = event
        {
            let end = (state.total - state.height).max(0.0);
            let target = match key {
                Named::Home => Some(0.0),
                Named::End => Some(end),
                Named::PageUp => Some(before - state.height),
                Named::PageDown => Some(before + state.height),
                Named::Space if modifiers.shift() => Some(before - state.height),
                Named::Space => Some(before + state.height),
                Named::ArrowUp => Some(before - 40.0),
                Named::ArrowDown => Some(before + 40.0),
                _ => None,
            };
            if let Some(y) = target {
                state.smooth = None;
                let y = y.clamp(0.0, end);
                set_transcript_offset(t, y);
                state.released = y < end - 1.0;
                shell.capture_event();
                shell.request_redraw();
            }
        }
        let after = transcript_offset(t);
        if let Some(dest) = state.smooth {
            let max = (state.total - state.height).max(0.0);
            let target = dest.offset(&state.rows, max, self.options.margin);
            if target.is_none_or(|y| (y - after).abs() <= 1.0) {
                state.smooth = None;
            }
        } else if (after - before).abs() > 0.1 {
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
