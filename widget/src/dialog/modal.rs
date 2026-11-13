//! The widget behind [`Dialog`](super::Dialog) and
//! [`Sheet`](crate::sheet::Sheet): it lays out its base content in place and,
//! while open, its panel in an overlay above a backdrop.
//!
//! The overlay is modal: it captures every pointer event, and closes the
//! dialog on a press outside the panel, on `Escape`, or when a
//! [`marker`] inside the panel records [`Signal::Close`] (a `DialogClose`).

use crate::core::keyboard;
use crate::core::keyboard::key;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::time::{Duration, Instant};
use crate::core::touch;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::{
    Background, Color, Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget,
};
use crate::core::{Point, Transformation, window};
use crate::marker;
use crate::operation::focus;

/// What a dialog's markers signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    /// A trigger was pressed.
    Open,
    /// A close button was pressed.
    Close,
}

/// Where the panel sits in the window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Placement {
    /// Centered, at most `max_width` wide (a dialog).
    Center {
        /// The panel's maximum width.
        max_width: f32,
    },
    /// Pinned to a window edge (a sheet).
    Side(Side),
}

/// The window edge a sheet slides in from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Side {
    /// Full width, at the top.
    Top,
    /// Full height, on the right.
    #[default]
    Right,
    /// Full width, at the bottom.
    Bottom,
    /// Full height, on the left.
    Left,
}

/// Base content plus an optional modal panel.
pub struct Modal<'a, Message, Theme, Renderer> {
    base: Element<'a, Message, Theme, Renderer>,
    panel: Option<Element<'a, Message, Theme, Renderer>>,
    placement: Placement,
    on_close: Box<dyn Fn() -> Message + 'a>,
    dismiss_outside: bool,
    closing: Option<(Instant, fn() -> Message)>,
    drawer_height: Option<f32>,
    side_width: Option<f32>,
}

impl<'a, Message, Theme, Renderer> Modal<'a, Message, Theme, Renderer> {
    /// Show `base` in place and, when `panel` is `Some`, the panel above
    /// everything; `on_close` is published when the panel is dismissed.
    pub fn new(
        base: Element<'a, Message, Theme, Renderer>,
        panel: Option<Element<'a, Message, Theme, Renderer>>,
        placement: Placement,
        on_close: Message,
    ) -> Self
    where
        Message: Clone + 'a,
    {
        Self::with_close(base, panel, placement, move || on_close.clone())
    }
    pub(crate) fn with_close(
        base: Element<'a, Message, Theme, Renderer>,
        panel: Option<Element<'a, Message, Theme, Renderer>>,
        placement: Placement,
        on_close: impl Fn() -> Message + 'a,
    ) -> Self {
        Self {
            base,
            panel,
            placement,
            on_close: Box::new(on_close),
            dismiss_outside: true,
            closing: None,
            drawer_height: None,
            side_width: None,
        }
    }

    pub(crate) fn side_width(mut self, width: f32) -> Self {
        self.side_width = Some(width);
        self
    }
    pub(super) fn animation(mut self, at: Option<Instant>, exited: fn() -> Message) -> Self {
        self.closing = at.map(|instant| (instant, exited));
        self
    }
    pub(super) fn drawer_height(mut self, height: Option<f32>) -> Self {
        self.drawer_height = height;
        self
    }
    fn open(&self) -> bool {
        self.panel.is_some() && self.closing.is_none()
    }
    /// Whether a press outside the panel dismisses it (on by default; an
    /// alert dialog only closes from its own actions and `Escape`).
    pub fn dismiss_outside(mut self, dismiss: bool) -> Self {
        self.dismiss_outside = dismiss;
        self
    }
}

#[derive(Default)]
struct Focus {
    mounted: bool,
    enter: bool,
    restore: Option<usize>,
    drag: Option<(touch::Finger, Point)>,
}

