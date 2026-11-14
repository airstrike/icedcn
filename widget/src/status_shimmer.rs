//! Highlight a moving band of an activity label without rebuilding its paragraph.
use iced_core::Renderer as _;
use iced_core::Widget;
use iced_core::time::Instant;
use iced_core::widget::{Meta, Operation, Tree, tree};
use iced_core::{Event, Layout, Length, Rectangle, Shell, Size, layout, mouse, renderer, window};
use iced_widget::Widget as _;
use iced_widget::{Renderer, text};
use icedcn_core::Theme;

use crate::Slot;

/// A status label with a soft highlight that travels across the glyphs.
pub fn shimmer<'a, Message: 'a>(label: impl Into<String>) -> Slot<'a, Message> {
    Slot::from(
        Shimmer {
            text: text(label.into()).size(14).selectable(true),
            period: 2.2,
            foreground: false,
        }
        .boxed(),
    )
}
/// Animate an attachment title with a two-second sweep over its existing text style.
pub(crate) fn title<'a, Message: 'a>(text: iced_widget::Text<'a, Theme>) -> Slot<'a, Message> {
    Shimmer {
        text,
        period: 2.,
        foreground: true,
    }
    .boxed()
    .into()
}
struct Shimmer<'a> {
    text: iced_widget::Text<'a, Theme>,
    period: f32,
    foreground: bool,
}
#[derive(Default)]
struct Phase {
    start: Option<Instant>,
    offset: f32,
}
impl Meta for Shimmer<'_> {}
impl<Message> Widget<Message, Theme, Renderer> for Shimmer<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Phase>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(Phase::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children::<Message, Theme, Renderer>(std::slice::from_mut(&mut self.text));
    }
    fn size(&self) -> Size<Length> {
        <iced_widget::Text<Theme> as Widget<Message, Theme, Renderer>>::size(&self.text)
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        <iced_widget::Text<Theme> as Widget<Message, Theme, Renderer>>::layout(
            &mut self.text,
            &mut tree.children[0],
            renderer,
            limits,
        );
        tree.size = tree.children[0].size;
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        _: mouse::Cursor,
        _: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event
            && layout.bounds().intersects(viewport)
        {
            let state = tree.state.downcast_mut::<Phase>();
            let start = *state.start.get_or_insert(*now);
            state.offset = now.duration_since(start).as_secs_f32() % self.period / self.period;
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
        let Some((child, tree_ref)) = layout.iter(&tree.children).next() else {
            return;
        };
        let bounds = child.bounds();
        let paint = |renderer: &mut Renderer, color| {
            <iced_widget::Text<Theme> as Widget<Message, Theme, Renderer>>::draw(
                &self.text,
                tree_ref,
                renderer,
                theme,
                &renderer::Style { text_color: color },
                child,
                cursor,
                viewport,
            )
        };
        let offset = tree.state.downcast_ref::<Phase>().offset;
        if self.foreground {
            let spread = (bounds.width * 0.25).max(1.);
            let center = bounds.x - spread + (bounds.width + spread * 2.) * offset;
            for column in 0..bounds.width.ceil() as usize {
                let x = bounds.x + column as f32;
                let alpha = 0.2 + 0.8 * ((x + 0.5 - center).abs() / spread).min(1.);
                let strip = Rectangle {
                    x,
                    y: bounds.y,
                    width: 1.,
                    height: bounds.height,
                };
                if let Some(clip) = strip
                    .intersection(viewport)
                    .and_then(|r| r.intersection(&bounds))
                {
                    renderer.with_layer(clip, |renderer| {
                        paint(renderer, style.text_color.scale_alpha(alpha))
                    });
                }
            }
            return;
        }
        paint(renderer, theme.muted.foreground.into());
        let radius = 20.;
        let center = bounds.x - radius + (bounds.width + radius * 2.) * offset;
        // Successively narrower bands soften the two edges of the highlight.
        for (fraction, alpha) in [(1., 0.18), (0.7, 0.38), (0.4, 0.65)] {
            let band = Rectangle {
                x: center - radius * fraction,
                y: bounds.y,
                width: radius * fraction * 2.,
                height: bounds.height,
            };
            if let Some(clip) = band
                .intersection(viewport)
                .and_then(|r| r.intersection(&bounds))
            {
                renderer.with_layer(clip, |renderer| {
                    paint(
                        renderer,
                        theme
                            .muted
                            .foreground
                            .blend(theme.neutral.foreground, alpha)
                            .into(),
                    )
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
        op: &mut dyn Operation,
    ) {
        if let Some((child, state)) = layout.iter_mut(&mut tree.children).next() {
            <iced_widget::Text<Theme> as Widget<Message, Theme, Renderer>>::operate(
                &mut self.text,
                state,
                child,
                viewport,
                renderer,
                op,
            );
        }
    }
}
