//! Badge-owned passive hover, link input, clipping and state paint.

use std::time::Duration;

use iced_core::Renderer as _;
use iced_core::animation::Easing;
use iced_core::border::Radius;
use iced_core::keyboard::{self, key::Named};
use iced_core::time::Instant;
use iced_core::widget::operation::{self, Focusable};
use iced_core::widget::tree::{self, Tree};
use iced_core::widget::{Id, Meta, Operation};
use iced_core::{
    Animation, Background, Color, Element, Event, Layout, Length, Padding, Rectangle, Shell, Size,
    Vector, Widget, layout, mouse, overlay, renderer, touch, window,
};
use iced_widget::Renderer;
use icedxdy_core::theme::badge::state::{Status, Style, StyleFn};
use icedxdy_core::{self, Theme};

use crate::operation::focus;

pub(super) struct Badge<'a, Message> {
    pub content: Element<'a, Message, Theme, Renderer>,
    pub style: StyleFn<'a>,
    pub on_press: Option<Message>,
    pub disabled: bool,
    pub invalid: bool,
    pub padding: Padding,
}

struct State {
    focused: bool,
    indicator: focus::Visible,
    pressed: bool,
    hover: Animation<bool>,
    focus: Animation<bool>,
    frame: Instant,
    text: Vec<Rectangle>,
}

fn transition() -> Animation<bool> {
    Animation::new(false)
        .easing(Easing::Custom(|t| {
            crate::animation::cubic_bezier(0.4, 0.0, 0.2, 1.0, t)
        }))
        .duration(Duration::from_millis(150))
}

impl Default for State {
    fn default() -> Self {
        Self {
            focused: false,
            indicator: focus::Visible::default(),
            pressed: false,
            hover: transition(),
            focus: transition(),
            frame: Instant::now(),
            text: Vec::new(),
        }
    }
}

impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }
    fn focus(&mut self) {
        self.focused = true;
        self.indicator.0 = true;
    }
    fn unfocus(&mut self) {
        self.focused = false;
        self.pressed = false;
    }
}

impl<Message> Meta for Badge<'_, Message> {}

impl<Message> Badge<'_, Message> {
    fn status(&self, state: &State, hovered: bool) -> Status {
        Status {
            hovered,
            pressed: state.pressed,
            focused: state.focused && state.indicator.0,
            interactive: self.on_press.is_some(),
            disabled: self.disabled,
            invalid: self.invalid,
        }
        .normalized()
    }
}

