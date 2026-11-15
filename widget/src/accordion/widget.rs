//! Accordion layout, interaction, and animation. The widget measures the natural height of each
//! body and clips it to its animated height during drawing. Opening or closing takes 200 ms.
//! The trigger chevron switches immediately.
//!
//! An uncontrolled accordion keeps its open items in the widget tree. A controlled accordion
//! reads the open set from the application. Pressing a trigger applies [`Mode::toggle`] and
//! publishes the complete new set.

use keyboard::Event::KeyReleased;
use std::time::Duration;

use iced_core::Length::{Fill, Shrink};
use iced_core::animation::Easing;
use iced_core::border::Border;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::layout;
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::time::Instant;
use iced_core::touch;
use iced_core::widget::operation::Focusable;
use iced_core::widget::tree::{self, Tree};
use iced_core::widget::{Meta, Operation};
use iced_core::window;
use iced_core::{
    Animation, Background, Color, Element, Event, Layout, Length, Padding, Rectangle, Shell, Size,
    Vector, Widget,
};

// Bring the renderer trait methods (`fill_quad`, `with_layer`) into scope
// for the concrete `iced_widget::Renderer`.
use iced_core::Renderer as _;

use icedcn_core::Theme;
use icedcn_core::theme::accordion::{self, Status};

/// Duration of the opening and closing animation.
const TRANSITION_DURATION: Duration = Duration::from_millis(200);

/// Vertical padding around each trigger row (`py-2.5` = 10 px).
const TRIGGER_PADDING_Y: f32 = 11.0; // 10 px padding plus the transparent 1 px border.

/// Transparent left and right borders on the trigger.
const TRIGGER_BORDER: f32 = 1.0;

/// Bottom padding inside the body wrapper (`pb-2.5` = 10 px). The top
/// padding is `pt-0` per nova spec — the trigger's own bottom padding
/// already separates label from body.
const BODY_PADDING_BOTTOM: f32 = 10.0;

/// One disclosure item — the trigger row, the body, and a per-item id for
/// open-state matching.
pub struct Item<'a, Message> {
    pub(crate) id: String,
    pub(crate) trigger: Element<'a, Message, Theme, iced_widget::Renderer>,
    pub(crate) chevron_closed: Element<'a, Message, Theme, iced_widget::Renderer>,
    pub(crate) chevron_open: Element<'a, Message, Theme, iced_widget::Renderer>,
    pub(crate) body: Element<'a, Message, Theme, iced_widget::Renderer>,
    pub(crate) disabled: bool,
    pub(crate) inset: f32,
}

/// Whether several items can be open together. In single mode, disabling collapse prevents the
/// user from closing the open item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// At most one item open. `collapsible = true` allows zero open.
    Single { collapsible: bool },
    /// Any subset open.
    Multiple,
}

impl Mode {
    /// The open set after a click on `clicked`'s trigger, or `None` when the
    /// mode refuses the click (closing the open item of a non-collapsible
    /// single accordion).
    pub fn toggle(self, open: &[String], clicked: &str) -> Option<Vec<String>> {
        let is_open = open.iter().any(|id| id == clicked);

        match (self, is_open) {
            (Mode::Single { collapsible: false }, true) => None,
            (Mode::Single { collapsible: true }, true) => Some(Vec::new()),
            (Mode::Single { .. }, false) => Some(vec![clicked.to_owned()]),
            (Mode::Multiple, true) => Some(
                open.iter()
                    .filter(|id| id.as_str() != clicked)
                    .cloned()
                    .collect(),
            ),
            (Mode::Multiple, false) => {
                let mut next = open.to_vec();
                next.push(clicked.to_owned());
                Some(next)
            }
        }
    }
}

