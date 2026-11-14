//! Two panels separated by a draggable, keyboard-operable divider. Nest groups
//! to build a pane arrangement with independent axes.
use crate::compose::{Compose, Inherited, Slot};
use iced_core::border;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::widget::operation::Focusable;
use iced_core::widget::{Meta, Operation, Tree, tree};
use iced_core::{
    Element, Event, Layout, Length, Point, Rectangle, Renderer as _, Shell, Size, Vector, Widget,
    layout, mouse, overlay, renderer, touch,
};
use iced_widget::Renderer;
use icedcn_core::{Orientation, Theme};

pub struct Resizable<'a, Message> {
    panels: [Slot<'a, Message>; 2],
    axis: Orientation,
    initial: f32,
    value: Option<f32>,
    change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    handle: bool,
    width: Length,
    height: Length,
}

pub fn resizable<'a, Message: 'a>(
    first: impl Into<Slot<'a, Message>>,
    second: impl Into<Slot<'a, Message>>,
) -> Resizable<'a, Message> {
    Resizable {
        panels: [first.into(), second.into()],
        axis: Orientation::Horizontal,
        initial: 0.5,
        value: None,
        change: None,
        handle: false,
        width: Length::Fill,
        height: Length::Fill,
    }
}

fn fraction(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.5
    }
}

impl<'a, Message> Resizable<'a, Message> {
    pub fn orientation(mut self, axis: Orientation) -> Self {
        self.axis = axis;
        self
    }
    /// Initial fraction of available space assigned to the first panel.
    pub fn default_value(mut self, value: f32) -> Self {
        self.initial = fraction(value);
        self
    }
    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(fraction(value));
        self
    }
    pub fn on_value_change(mut self, callback: impl Fn(f32) -> Message + 'a) -> Self {
        self.change = Some(Box::new(callback));
        self
    }
    pub fn with_handle(mut self) -> Self {
        self.handle = true;
        self
    }
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Resizable<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        for panel in &mut self.panels {
            panel.inherit(from);
        }
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Resizable {
            panels,
            axis,
            initial,
            value,
            change,
            handle,
            width,
            height,
        } = *self;
        Panes {
            children: panels.map(Slot::compose),
            axis,
            initial,
            value,
            change,
            handle,
            width,
            height,
        }
        ._boxed()
    }
}
impl<'a, Message: 'a> From<Resizable<'a, Message>> for Slot<'a, Message> {
    fn from(group: Resizable<'a, Message>) -> Self {
        Slot::new(group)
    }
}
impl<'a, Message: 'a> From<Resizable<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(group: Resizable<'a, Message>) -> Self {
        Box::new(group).compose()
    }
}

