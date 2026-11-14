use crate::animation::cubic_bezier;
use iced_core::animation::Easing;
use iced_core::time::Instant;
use iced_core::{Animation, Background, window};
use std::f32::consts::{FRAC_PI_2, PI};
use std::time::Duration;

use iced_core::widget::{Meta, Operation, Tree, operation, tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Renderer as _, Shell, Size, Vector, Widget, layout,
    mouse, overlay, renderer,
};
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme::container::Style;

/// The root owns paint and dispatch: independent actions receive input before
/// the full-card target. Its layout remains the content's natural size.
pub(super) struct Surface<'a, Message> {
    pub content: Element<'a, Message, Theme, Renderer>,
    pub trigger: Option<Element<'a, Message, Theme, Renderer>>,
    pub style: Box<dyn Fn(&Theme) -> Style + 'a>,
    pub idle: bool,
}
pub(super) struct State {
    focused: bool,
    hover: Animation<bool>,
    frame: Instant,
}
impl Default for State {
    fn default() -> Self {
        Self {
            focused: false,
            frame: Instant::now(),
            hover: Animation::new(false)
                .duration(Duration::from_millis(150))
                .easing(Easing::Custom(|t| cubic_bezier(0.4, 0., 0.2, 1., t))),
        }
    }
}
impl<Message> Meta for Surface<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Surface<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        let mut children = vec![&mut self.content];
        if let Some(trigger) = &mut self.trigger {
            children.push(trigger);
        }
        tree.diff_children(&mut children);
    }
    fn size(&self) -> Size<Length> {
        self.content.size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
        if let Some(trigger) = &mut self.trigger {
            trigger.layout(
                &mut tree.children[1],
                renderer,
                &layout::Limits::new(tree.size, tree.size),
            );
        }
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.custom(None, layout.bounds(), tree.state.downcast_mut::<State>());
        let mut children = layout.iter_mut(&mut tree.children);
        let (content_layout, content_tree) = children.next().expect("attachment content");
        if let Some(trigger) = &mut self.trigger {
            let (child, state) = children.next().expect("attachment trigger");
            trigger.operate(state, child, viewport, renderer, operation);
        }
        self.content
            .operate(content_tree, content_layout, viewport, renderer, operation);
        drop(children);
        let mut focused = Focus(false);
        let mut children = layout.iter_mut(&mut tree.children);
        let (child, state) = children.next().expect("attachment content");
        self.content
            .operate(state, child, viewport, renderer, &mut focused);
        if let Some(trigger) = &mut self.trigger {
            let (child, state) = children.next().expect("attachment trigger");
            trigger.operate(state, child, viewport, renderer, &mut focused);
        }
        tree.state.downcast_mut::<State>().focused = focused.0;
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
        let mut children = layout.iter_mut(&mut tree.children);
        let (child, state) = children.next().expect("attachment content");
        self.content
            .update(state, event, child, cursor, renderer, shell, viewport);
        if !shell.is_event_captured()
            && let Some(trigger) = &mut self.trigger
        {
            let (child, state) = children.next().expect("attachment trigger");
            trigger.update(state, event, child, cursor, renderer, shell, viewport);
        }
        drop(children);
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<State>();
            state.frame = *now;
            let hovered = self.trigger.is_some() && cursor.is_over(layout.bounds());
            if state.hover.value() != hovered {
                state.hover.go_mut(hovered, *now);
            }
            if state.hover.is_animating(*now) {
                shell.request_redraw();
            }
        }
        let previous = tree.state.downcast_ref::<State>().focused;
        self.operate(tree, layout, viewport, renderer, &mut Focus(false));
        if previous != tree.state.downcast_ref::<State>().focused {
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
        let style = (self.style)(theme);
        let inherited = renderer::Style {
            text_color: style
                .text_color
                .map(Into::into)
                .unwrap_or(inherited.text_color),
        };
        let border: iced_core::Border = style.border.into();
        let state = tree.state.downcast_ref::<State>();
        let base: Background = style
            .background
            .map(Into::into)
            .unwrap_or(iced_core::Color::TRANSPARENT.into());
        let fill = match base {
            Background::Color(color) => Background::Color(state.hover.interpolate(
                color,
                theme.muted.base.alpha(0.5).into(),
                state.frame,
            )),
            background => background,
        };
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced_core::Border {
                    width: if self.idle { 0. } else { border.width },
                    ..border
                },
                ..renderer::Quad::default()
            },
            fill,
        );
        if self.idle {
            // Dash along the rounded perimeter, retaining the same border inset.
            let radius = border.radius.top_left.min(bounds.height / 2.);
            let straight_x = bounds.width - radius * 2.;
            let straight_y = bounds.height - radius * 2.;
            let arc = FRAC_PI_2 * radius;
            let perimeter = 2. * (straight_x + straight_y) + 4. * arc;
            let point = |mut d: f32| {
                for (cx, cy, angle, length, dx, dy) in [
                    (bounds.x + radius, bounds.y + 0.5, 0., straight_x, 1., 0.),
                    (
                        bounds.x + bounds.width - radius,
                        bounds.y + radius,
                        -FRAC_PI_2,
                        arc,
                        0.,
                        0.,
                    ),
                    (
                        bounds.x + bounds.width - 0.5,
                        bounds.y + radius,
                        0.,
                        straight_y,
                        0.,
                        1.,
                    ),
                    (
                        bounds.x + bounds.width - radius,
                        bounds.y + bounds.height - radius,
                        0.,
                        arc,
                        0.,
                        0.,
                    ),
                    (
                        bounds.x + bounds.width - radius,
                        bounds.y + bounds.height - 0.5,
                        0.,
                        straight_x,
                        -1.,
                        0.,
                    ),
                    (
                        bounds.x + radius,
                        bounds.y + bounds.height - radius,
                        FRAC_PI_2,
                        arc,
                        0.,
                        0.,
                    ),
                    (
                        bounds.x + 0.5,
                        bounds.y + bounds.height - radius,
                        0.,
                        straight_y,
                        0.,
                        -1.,
                    ),
                    (bounds.x + radius, bounds.y + radius, PI, arc, 0., 0.),
                ] {
                    if d <= length {
                        return if dx != 0. || dy != 0. {
                            (cx + dx * d, cy + dy * d)
                        } else {
                            let a = angle + d / radius;
                            (cx + (radius - 0.5) * a.cos(), cy + (radius - 0.5) * a.sin())
                        };
                    }
                    d -= length;
                }
                (bounds.x + radius, bounds.y + 0.5)
            };
            for i in 0..perimeter.ceil() as usize {
                if i % 6 < 3 {
                    let (x, y) = point(i as f32);
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: x - 0.5,
                                y: y - 0.5,
                                width: 1.,
                                height: 1.,
                            },
                            ..renderer::Quad::default()
                        },
                        border.color,
                    );
                }
            }
        }
        if tree.state.downcast_ref::<State>().focused {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: bounds.expand(1.),
                    border: iced_core::Border {
                        color: theme.neutral.ring.alpha(0.5).into(),
                        width: 1.,
                        radius: border.radius,
                    },
                    ..renderer::Quad::default()
                },
                iced_core::Color::TRANSPARENT,
            );
        }
        let mut children = layout.iter(&tree.children);
        let (child, state) = children.next().expect("attachment content");
        self.content
            .draw(state, renderer, theme, &inherited, child, cursor, viewport);
        if let Some(trigger) = &self.trigger {
            let (child, state) = children.next().expect("attachment trigger");
            trigger.draw(state, renderer, theme, &inherited, child, cursor, viewport);
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
        let mut children = layout.iter(&tree.children);
        let (child, state) = children.next().expect("attachment content");
        let content = self
            .content
            .mouse_interaction(state, child, cursor, viewport, renderer);
        if let Some(trigger) = &self.trigger {
            let (child, state) = children.next().expect("attachment trigger");
            content.max(trigger.mouse_interaction(state, child, cursor, viewport, renderer))
        } else {
            content
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
        let mut children = layout.iter_mut(&mut tree.children);
        let (child, state) = children.next().expect("attachment content");
        let mut overlays =
            self.content
                .overlay(state, child, renderer, viewport, translation, window);
        if let Some(trigger) = &mut self.trigger {
            let (child, state) = children.next().expect("attachment trigger");
            overlays.extend(trigger.overlay(state, child, renderer, viewport, translation, window));
        }
        overlays
    }
}
struct Focus(bool);
impl Operation for Focus {
    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Operation)) {
        f(self);
    }
    fn focusable(
        &mut self,
        _: Option<&iced_core::widget::Id>,
        _: Rectangle,
        state: &mut dyn operation::Focusable,
    ) {
        self.0 |= state.is_focused();
    }
}