/// The disclosure widget proper. Constructed by the public `accordion()`
/// builder in the parent module; users never name this type directly.
pub struct Accordion<'a, Message> {
    pub(crate) items: Vec<Item<'a, Message>>,
    pub(crate) mode: Mode,
    /// The open set: the app's when controlled, otherwise synced from the
    /// tree state in `diff`.
    pub(crate) open: Vec<String>,
    pub(crate) is_controlled: bool,
    pub(crate) on_value_change: Option<Box<dyn Fn(Vec<String>) -> Message + 'a>>,
    pub(crate) class: <Theme as Catalog>::Class<'a>,
    pub(crate) outer_padding: Padding,
}

impl<'a, Message> Accordion<'a, Message> {
    /// Whether item `id` is open. Used both at update time (to spot
    /// transitions) and at draw time (to pick which chevron to render).
    fn is_open(&self, id: &str) -> bool {
        self.open.iter().any(|s| s == id)
    }
}

/// Per-item runtime state stored in the widget's tree.
struct ItemState {
    /// Stable id for matching across renders.
    id: String,
    /// Last `is_open` value seen by `update` — when this drifts from the
    /// current external value, kick off the animation.
    last_is_open: bool,
    /// `Animation<bool>` tracks the eased transition between open/closed.
    /// `interpolate(0.0, natural_body_height, now)` yields the displayed
    /// body height.
    animation: Animation<bool>,
    /// Whether the cursor is currently inside this item's trigger row —
    /// drives the catalog's `Status::Hovered` branch.
    hovered: bool,
    focused: bool,
    focus_visible: bool,
    pressed: bool,
}

impl Focusable for ItemState {
    fn is_focused(&self) -> bool {
        self.focused
    }
    fn focus(&mut self) {
        self.focused = true;
        self.focus_visible = true;
    }
    fn unfocus(&mut self) {
        self.focused = false;
        self.pressed = false;
    }
}

impl ItemState {
    fn seeded(id: String, is_open: bool) -> Self {
        Self {
            id,
            last_is_open: is_open,
            animation: Animation::new(is_open)
                .duration(TRANSITION_DURATION)
                .easing(Easing::Custom(crate::animation::ease_out)),
            hovered: false,
            focused: false,
            focus_visible: false,
            pressed: false,
        }
    }
}

/// Tree state for the whole accordion: one [`ItemState`] per item, keyed
/// by id so reordering items between renders preserves animation state.
struct State {
    /// The open set of an uncontrolled accordion.
    open: Vec<String>,
    items: Vec<ItemState>,
    /// Most recent frame timestamp from the runtime. `None` until the
    /// first `RedrawRequested` fires.
    now: Option<Instant>,
}

impl<Message> Meta for Accordion<'_, Message> {}