impl<Message: Clone> Widget<Message, Theme, Renderer> for Badge<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
        if self.disabled || self.on_press.is_none() {
            let state = tree.state.downcast_mut::<State>();
            state.focused = false;
            state.pressed = false;
            state.indicator.0 = false;
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fit.stack(self.content.size().width),
            Length::Fixed(20.0),
        )
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        layout::positioned(
            tree,
            limits,
            Length::Fit.stack(self.content.size().width),
            20,
            self.padding,
            |child, limits| {
                self.content.layout(child, renderer, limits);
                child.size
            },
            |content, available| Vector::new(0.0, (available.height - content.height) / 2.0),
        );
        // Text decoration uses the laid-out text bounds. SVGs do not report
        // text, so underlines never extend through their icon boxes or gaps.
        let mut text = Vec::new();
        let local = Layout::new(tree.size);
        let (child_layout, child) = local
            .iter_mut(&mut tree.children)
            .next()
            .expect("Badge content");
        self.content.operate(
            child,
            child_layout,
            &local.bounds(),
            renderer,
            &mut TextBounds(&mut text),
        );
        tree.state.downcast_mut::<State>().text = text;
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if self.on_press.is_some() && !self.disabled {
            let state = tree.state.downcast_mut::<State>();
            operation.focusable(None, layout.bounds(), state);
            operation.custom(None, layout.bounds(), &mut state.indicator);
        }
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            let (child_layout, child) = layout
                .iter_mut(&mut tree.children)
                .next()
                .expect("Badge content");
            self.content
                .operate(child, child_layout, viewport, renderer, operation);
        });
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
        let hit = layout
            .bounds()
            .intersection(viewport)
            .is_some_and(|b| cursor.is_over(b));
        let (child_layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("Badge content");
        self.content.update(
            child,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
        let state = tree.state.downcast_mut::<State>();
        let interactive = self.on_press.is_some() && !self.disabled;
        if interactive && !shell.is_event_captured() {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    if hit {
                        state.pressed = true;
                        state.focused = true;
                        state.indicator.0 = false;
                        shell.capture_event();
                    } else {
                        state.unfocus();
                    }
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    if state.pressed {
                        state.pressed = false;
                        if hit && let Some(message) = &self.on_press {
                            shell.publish(message.clone());
                        }
                        shell.capture_event();
                    }
                }
                Event::Touch(touch::Event::FingerPressed { position, .. }) => {
                    if layout.bounds().contains(*position) && viewport.contains(*position) {
                        state.pressed = true;
                        state.focused = true;
                        state.indicator.0 = false;
                        shell.capture_event();
                    } else {
                        state.unfocus();
                    }
                }
                Event::Touch(touch::Event::FingerLifted { position, .. }) => {
                    if state.pressed {
                        state.pressed = false;
                        if layout.bounds().contains(*position)
                            && viewport.contains(*position)
                            && let Some(message) = &self.on_press
                        {
                            shell.publish(message.clone());
                        }
                        shell.capture_event();
                    }
                }
                Event::Touch(touch::Event::FingerLost { .. })
                | Event::Window(window::Event::Unfocused) => {
                    state.pressed = false;
                }
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) if state.focused => {
                    state.indicator.0 = true;
                    if matches!(
                        key.as_ref(),
                        keyboard::Key::Named(Named::Enter | Named::Space)
                    ) {
                        if let Some(message) = &self.on_press {
                            shell.publish(message.clone());
                        }
                        shell.capture_event();
                    }
                }
                _ => {}
            }
        }
        let now = if let Event::Window(window::Event::RedrawRequested(now)) = event {
            *now
        } else {
            Instant::now()
        };
        state.frame = now;
        let status = self.status(state, hit);
        if status.hovered != state.hover.value() {
            state.hover.go_mut(status.hovered, now);
            shell.request_redraw();
        }
        if status.focused != state.focus.value() {
            state.focus.go_mut(status.focused, now);
            shell.request_redraw();
        }
        if state.hover.is_animating(now) || state.focus.is_animating(now) {
            shell.request_redraw();
        }
        if !matches!(event, Event::Window(window::Event::RedrawRequested(_))) && interactive {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let Some(clip) = bounds.intersection(viewport) else {
            return;
        };
        let state = tree.state.downcast_ref::<State>();
        let status = self.status(state, cursor.is_over(clip));
        let hover = state.hover.interpolate(0.0, 1.0, state.frame);
        let focus = state.focus.interpolate(0.0, 1.0, state.frame);
        let paint = |focused| {
            let a = (self.style)(
                theme,
                Status {
                    hovered: false,
                    focused,
                    ..status
                }
                .normalized(),
            );
            let b = (self.style)(
                theme,
                Status {
                    hovered: true,
                    focused,
                    ..status
                }
                .normalized(),
            );
            mix(a, b, hover, inherited.text_color)
        };
        let mut style = mix(paint(false), paint(true), focus, inherited.text_color);
        // Text decoration is discrete in CSS; do not delay it until the fade ends.
        style.underline = (self.style)(theme, status).underline;
        let text_color: Color = style
            .text_color
            .map(Into::into)
            .unwrap_or(inherited.text_color);
        if status.interactive
            && !status.disabled
            && focus > 0.0
            && let Some(ring) = style.ring
        {
            let spread = ring.width + style.ring_offset;
            let mut border: iced_core::Border = ring.into();
            border.radius = Radius {
                top_left: style.border.radius.top_left + spread,
                top_right: style.border.radius.top_right + spread,
                bottom_right: style.border.radius.bottom_right + spread,
                bottom_left: style.border.radius.bottom_left + spread,
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: bounds.expand(spread),
                    border,
                    snap: style.snap,
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                // CSS backgrounds extend under the border. Iced subtracts
                // even a transparent border from its background geometry, so
                // paint the full capsule first and then overlay the stroke.
                border: iced_core::Border {
                    radius: style.border.radius.into(),
                    ..iced_core::Border::default()
                },
                shadow: style.shadow.into(),
                snap: style.snap,
            },
            style
                .background
                .map(Background::from)
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );
        if style.border.width > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border.into(),
                    snap: style.snap,
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }
        renderer.with_layer(clip, |renderer| {
            let (child_layout, child) = layout.iter(&tree.children).next().expect("Badge content");
            self.content.draw(
                child,
                renderer,
                theme,
                &renderer::Style { text_color },
                child_layout,
                cursor,
                &clip,
            );
            if style.underline {
                for text in &state.text {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: bounds.x + text.x,
                                y: bounds.y + text.y + text.height,
                                width: text.width,
                                height: 1.0,
                            },
                            ..renderer::Quad::default()
                        },
                        text_color,
                    );
                }
            }
        });
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
            return mouse::Interaction::default();
        }
        if self.on_press.is_some() && cursor.is_over(layout.bounds()) && cursor.is_over(*viewport) {
            return mouse::Interaction::Pointer;
        }
        let (child_layout, child) = layout.iter(&tree.children).next().expect("Badge content");
        self.content
            .mouse_interaction(child, child_layout, cursor, viewport, renderer)
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
        let (child_layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("Badge content");
        self.content
            .overlay(child, child_layout, renderer, viewport, translation, window)
    }
}