impl<Message, Theme, Renderer> Meta for Modal<'_, Message, Theme, Renderer> {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Modal<'_, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Focus>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(Focus::default())
    }

    fn diff(&mut self, tree: &mut Tree) {
        let focus = tree.state.downcast_mut::<Focus>();
        let open = self.open();
        if open && !focus.mounted {
            focus.enter = true;
        }
        focus.mounted = open;
        // Child 0 is the base, child 1 the panel while open. Closing drops
        // the panel's tree, so it mounts fresh next time, as in Radix.
        match &mut self.panel {
            Some(panel) => tree.diff_children(&mut [&mut self.base, panel]),
            None => tree.diff_children(std::slice::from_mut(&mut self.base)),
        }
    }

    fn size(&self) -> Size<Length> {
        self.base.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.base.layout(&mut tree.children[0], renderer, limits);
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
            .expect("a modal has its base");

        if self.open() && matches!(event, Event::Keyboard(_)) {
            return;
        }
        self.base.update(
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
            .expect("a modal has its base");

        self.base.draw(
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
            .expect("a modal has its base");

        self.base
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
            .expect("a modal has its base");

        if !self.open() {
            self.base
                .operate(child_tree, child_layout, viewport, renderer, operation);
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
        let (base_tree, panel_tree) = tree.children.split_at_mut(1);

        let (child_layout, base_tree) = layout
            .iter_mut(base_tree)
            .next()
            .expect("a modal has its base");

        let focus = tree.state.downcast_mut::<Focus>();
        if focus.enter && self.open() {
            let count = focus::count(|op| {
                self.base
                    .operate(base_tree, child_layout, viewport, renderer, op)
            });
            focus.restore = count.focused.or((count.total > 0).then_some(0));
            self.base.operate(
                base_tree,
                child_layout,
                viewport,
                renderer,
                &mut focus::select(None),
            );
        } else if !self.open()
            && let Some(index) = focus.restore.take()
        {
            self.base.operate(
                base_tree,
                child_layout,
                viewport,
                renderer,
                &mut focus::select(Some(index)),
            );
        }
        let mut overlays = self.base.overlay(
            base_tree,
            child_layout,
            renderer,
            viewport,
            translation,
            window,
        );

        if let (Some(panel), Some(panel_tree)) = (&mut self.panel, panel_tree.first_mut()) {
            let limits = match self.placement {
                Placement::Center { max_width } => layout::Limits::new(
                    Size::ZERO,
                    Size::new(
                        (window.width - 32.0).min(max_width).max(0.0),
                        (window.height - 32.0).max(0.0),
                    ),
                ),
                Placement::Side(Side::Left | Side::Right) => {
                    let width = self.side_width.map_or_else(
                        || (window.width * 0.75).min(384.),
                        |width| width.min(window.width),
                    );
                    layout::Limits::new(
                        Size::new(width, window.height),
                        Size::new(width, window.height),
                    )
                }
                // `h-auto`: the content's height, up to the window's.
                Placement::Side(Side::Top | Side::Bottom) => layout::Limits::new(
                    Size::new(window.width, 0.0),
                    Size::new(
                        window.width,
                        window.height * self.drawer_height.unwrap_or(1.0),
                    ),
                ),
            };

            panel.layout(panel_tree, renderer, &limits);
            let size = panel_tree.size;

            let position = match self.placement {
                Placement::Center { .. } => [
                    (window.width - size.width) / 2.0,
                    (window.height - size.height) / 2.0,
                ],
                Placement::Side(Side::Left | Side::Top) => [0.0, 0.0],
                Placement::Side(Side::Right) => [window.width - size.width, 0.0],
                Placement::Side(Side::Bottom) => [0.0, window.height - size.height],
            };

            let layout = Layout::new(size).move_to(position);
            if std::mem::take(&mut focus.enter) {
                panel.operate(
                    panel_tree,
                    layout,
                    &Rectangle::with_size(window),
                    renderer,
                    &mut focus::select(Some(0)),
                );
            }
            overlays.push(overlay::Element::new(Box::new(Panel {
                panel,
                tree: panel_tree,
                layout,
                window,
                on_close: self.on_close.as_ref(),
                dismiss_outside: self.dismiss_outside,
                closing: self.closing,
                placement: self.placement,
                drag: self.drawer_height.map(|_| &mut focus.drag),
            })));
        }

        overlays
    }
}

impl<'a, Message, Theme, Renderer> From<Modal<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: crate::core::Renderer + 'a,
{
    fn from(modal: Modal<'a, Message, Theme, Renderer>) -> Self {
        modal._boxed()
    }
}

/// The open panel, drawn above a backdrop covering the window.
struct Panel<'a, 'b, Message, Theme, Renderer> {
    panel: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut Tree,
    layout: Layout,
    window: Size,
    on_close: &'b dyn Fn() -> Message,
    dismiss_outside: bool,
    closing: Option<(Instant, fn() -> Message)>,
    placement: Placement,
    drag: Option<&'b mut Option<(touch::Finger, Point)>>,
}

impl<Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Panel<'_, '_, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer,
{
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        let viewport = Rectangle::with_size(self.window);

        let progress = self.closing.as_ref().map_or(0., |(start, _)| {
            (start.elapsed().as_secs_f32() / 0.15).clamp(0., 1.)
        });
        if progress >= 1. {
            return;
        }
        let bounds = self.layout.bounds();
        let transform = match self.placement {
            Placement::Center { .. } => {
                let center = bounds.center();
                Transformation::translate(center.x, center.y)
                    * Transformation::scale(1. - 0.05 * progress)
                    * Transformation::translate(-center.x, -center.y)
            }
            Placement::Side(Side::Left) => Transformation::translate(-bounds.width * progress, 0.),
            Placement::Side(Side::Right) => Transformation::translate(bounds.width * progress, 0.),
            Placement::Side(Side::Top) => Transformation::translate(0., -bounds.height * progress),
            Placement::Side(Side::Bottom) => {
                Transformation::translate(0., bounds.height * progress)
            }
        };
        renderer.with_layer(viewport, |renderer| {
            // nova `bg-black/10`.
            renderer.fill_quad(
                renderer::Quad {
                    bounds: viewport,
                    ..renderer::Quad::default()
                },
                Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.1 * (1. - progress))),
            );

            renderer.with_transformation(transform, |renderer| {
                self.panel.draw(
                    self.tree,
                    renderer,
                    theme,
                    style,
                    self.layout,
                    cursor,
                    &viewport,
                )
            });
        });
    }

    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn Operation) {
        if self.closing.is_some() {
            return;
        }
        self.panel.operate(
            self.tree,
            self.layout,
            &Rectangle::with_size(self.window),
            renderer,
            operation,
        );
    }

    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        if let Some((start, exited)) = &self.closing {
            if let Event::Window(window::Event::RedrawRequested(now)) = event {
                if now.saturating_duration_since(*start) >= Duration::from_millis(150) {
                    shell.publish(exited());
                } else {
                    shell.request_redraw();
                }
            }
            return;
        }
        if let Some(drag) = self.drag.as_deref_mut() {
            match *event {
                Event::Touch(touch::Event::FingerPressed { id, position })
                    if self.layout.bounds().contains(position) =>
                {
                    *drag = Some((id, position))
                }
                Event::Touch(touch::Event::FingerLost { id, .. })
                    if drag.is_some_and(|(finger, _)| finger == id) =>
                {
                    *drag = None
                }
                Event::Touch(touch::Event::FingerLifted { id, position })
                    if drag.is_some_and(|(finger, _)| finger == id) =>
                {
                    if let Some((_, start)) = drag.take() {
                        let delta = position - start;
                        let (distance, cross) = match self.placement {
                            Placement::Side(Side::Bottom) => (delta.y, delta.x),
                            Placement::Side(Side::Top) => (-delta.y, delta.x),
                            Placement::Side(Side::Left) => (-delta.x, delta.y),
                            Placement::Side(Side::Right) => (delta.x, delta.y),
                            _ => (0., 0.),
                        };
                        if distance > 64. && distance > cross.abs() * 1.5 {
                            shell.publish((self.on_close)());
                            shell.capture_event();
                            return;
                        }
                    }
                }
                _ => {}
            }
        }
        let viewport = Rectangle::with_size(self.window);

        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::Tab),
            modifiers,
            ..
        }) = event
        {
            let count = focus::count(|op| {
                self.panel
                    .operate(self.tree, self.layout, &viewport, renderer, op)
            });
            self.panel.operate(
                self.tree,
                self.layout,
                &viewport,
                renderer,
                &mut focus::select(focus::adjacent(count, modifiers.shift())),
            );
            shell.request_redraw();
            shell.capture_event();
            return;
        }
        self.panel.update(
            self.tree,
            event,
            self.layout,
            cursor,
            renderer,
            shell,
            &viewport,
        );

        let mut drain = marker::drain::<Signal>();
        self.panel
            .operate(self.tree, self.layout, &viewport, renderer, &mut drain);

        let mut close = drain.into_signals().contains(&Signal::Close);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(_))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                close |= self.dismiss_outside && !cursor.is_over(self.layout.bounds());
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            }) => {
                close = true;
            }
            _ => {}
        }

        if close {
            shell.publish((self.on_close)());
        }

        // Modal: nothing below the backdrop sees the pointer.
        if matches!(
            event,
            Event::Mouse(_) | Event::Touch(_) | Event::Keyboard(_)
        ) {
            shell.capture_event();
        }
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        if self.closing.is_some() {
            return mouse::Interaction::None;
        }
        let interaction = self.panel.mouse_interaction(
            self.tree,
            self.layout,
            cursor,
            &Rectangle::with_size(self.window),
            renderer,
        );

        // The backdrop covers the window, so the base never sees the cursor.
        if interaction == mouse::Interaction::None {
            mouse::Interaction::Idle
        } else {
            interaction
        }
    }

    fn overlay<'c>(
        &'c mut self,
        renderer: &Renderer,
    ) -> Vec<overlay::Element<'c, Message, Theme, Renderer>> {
        if self.closing.is_some() {
            return Vec::new();
        }
        self.panel.overlay(
            self.tree,
            self.layout,
            renderer,
            &Rectangle::with_size(self.window),
            Vector::ZERO,
            self.window,
        )
    }
}