impl<Message> Widget<Message, Theme, iced_widget::Renderer> for Accordion<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            open: self.open.clone(),
            items: Vec::new(),
            now: None,
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();
        if self.is_controlled {
            state.open.clone_from(&self.open);
        } else {
            self.open.clone_from(&state.open);
        }

        // Reconcile the four child widgets together with their header's interaction
        // and animation state. A retained key owns the whole row across moves.
        for (index, item) in self.items.iter_mut().enumerate() {
            if state.items.get(index).is_none_or(|s| s.id != item.id) {
                if let Some(offset) = state.items[index..].iter().position(|s| s.id == item.id) {
                    state.items.swap(index, index + offset);
                    tree.children.swap(index, index + offset);
                } else {
                    let open = self.open.contains(&item.id);
                    state
                        .items
                        .insert(index, ItemState::seeded(item.id.clone(), open));
                    tree.children.insert(index, Tree::empty());
                }
            }

            tree.children[index].diff_children(&mut [
                &mut item.trigger,
                &mut item.chevron_closed,
                &mut item.chevron_open,
                &mut item.body,
            ]);
        }
        state.items.truncate(self.items.len());
        tree.children.truncate(self.items.len());
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Fill,
            height: Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced_widget::Renderer,
        limits: &layout::Limits,
    ) {
        let limits = limits.width(Fill);

        // Outer padding eats into the available width before children lay out.
        let pad_h = self.outer_padding.left + self.outer_padding.right;
        let pad_v = self.outer_padding.top + self.outer_padding.bottom;
        let inner_limits = limits.shrink(Size::new(pad_h, pad_v));

        let inner_max = inner_limits.max;
        // Scrollable columns keep a finite remaining-height hint even when
        // their children can grow vertically. Below the first viewport that
        // hint can be zero; the trigger and its icons still need their height.
        let trigger_limits = layout::Limits::with_flags(
            Size::ZERO,
            inner_max,
            Size::new(false, inner_limits.compression.height),
            Size::new(false, inner_limits.infinite.height),
        );
        let mut cursor_y = 0.0_f32;
        let mut max_width = 0.0_f32;

        let state = tree.state.downcast_mut::<State>();
        let now_ref = state.now;

        let last = self.items.len().saturating_sub(1);
        for (index, (item, item_tree)) in self
            .items
            .iter_mut()
            .zip(tree.children.iter_mut())
            .enumerate()
        {
            // Tree layout: 4 child trees per item — trigger, chevron-closed,
            // chevron-open, body.
            let [
                trigger_tree,
                chevron_closed_tree,
                chevron_open_tree,
                body_tree,
            ] = item_tree.children.as_mut_slice()
            else {
                unreachable!("diff plants four trees per accordion item");
            };

            // ----- chevrons (laid out first so we know their size for the
            // trigger row's width budget) ------------------------------
            item.chevron_closed
                .layout(chevron_closed_tree, renderer, &trigger_limits);
            item.chevron_open
                .layout(chevron_open_tree, renderer, &trigger_limits);
            let chevron_closed_size = chevron_closed_tree.size;
            let chevron_open_size = chevron_open_tree.size;
            let chevron_w = chevron_closed_size.width.max(chevron_open_size.width);
            let chevron_h = chevron_closed_size.height.max(chevron_open_size.height);

            // ----- trigger label ---------------------------------------
            // Trigger label takes whatever's left after the chevron + gap.
            let item_inset = item.inset.min(inner_max.width / 2.0);
            let content_w = (inner_max.width - 2.0 * item_inset).max(0.0);
            let trigger_max_w = (content_w - chevron_w - 2.0 * TRIGGER_BORDER).max(0.0);
            let label_limits = layout::Limits {
                max: Size::new(trigger_max_w, inner_max.height),
                ..trigger_limits
            };
            item.trigger.layout(trigger_tree, renderer, &label_limits);

            let trigger_label_h = trigger_tree.size.height;
            let trigger_inner_h = trigger_label_h.max(chevron_h);
            let trigger_h = trigger_inner_h + TRIGGER_PADDING_Y * 2.0;
            let trigger_w = inner_max.width;

            // Nova aligns the top edges even when a long trigger wraps.
            let left = item_inset + TRIGGER_BORDER;
            let right = trigger_w - left;
            trigger_tree.translation = Vector::new(left, TRIGGER_PADDING_Y);
            chevron_closed_tree.translation =
                Vector::new(right - chevron_closed_size.width, TRIGGER_PADDING_Y);
            chevron_open_tree.translation =
                Vector::new(right - chevron_open_size.width, TRIGGER_PADDING_Y);

            // ----- body --------------------------------------------------
            // Always lay out the body to capture its natural height. Width is
            // the same as the trigger row so child reflow stays consistent
            // across open/close transitions.
            let body_limits =
                layout::Limits::new(Size::ZERO, Size::new(content_w, f32::INFINITY)).loose();
            item.body.layout(body_tree, renderer, &body_limits);

            let body_natural_h = body_tree.size.height + BODY_PADDING_BOTTOM;

            let item_state = &state.items[index];

            // Compute the displayed body height — interpolate between 0
            // and `body_natural_h` over the animation progress.
            let displayed_body_h = match now_ref {
                Some(now) => item_state.animation.interpolate(0.0, body_natural_h, now),
                None => {
                    // Pre-first-frame: snap to the steady-state extreme. The
                    // animation hasn't been able to start ticking yet, so
                    // any in-flight transition won't render until the next
                    // RedrawRequested arrives — we fall back to "wherever
                    // the animation thinks it currently is" via `value()`.
                    if item_state.animation.value() {
                        body_natural_h
                    } else {
                        0.0
                    }
                }
            };

            // Body sits below the trigger row. The body's *own* layout
            // node keeps its full natural height (so child paint coords
            // are stable); we tell the parent layout that the item's
            // outer height is `trigger + displayed_body_h` and clip the
            // body in draw to the same band.
            let body_y = trigger_h;
            // Position content with no top padding — `pt-0` per nova spec.
            body_tree.translation = Vector::new(item_inset, body_y);

            let item_outer_h = trigger_h + displayed_body_h + if index < last { 1.0 } else { 0.0 };
            let item_outer_w = trigger_w;

            item_tree.size = Size::new(item_outer_w, item_outer_h);
            item_tree.translation =
                Vector::new(self.outer_padding.left, self.outer_padding.top + cursor_y);

            cursor_y += item_outer_h;
            max_width = max_width.max(item_outer_w);
        }

        let total_w = max_width + pad_h;
        let total_h = cursor_y + pad_v;
        tree.size = Size::new(total_w, total_h);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &iced_widget::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        let state = tree.state.downcast_mut::<State>();
        for ((item, header), (item_layout, item_tree)) in self
            .items
            .iter_mut()
            .zip(&mut state.items)
            .zip(layout.iter_mut(&mut tree.children))
        {
            let mut children = item_layout.iter_mut(&mut item_tree.children);
            let (trigger_layout, trigger_tree) = children.next().expect("trigger layout");
            let _chevron_closed = children.next();
            let _chevron_open = children.next();
            let (body_layout, body_tree) = children.next().expect("body layout");

            if !item.disabled {
                operation.focusable(None, trigger_layout.bounds(), header);
            }
            item.trigger
                .operate(trigger_tree, trigger_layout, viewport, renderer, operation);
            if state.open.contains(&item.id) {
                item.body
                    .operate(body_tree, body_layout, viewport, renderer, operation);
            }
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &iced_widget::Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        // Animation bookkeeping. Order matters: cache `now` BEFORE the
        // is_open-drift detection below so newly-kicked animations see a
        // valid `now` on the same frame they're started.
        //
        // Body height is animated, and the interpolation runs in `layout`
        // (`displayed_body_h = animation.interpolate(0, natural, now)`).
        // Iced caches layout across frames and only re-runs it when a
        // shell explicitly invalidates — so EVERY frame an animation is
        // in progress we must `invalidate_layout`, otherwise the cached
        // `displayed_body_h` from the last layout pass freezes the body
        // mid-tween. Unlike colour-animating widgets (`button`, `switch`,
        // `progress`) which animate purely in `draw`, accordion's tween
        // is a layout-affecting size, so request_redraw alone is not enough.
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let previous = state.now.replace(*now);
            for s in &state.items {
                if s.animation.is_animating(*now) {
                    shell.invalidate_layout();
                    shell.request_redraw();
                } else if previous.is_some_and(|last| s.animation.is_animating(last)) {
                    shell.invalidate_layout();
                }
            }
        }

        // Detect external is_open drift and kick the animation.
        for (item, s) in self.items.iter().zip(&mut state.items) {
            let is_open = self.is_open(&item.id);
            if is_open != s.last_is_open {
                if let Some(now) = state.now {
                    s.animation.go_mut(is_open, now);
                    s.last_is_open = is_open;
                }
                shell.invalidate_layout();
                shell.request_redraw();
            }
        }

        // Hover + click handling per item.
        let mut clicked = None;
        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Named(key),
            ..
        }) = event
            && let Some(index) = state.items.iter().position(|item| item.focused)
            && !self.items[index].disabled
        {
            let enabled = || {
                self.items
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| !item.disabled)
                    .map(|(i, _)| i)
            };
            let target = match key {
                Named::Home => enabled().next(),
                Named::End => enabled().next_back(),
                Named::ArrowDown => enabled().find(|i| *i > index).or_else(|| enabled().next()),
                Named::ArrowUp => enabled()
                    .rev()
                    .find(|i| *i < index)
                    .or_else(|| enabled().next_back()),
                Named::Enter => {
                    clicked = Some(self.items[index].id.clone());
                    Some(index)
                }
                Named::Space => {
                    state.items[index].pressed = true;
                    Some(index)
                }
                _ => None,
            };
            if let Some(target) = target {
                if index != target {
                    state.items[index].unfocus();
                }
                state.items[target].focus();
                shell.request_redraw();
                shell.capture_event();
            }
        }

        if matches!(
            event,
            Event::Keyboard(KeyReleased {
                key: Key::Named(Named::Space),
                ..
            })
        ) && let Some(index) = state
            .items
            .iter()
            .position(|item| item.focused && item.pressed)
            && !self.items[index].disabled
        {
            state.items[index].pressed = false;
            clicked = Some(self.items[index].id.clone());
            shell.capture_event();
        }

        for ((item, s), (item_layout, item_tree)) in self
            .items
            .iter_mut()
            .zip(&mut state.items)
            .zip(layout.iter_mut(&mut tree.children))
        {
            let mut children = item_layout.iter_mut(&mut item_tree.children);
            let (trigger_layout, trigger_tree) = children.next().expect("trigger layout");
            let _ = children.next().expect("chevron-closed layout");
            let _ = children.next();
            let (body_layout, body_tree) = children.next().expect("body layout");

            let trigger_row_height = body_layout.bounds().y - item_layout.bounds().y;
            let trigger_hit = Rectangle {
                x: item_layout.bounds().x + item.inset,
                y: item_layout.bounds().y,
                width: (item_layout.bounds().width - 2.0 * item.inset).max(0.0),
                height: trigger_row_height,
            };

            let over = !item.disabled
                && match event {
                    Event::Touch(
                        touch::Event::FingerPressed { position, .. }
                        | touch::Event::FingerLifted { position, .. }
                        | touch::Event::FingerMoved { position, .. },
                    ) => trigger_hit.contains(*position),
                    _ => cursor.is_over(trigger_hit),
                };
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    s.pressed = over;
                    s.focused = over;
                    s.focus_visible = false;
                    if over {
                        shell.capture_event();
                    }
                    shell.request_redraw();
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerLifted { .. }) => {
                    if s.pressed && over {
                        clicked = Some(item.id.clone());
                        shell.capture_event();
                    }
                    s.pressed = false;
                }
                Event::Touch(touch::Event::FingerLost { .. })
                | Event::Window(window::Event::Unfocused) => {
                    s.pressed = false;
                }
                _ => {}
            }
            if over != s.hovered {
                s.hovered = over;
                shell.request_redraw();
            }

            item.trigger.update(
                trigger_tree,
                event,
                trigger_layout,
                cursor,
                renderer,
                shell,
                viewport,
            );

            // Skip body event forwarding when fully collapsed (avoids
            // clicks bleeding into hidden content).
            if self.open.contains(&item.id)
                && let Some(clip) = item_layout.bounds().intersection(viewport)
                && let Some(clip) = clip.intersection(&body_layout.bounds())
            {
                let cursor = if cursor.is_over(clip) {
                    cursor
                } else {
                    mouse::Cursor::Unavailable
                };
                item.body.update(
                    body_tree,
                    event,
                    body_layout,
                    cursor,
                    renderer,
                    shell,
                    &clip,
                );
            }
        }

        if let Some(clicked) = clicked
            && let Some(next) = self.mode.toggle(&self.open, &clicked)
        {
            if !self.is_controlled {
                state.open.clone_from(&next);
                self.open.clone_from(&next);
                shell.invalidate_layout();
                shell.request_redraw();
            }

            if let Some(on_value_change) = &self.on_value_change {
                shell.publish(on_value_change(next));
            }

            shell.capture_event();
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced_widget::Renderer,
    ) -> mouse::Interaction {
        for (item, (item_layout, item_tree)) in self.items.iter().zip(layout.iter(&tree.children)) {
            let Some((body_layout, body_tree)) = item_layout.iter(&item_tree.children).nth(3)
            else {
                continue;
            };
            if self.is_open(&item.id)
                && cursor.is_over(item_layout.bounds())
                && cursor.is_over(body_layout.bounds())
            {
                return item.body.mouse_interaction(
                    body_tree,
                    body_layout,
                    cursor,
                    viewport,
                    renderer,
                );
            }
        }
        mouse::Interaction::default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced_widget::Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        let resolve_style = |status: Status| theme.style(&self.class, status);

        // Outer border for the wrapped accordion ("borders" example).
        // Drawn first so item content paints on top.
        let outer_style = resolve_style(Status::Active { is_open: false });
        if outer_style.border.width > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        width: outer_style.border.width,
                        color: outer_style.border.color.into(),
                        radius: outer_style.border.radius.into(),
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
        }

        let last_index = self.items.len().saturating_sub(1);

        for (item_index, (item, (item_layout, item_tree))) in self
            .items
            .iter()
            .zip(layout.iter(&tree.children))
            .enumerate()
        {
            let mut children = item_layout.iter(&item_tree.children);
            let (trigger_layout, trigger_tree) = children.next().expect("trigger layout");
            let (chevron_closed_layout, chevron_closed_tree) =
                children.next().expect("chevron-closed layout");
            let (chevron_open_layout, chevron_open_tree) =
                children.next().expect("chevron-open layout");
            let (body_layout, body_tree) = children.next().expect("body layout");

            let item_state = &state.items[item_index];

            let is_open_now = self.is_open(&item.id);
            let status = if item.disabled {
                Status::Disabled {
                    is_open: is_open_now,
                }
            } else if item_state.focused && item_state.focus_visible {
                Status::Focused {
                    is_open: is_open_now,
                }
            } else if item_state.hovered {
                Status::Hovered {
                    is_open: is_open_now,
                }
            } else {
                Status::Active {
                    is_open: is_open_now,
                }
            };
            let style = resolve_style(status);

            let alpha = if item.disabled {
                style.disabled_alpha
            } else {
                1.0
            };

            // ----- divider ---------------------------------------------
            // Paint the bottom border under every item except the last.
            if item_index < last_index && style.divider.a > 0.0 {
                let bounds = item_layout.bounds();
                let line_y = bounds.y + bounds.height - 1.0;
                let line = Rectangle {
                    x: bounds.x,
                    y: line_y,
                    width: bounds.width,
                    height: 1.0,
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: line,
                        ..renderer::Quad::default()
                    },
                    Background::Color(style.divider.into()),
                );
            }

            // ----- focus ring (around the trigger row) -----------------
            if let Some(ring) = style.ring {
                let trigger_row_bounds = Rectangle {
                    x: item_layout.bounds().x + item.inset,
                    y: item_layout.bounds().y,
                    width: (item_layout.bounds().width - item.inset * 2.0).max(0.0),
                    height: body_layout.bounds().y - item_layout.bounds().y,
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: trigger_row_bounds.x - ring.width,
                            y: trigger_row_bounds.y - ring.width,
                            width: trigger_row_bounds.width + ring.width * 2.0,
                            height: trigger_row_bounds.height + ring.width * 2.0,
                        },
                        border: Border {
                            width: ring.width,
                            color: ring.color.into(),
                            radius: style.trigger_radius.into(),
                        },
                        ..renderer::Quad::default()
                    },
                    Background::Color(Color::TRANSPARENT),
                );
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: trigger_row_bounds,
                        border: Border {
                            width: TRIGGER_BORDER,
                            color: theme.neutral.ring.into(),
                            radius: style.trigger_radius.into(),
                        },
                        ..renderer::Quad::default()
                    },
                    Background::Color(Color::TRANSPARENT),
                );
            }

            // ----- trigger label ---------------------------------------
            let label_color = style.trigger_text.scale_alpha(alpha);
            item.trigger.draw(
                trigger_tree,
                renderer,
                theme,
                &renderer::Style {
                    text_color: label_color.into(),
                },
                trigger_layout,
                cursor,
                viewport,
            );

            if item_state.hovered && !item.disabled {
                let label = trigger_layout.bounds();
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: label.x,
                            y: label.y + label.height - 3.0,
                            width: label.width,
                            height: 1.0,
                        },
                        ..renderer::Quad::default()
                    },
                    Background::Color(label_color.into()),
                );
            }

            // ----- chevron (one of two; the other is laid out but skipped) -----
            let chevron_color = style.chevron.scale_alpha(alpha);
            let chevron_defaults = renderer::Style {
                text_color: chevron_color.into(),
            };
            let (chevron_layout, chevron_tree, chevron_element) = if is_open_now {
                (chevron_open_layout, chevron_open_tree, &item.chevron_open)
            } else {
                (
                    chevron_closed_layout,
                    chevron_closed_tree,
                    &item.chevron_closed,
                )
            };
            chevron_element.draw(
                chevron_tree,
                renderer,
                theme,
                &chevron_defaults,
                chevron_layout,
                cursor,
                viewport,
            );

            // ----- body (clipped to the animated height band) ----------
            let item_bounds = item_layout.bounds();
            let trigger_row_height = body_layout.bounds().y - item_bounds.y;
            let divider = if item_index < last_index { 1.0 } else { 0.0 };
            let displayed_body_h = (item_bounds.height - trigger_row_height - divider).max(0.0);
            if displayed_body_h > 0.0 {
                let body_clip = Rectangle {
                    x: item_bounds.x,
                    y: item_bounds.y + trigger_row_height,
                    width: item_bounds.width,
                    height: displayed_body_h,
                };

                let Some(intersected) = body_clip.intersection(viewport) else {
                    continue;
                };
                let body_text_color = style.body_text;
                let body_defaults = renderer::Style {
                    text_color: body_text_color.into(),
                };

                renderer.with_layer(intersected, |renderer| {
                    item.body.draw(
                        body_tree,
                        renderer,
                        theme,
                        &body_defaults,
                        body_layout,
                        cursor,
                        viewport,
                    );
                });
            }

            // Suppress unused-variable warnings when `defaults` is the
            // best fallback for the trigger but no current branch uses it
            // directly.
            let _ = defaults;
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &iced_widget::Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, iced_widget::Renderer>> {
        let state = tree.state.downcast_ref::<State>();
        let mut overlays: Vec<overlay::Element<'b, Message, Theme, iced_widget::Renderer>> =
            Vec::new();

        for (item, (item_layout, item_tree)) in self
            .items
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            let mut children = item_layout.iter_mut(&mut item_tree.children);
            let (trigger_layout, trigger_tree) = children.next().expect("trigger layout");
            let _chevron_closed = children.next();
            let _chevron_open = children.next();
            let (body_layout, body_tree) = children.next().expect("body layout");

            overlays.extend(item.trigger.overlay(
                trigger_tree,
                trigger_layout,
                renderer,
                viewport,
                translation,
                window,
            ));
            if state.open.contains(&item.id) {
                overlays.extend(item.body.overlay(
                    body_tree,
                    body_layout,
                    renderer,
                    viewport,
                    translation,
                    window,
                ));
            }
        }

        overlays
    }
}

impl<'a, Message: 'a> From<Accordion<'a, Message>>
    for Element<'a, Message, Theme, iced_widget::Renderer>
{
    fn from(a: Accordion<'a, Message>) -> Self {
        a._boxed()
    }
}

// --- Catalog -----------------------------------------------------------------

/// Theme catalog for the accordion widget.
///
/// Mirrors the shape of `switch::Catalog` / `button::Catalog`: a class
/// type per `'a` and a `style(class, status)` resolver.
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`accordion::Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> accordion::Style;
}

/// Convenience type-alias for the styling closure type.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> accordion::Style + 'a>;