struct TextBounds<'a>(&'a mut Vec<Rectangle>);

impl Operation for TextBounds<'_> {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn text(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn operation::Text) {
        self.0.push(bounds);
    }
}

fn mix(a: Style, b: Style, t: f32, inherited: Color) -> Style {
    let t = t.clamp(0.0, 1.0);
    let color = |a: icedxdy_core::Color, b: icedxdy_core::Color| a.blend(b, t);
    let background = match (a.background, b.background) {
        (Some(icedxdy_core::Background::Color(a)), Some(icedxdy_core::Background::Color(b))) => {
            Some(color(a, b).into())
        }
        (None, Some(icedxdy_core::Background::Color(b))) => Some(color(b.alpha(0.0), b).into()),
        (Some(icedxdy_core::Background::Color(a)), None) => Some(color(a, a.alpha(0.0)).into()),
        _ => {
            if t < 0.5 {
                a.background
            } else {
                b.background
            }
        }
    };
    let ring = match (a.ring, b.ring) {
        (None, None) => None,
        (a, b) => {
            let base = b.or(a).expect("one ring");
            let absent = icedxdy_core::Border {
                width: 0.0,
                color: base.color.alpha(0.0),
                ..base
            };
            let a = a.unwrap_or(absent);
            let b = b.unwrap_or(absent);
            Some(icedxdy_core::Border {
                width: a.width + (b.width - a.width) * t,
                color: color(a.color, b.color),
                ..b
            })
        }
    };
    Style {
        background,
        text_color: Some(color(
            a.text_color.unwrap_or_else(|| inherited.into()),
            b.text_color.unwrap_or_else(|| inherited.into()),
        )),
        border: icedxdy_core::Border {
            color: color(a.border.color, b.border.color),
            width: a.border.width + (b.border.width - a.border.width) * t,
            ..b.border
        },
        ring,
        ring_offset: a.ring_offset + (b.ring_offset - a.ring_offset) * t,
        ..b
    }
}