struct Panes<'a, Message> {
    children: [Element<'a, Message, Theme, Renderer>; 2],
    axis: Orientation,
    initial: f32,
    value: Option<f32>,
    change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    handle: bool,
    width: Length,
    height: Length,
}
#[derive(Clone, Copy, PartialEq)]
enum Pointer {
    Mouse,
    Touch(touch::Finger),
}
#[derive(Default)]
struct State {
    value: Option<f32>,
    dragging: Option<Pointer>,
    focus: bool,
}
impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focus
    }
    fn focus(&mut self) {
        self.focus = true;
    }
    fn unfocus(&mut self) {
        self.focus = false;
    }
}
impl<Message> Panes<'_, Message> {
    fn current(&self, state: &State) -> f32 {
        self.value.or(state.value).unwrap_or(self.initial)
    }
    fn separator(&self, bounds: Rectangle, value: f32, thickness: f32) -> Rectangle {
        match self.axis {
            Orientation::Horizontal => Rectangle {
                x: bounds.x + (bounds.width - 1.0).max(0.0) * value + (1.0 - thickness) / 2.0,
                width: thickness,
                ..bounds
            },
            Orientation::Vertical => Rectangle {
                y: bounds.y + (bounds.height - 1.0).max(0.0) * value + (1.0 - thickness) / 2.0,
                height: thickness,
                ..bounds
            },
        }
    }
    fn at(&self, bounds: Rectangle, point: Point) -> f32 {
        match self.axis {
            Orientation::Horizontal => {
                fraction((point.x - bounds.x) / (bounds.width - 1.0).max(1.0))
            }
            Orientation::Vertical => {
                fraction((point.y - bounds.y) / (bounds.height - 1.0).max(1.0))
            }
        }
    }
}
impl<Message> Meta for Panes<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Panes<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.children);
    }
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let size = limits.resolve(self.width, self.height, Size::ZERO);
        let value = self.current(tree.state.downcast_ref::<State>());
        let area = Rectangle::with_size(size);
        let divider = self.separator(area, value, 1.0);
        let (first, second, offset) = match self.axis {
            Orientation::Horizontal => (
                Size::new(divider.x, size.height),
                Size::new((size.width - divider.x - 1.0).max(0.0), size.height),
                Vector::new(divider.x + 1.0, 0.0),
            ),
            Orientation::Vertical => (
                Size::new(size.width, divider.y),
                Size::new(size.width, (size.height - divider.y - 1.0).max(0.0)),
                Vector::new(0.0, divider.y + 1.0),
            ),
        };
        for ((panel, state), (size, position)) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip([(first, Vector::ZERO), (second, offset)])
        {
            panel.layout(state, renderer, &layout::Limits::new(size, size));
            state.translation = position;
        }
        tree.size = size;
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let divider = self.separator(layout.bounds(), self.current(state), 10.0);
        operation.focusable(None, divider, state);
        for (panel, (layout, state)) in self
            .children
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            panel.operate(state, layout, viewport, renderer, operation);
        }
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
        let value = self.current(state);
        let bounds = layout.bounds();
        let hit = self.separator(bounds, value, 10.0);
        let press = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                cursor.position().map(|p| (Pointer::Mouse, p))
            }
            Event::Touch(touch::Event::FingerPressed { id, position }) => {
                Some((Pointer::Touch(*id), *position))
            }
            _ => None,
        };
        if let Some((pointer, point)) = press
            && state.dragging.is_none()
        {
            let inside = hit.contains(point) && viewport.contains(point);
            if inside != state.focus {
                state.focus = inside;
                shell.request_redraw();
            }
            if inside {
                state.dragging = Some(pointer);
                shell.capture_event();
                return;
            }
        }
        let next = match event {
            Event::Mouse(mouse::Event::CursorMoved { position })
                if state.dragging == Some(Pointer::Mouse) =>
            {
                Some(self.at(bounds, *position))
            }
            Event::Touch(touch::Event::FingerMoved { id, position })
                if state.dragging == Some(Pointer::Touch(*id)) =>
            {
                Some(self.at(bounds, *position))
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if state.dragging == Some(Pointer::Mouse) =>
            {
                state.dragging = None;
                shell.capture_event();
                return;
            }
            Event::Touch(
                touch::Event::FingerLifted { id, .. } | touch::Event::FingerLost { id, .. },
            ) if state.dragging == Some(Pointer::Touch(*id)) => {
                state.dragging = None;
                shell.capture_event();
                return;
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key),
                modifiers,
                ..
            }) if state.focus => {
                let step = if modifiers.shift() { 0.10 } else { 0.01 };
                match (self.axis, key) {
                    (_, Named::Home) => Some(0.0),
                    (_, Named::End) => Some(1.0),
                    (Orientation::Horizontal, Named::ArrowLeft)
                    | (Orientation::Vertical, Named::ArrowUp) => Some(value - step),
                    (Orientation::Horizontal, Named::ArrowRight)
                    | (Orientation::Vertical, Named::ArrowDown) => Some(value + step),
                    _ => None,
                }
            }
            _ => None,
        };
        if let Some(next) = next {
            let next = fraction(next);
            if next != value {
                if self.value.is_none() {
                    state.value = Some(next);
                    shell.invalidate_layout();
                }
                if let Some(change) = &self.change {
                    shell.publish(change(next));
                }
                shell.request_redraw();
            }
            shell.capture_event();
            return;
        }
        for (panel, (layout, state)) in self
            .children
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            if let Some(visible) = viewport.intersection(&layout.bounds()) {
                panel.update(state, event, layout, cursor, renderer, shell, &visible);
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
        let state = tree.state.downcast_ref::<State>();
        if state.dragging.is_some()
            || (cursor.is_over(*viewport)
                && cursor.is_over(self.separator(layout.bounds(), self.current(state), 10.0)))
        {
            return match self.axis {
                Orientation::Horizontal => mouse::Interaction::ResizingColumn,
                Orientation::Vertical => mouse::Interaction::ResizingRow,
            };
        }
        self.children
            .iter()
            .zip(layout.iter(&tree.children))
            .filter_map(|(panel, (layout, state))| {
                viewport
                    .intersection(&layout.bounds())
                    .map(|v| panel.mouse_interaction(state, layout, cursor, &v, renderer))
            })
            .max()
            .unwrap_or_default()
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
        for (panel, (layout, state)) in self.children.iter().zip(layout.iter(&tree.children)) {
            if let Some(visible) = viewport.intersection(&layout.bounds()) {
                renderer.with_layer(visible, |renderer| {
                    panel.draw(state, renderer, theme, style, layout, cursor, &visible)
                });
            }
        }
        let state = tree.state.downcast_ref::<State>();
        let divider = self.separator(
            layout.bounds(),
            self.current(state),
            if state.focus { 2.0 } else { 1.0 },
        );
        let color = if state.focus {
            theme.neutral.ring
        } else {
            theme.neutral.border
        };
        renderer.fill_quad(
            renderer::Quad {
                bounds: divider,
                ..Default::default()
            },
            color,
        );
        if self.handle {
            let size = match self.axis {
                Orientation::Horizontal => Size::new(4.0, 24.0),
                Orientation::Vertical => Size::new(24.0, 4.0),
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle::new(
                        divider.center() - Vector::new(size.width / 2.0, size.height / 2.0),
                        size,
                    ),
                    border: border::rounded(4),
                    ..Default::default()
                },
                color,
            );
        }
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
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}
