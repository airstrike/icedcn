//! Native iced hover composition, extended to reveal actions on content focus.
//! Based on iced widget/hover.rs at fd824c8; kept local to Code Block.

use crate::animation::cubic_bezier;
use crate::core;
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget::operation::{self, focusable};
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::window;
use crate::core::{Event, Length, Rectangle, Shell, Size, Vector, Widget};
use icedxdy_core::Theme;
use std::time::{Duration, Instant};

/// A widget that displays another widget on top of it.
///
/// This works analogously to a stack, but it will only
/// display the layer on top when the cursor is over the base. It can be
/// useful for removing visual clutter.
pub struct Hover<W, V> {
    base: W,
    top: V,
    is_top_focused: bool,
    is_top_overlay_active: bool,
    is_hovered: bool,
}

impl<W, V> Hover<W, V> {
    /// Creates a new [`Hover`] widget with the given base and top widgets.
    pub fn new(base: W, top: V) -> Self {
        Self {
            base,
            top,
            is_top_focused: false,
            is_top_overlay_active: false,
            is_hovered: false,
        }
    }
}

impl<W, V> Meta for Hover<W, V> {}

impl<W, V, Message, Renderer> Widget<Message, Theme, Renderer> for Hover<W, V>
where
    Renderer: core::Renderer,
    W: Widget<Message, Theme, Renderer>,
    V: Widget<Message, Theme, Renderer>,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Fade>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Fade::default())
    }

    fn diff(&mut self, tree: &mut Tree) {
        if tree.children.len() != 2 {
            tree.children = vec![Tree::new(&self.base), Tree::new(&self.top)];
        }

        tree.children[0].diff(&mut self.base);
        tree.children[1].diff(&mut self.top);
    }

    fn size(&self) -> Size<Length> {
        self.base.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.base.layout(&mut tree.children[0], renderer, limits);

        let base_size = tree.children[0].size;

        self.top.layout(
            &mut tree.children[1],
            renderer,
            &layout::Limits::new(Size::ZERO, base_size),
        );

        tree.size = base_size;
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
        if let Some(bounds) = layout.bounds().intersection(viewport) {
            let mut children = layout.iter(&tree.children);

            let (base_layout, base_tree) =
                children.next().expect("two children established by diff");

            self.base.draw(
                base_tree,
                renderer,
                theme,
                style,
                base_layout,
                cursor,
                viewport,
            );

            let opacity = tree.state.downcast_ref::<Fade>().value;
            if opacity > 0. {
                let (top_layout, top_tree) =
                    children.next().expect("two children established by diff");

                let theme = fade_theme(theme, opacity);
                let style = renderer::Style {
                    text_color: style.text_color.scale_alpha(opacity),
                };
                renderer.with_layer(bounds, |renderer| {
                    self.top.draw(
                        top_tree, renderer, &theme, &style, top_layout, cursor, viewport,
                    );
                });
            }
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
        let mut children = layout.iter_mut(&mut tree.children);

        let (base_layout, base_tree) = children.next().expect("two children established by diff");
        let (top_layout, top_tree) = children.next().expect("two children established by diff");

        // Floating controls precede the content in the source composition.
        self.top
            .operate(top_tree, top_layout, viewport, renderer, operation);
        self.base
            .operate(base_tree, base_layout, viewport, renderer, operation);
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

        let (base_layout, base_tree) = children.next().expect("two children established by diff");
        let (top_layout, top_tree) = children.next().expect("two children established by diff");

        let is_hovered = cursor.is_over(layout.bounds());

        if matches!(event, Event::Window(window::Event::RedrawRequested(_))) {
            let mut count_focused = focusable::count();

            self.base.operate(
                base_tree,
                base_layout,
                viewport,
                renderer,
                &mut operation::black_box(&mut count_focused),
            );
            self.top.operate(
                top_tree,
                top_layout,
                viewport,
                renderer,
                &mut operation::black_box(&mut count_focused),
            );

            self.is_top_focused = match count_focused.finish() {
                operation::Outcome::Some(count) => count.focused.is_some(),
                _ => false,
            };

            self.is_hovered = is_hovered;
        } else if is_hovered != self.is_hovered {
            shell.request_redraw();
        }

        let is_visible = is_hovered || self.is_top_focused || self.is_top_overlay_active;
        let fade = tree.state.downcast_mut::<Fade>();
        let now = if let Event::Window(window::Event::RedrawRequested(now)) = event {
            *now
        } else {
            Instant::now()
        };
        fade.target(is_visible, now);
        if fade.advance(now) {
            shell.request_redraw_at(now + Duration::from_millis(16));
        }

        if matches!(
            event,
            Event::Mouse(mouse::Event::CursorMoved { .. } | mouse::Event::ButtonReleased(_))
        ) || is_visible
        {
            let redraw_request = shell.redraw_request();

            self.top.update(
                top_tree, event, top_layout, cursor, renderer, shell, viewport,
            );

            // Ignore redraw requests of invisible content
            if !is_visible {
                Shell::replace_redraw_request(shell, redraw_request);
            }

            if shell.is_event_captured() {
                return;
            }
        };

        self.base.update(
            base_tree,
            event,
            base_layout,
            cursor,
            renderer,
            shell,
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
        let mut children = layout.iter(&tree.children).rev();

        let (top_layout, top_tree) = children.next().expect("two children established by diff");
        let interaction = self
            .top
            .mouse_interaction(top_tree, top_layout, cursor, viewport, renderer);

        if interaction != mouse::Interaction::None {
            return interaction;
        }

        let (base_layout, base_tree) = children.next().expect("two children established by diff");
        self.base
            .mouse_interaction(base_tree, base_layout, cursor, viewport, renderer)
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

        let (base_layout, base_tree) = children.next().expect("two children established by diff");
        let (top_layout, top_tree) = children.next().expect("two children established by diff");

        let base_overlays = self.base.overlay(
            base_tree,
            base_layout,
            renderer,
            viewport,
            translation,
            window,
        );
        let top_overlays = self.top.overlay(
            top_tree,
            top_layout,
            renderer,
            viewport,
            translation,
            window,
        );

        self.is_top_overlay_active = !top_overlays.is_empty();

        base_overlays.into_iter().chain(top_overlays).collect()
    }
}

#[derive(Default)]
struct Fade {
    value: f32,
    from: f32,
    to: f32,
    start: Option<Instant>,
}
impl Fade {
    fn target(&mut self, visible: bool, now: Instant) {
        let target = if visible { 1. } else { 0. };
        if self.to != target {
            self.advance(now);
            self.from = self.value;
            self.to = target;
            self.start = Some(now);
        }
    }
    fn advance(&mut self, now: Instant) -> bool {
        if let Some(start) = self.start {
            let p = (now.saturating_duration_since(start).as_secs_f32() / 0.15).min(1.);
            self.value = self.from + (self.to - self.from) * cubic_bezier(0.4, 0., 0.2, 1., p);
            if p >= 1. {
                self.start = None;
                self.value = self.to;
            }
        }
        self.start.is_some()
    }
}
// Like Motion, fade inherited text and theme-derived paint. Fixed-color artwork
// is outside the theme's opacity contract; no renderer-wide adapter is involved.
fn fade_theme(theme: &Theme, opacity: f32) -> Theme {
    let mut design = theme.design().clone();
    for palette in [
        &mut design.neutral,
        &mut design.primary,
        &mut design.secondary,
        &mut design.muted,
        &mut design.accent,
        &mut design.destructive,
        &mut design.info,
        &mut design.success,
        &mut design.warning,
        &mut design.card,
        &mut design.popover,
        &mut design.code,
    ] {
        palette.base = palette.base.scale_alpha(opacity);
        palette.foreground = palette.foreground.scale_alpha(opacity);
        palette.border = palette.border.scale_alpha(opacity);
        palette.ring = palette.ring.scale_alpha(opacity);
        palette.tint_text = palette.tint_text.scale_alpha(opacity);
    }
    design.input = design.input.scale_alpha(opacity);
    Theme::new(design)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fade_interpolates_reverses_and_reaches_endpoints() {
        let now = Instant::now();
        let mut fade = Fade::default();
        fade.target(true, now);
        assert_eq!(fade.value, 0.);
        assert!(fade.advance(now + Duration::from_millis(75)));
        let middle = fade.value;
        assert!(middle > 0. && middle < 1.);
        fade.target(false, now + Duration::from_millis(75));
        assert_eq!(fade.value, middle);
        assert!(fade.advance(now + Duration::from_millis(150)));
        assert!(fade.value > 0. && fade.value < middle);
        assert!(!fade.advance(now + Duration::from_millis(225)));
        assert_eq!(fade.value, 0.);
        fade.target(true, now + Duration::from_millis(225));
        assert!(!fade.advance(now + Duration::from_millis(375)));
        assert_eq!(fade.value, 1.);
        for theme in [Theme::light(), Theme::dark()] {
            let faded = fade_theme(&theme, 0.25);
            assert_eq!(
                faded.neutral.foreground.a,
                theme.neutral.foreground.a * 0.25
            );
            assert_eq!(faded.neutral.base.a, theme.neutral.base.a * 0.25);
            assert_eq!(faded.muted.base.a, theme.muted.base.a * 0.25);
        }
    }
    #[test]
    fn fade_state_survives_widget_rebuild() {
        use crate::core::Element;
        use iced_widget::{Renderer, Space};
        let view = || -> Element<'static, (), Theme, Renderer> {
            Hover::new(Space::new(), Space::new())._boxed()
        };
        let mut original = view();
        let mut tree = Tree::new(&mut original);
        let now = Instant::now();
        let fade = tree.state.downcast_mut::<Fade>();
        fade.target(true, now);
        fade.advance(now + Duration::from_millis(75));
        let midpoint = fade.value;
        let mut rebuilt = view();
        tree.diff(&mut rebuilt);
        assert_eq!(tree.state.downcast_ref::<Fade>().value, midpoint);
    }
}
