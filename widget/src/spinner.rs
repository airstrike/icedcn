// Circular spinner widget.
//
// Based on the loading_spinners example from iced by Héctor Ramón Jiménez (hecrj).
// Original: https://github.com/iced-rs/iced/tree/master/examples/loading_spinners
// License: MIT

use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::widget::Meta;
use iced_core::widget::tree::{self, Tree};
use iced_core::{
    Color, Element, Event, Layout, Length, Point, Radians, Rectangle, Shell, Size, Vector, Widget,
};
use iced_widget::Renderer;
use iced_widget::canvas;

use icedcn_core::Theme;

use std::f32::consts::PI;
use std::time::Duration;

/// The style of a [`Spinner`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    /// The track color (background circle). `None` = transparent.
    pub track_color: Option<Color>,
    /// The bar color (spinning arc). `None` = inherit parent text color.
    pub bar_color: Option<Color>,
}

/// The theme catalog of a [`Spinner`].
pub trait Catalog {
    /// The style class.
    type Class<'a>;

    /// The default class.
    fn default<'a>() -> Self::Class<'a>;

    /// The style for a class.
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// A styling function for a [`Spinner`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// Default spinner style — inherits parent text color.
pub fn default(_theme: &Theme) -> Style {
    Style::default()
}

/// A circular loading spinner.
///
/// ```ignore
/// use icedcn::widget::spinner;
///
/// spinner()
/// spinner().size(24).bar_height(2.0)
/// ```
pub struct Spinner<'a, Theme = icedcn_core::Theme>
where
    Theme: Catalog,
{
    size: f32,
    bar_height: Option<f32>,
    rotation_duration: Duration,
    shape: Shape,
    class: Theme::Class<'a>,
}

#[derive(Clone, Copy)]
enum Shape {
    Arc,
    Circle,
    Spokes,
}

impl<'a> Spinner<'a> {
    pub fn new() -> Self {
        Self {
            size: 16.0,
            bar_height: None,
            rotation_duration: Duration::from_millis(1000),
            shape: Shape::Arc,
            class: <Theme as Catalog>::default(),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Draw Lucide's loader-circle: a 288-degree arc with a radius of
    /// 9/24 of the size. The stroke scales as 2/24 of the size unless
    /// explicitly set with [`Self::bar_height`].
    ///
    /// The last call to `circle` or `spokes` selects the glyph.
    pub fn circle(mut self) -> Self {
        self.shape = Shape::Circle;
        self
    }

    /// Draw the eight radial strokes of the Lucide loader glyph.
    pub fn spokes(mut self) -> Self {
        self.shape = Shape::Spokes;
        self
    }

    pub fn bar_height(mut self, bar_height: f32) -> Self {
        self.bar_height = Some(bar_height);
        self
    }

    pub fn rotation_duration(mut self, duration: Duration) -> Self {
        self.rotation_duration = duration;
        self
    }

    /// Set the style.
    ///
    /// Accepts any value that converts into the spinner's [`StyleFn`] —
    /// typically `Box::new(closure)` for ad-hoc styles.
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self {
        self.class = style.into();
        self
    }
}

impl Default for Spinner<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a [`Spinner`] with default settings.
pub fn spinner<'a>() -> Spinner<'a> {
    Spinner::new()
}

// --- Animation ---

#[derive(Clone, Copy)]
struct Animation {
    rotation: u32,
    last: iced_core::time::Instant,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            rotation: 0,
            last: iced_core::time::Instant::now(),
        }
    }
}

impl Animation {
    fn advance(&mut self, rotation_duration: Duration, now: iced_core::time::Instant) {
        let dt = (now - self.last).as_secs_f32();
        let additional = (dt / rotation_duration.as_secs_f32() * u32::MAX as f32) as u32;
        self.rotation = self.rotation.wrapping_add(additional);
        self.last = now;
    }

    fn angle(&self) -> f32 {
        self.rotation as f32 / u32::MAX as f32
    }
}

#[derive(Default)]
struct State {
    animation: Animation,
    cache: canvas::Cache,
}

// --- Widget impl ---

impl Meta for Spinner<'_> {}

impl<'a, Message: 'a> Widget<Message, Theme, Renderer> for Spinner<'a> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(self.size),
            height: Length::Fixed(self.size),
        }
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) {
        tree.size = layout::atomic(limits, self.size, self.size);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        if let Event::Window(iced_core::window::Event::RedrawRequested(now)) = event {
            // Only animate while on-screen. A spinner self-drives its repaint
            // by requesting the next frame on every RedrawRequested; gating
            // that on viewport intersection means a spinner scrolled out of a
            // scrollable (or otherwise off-screen) stops driving redraws
            // instead of repainting the whole window for a frame nobody sees.
            // It resumes when scrolled back in: that scroll emits its own
            // redraw, during which the spinner sees it's visible and re-enters
            // the loop. (`viewport` here is the scrollable's visible window in
            // content space; `bounds` are the content-natural positions — the
            // same culling iced's own `draw` performs.)
            //
            // This only removes the OFF-SCREEN waste. A visible spinner still
            // repaints the whole window each frame — iced has no damage
            // tracking, so partial repaint is the real fix and isn't here.
            if !layout.bounds().intersects(viewport) {
                return;
            }
            state.animation.advance(self.rotation_duration, *now);
            state.cache.clear();
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
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        use iced_core::Renderer as _;

        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let catalog_style = theme.style(&self.class);

        let bar_color = catalog_style.bar_color.unwrap_or(style.text_color);
        let track_color = catalog_style.track_color.unwrap_or(Color::TRANSPARENT);

        let geometry = state.cache.draw(renderer, bounds.size(), |frame| {
            let bar_height = self.bar_height.unwrap_or(match self.shape {
                Shape::Circle => frame.width() / 12.0,
                Shape::Arc | Shape::Spokes => 2.0,
            });
            let track_radius = match self.shape {
                Shape::Circle => frame.width() * 3.0 / 8.0,
                Shape::Arc | Shape::Spokes => frame.width() / 2.0 - bar_height,
            };
            let track_path = canvas::Path::circle(frame.center(), track_radius);

            frame.stroke(
                &track_path,
                canvas::Stroke::default()
                    .with_color(track_color)
                    .with_width(bar_height),
            );

            let angle = state.animation.angle() * 2.0 * PI;
            let mut builder = canvas::path::Builder::new();
            if matches!(self.shape, Shape::Spokes) {
                let center = frame.center();
                for i in 0..8 {
                    let theta = angle + PI * i as f32 / 4.0;
                    let point =
                        |r: f32| Point::new(center.x + theta.cos() * r, center.y + theta.sin() * r);
                    builder.move_to(point(track_radius * 0.65));
                    builder.line_to(point(track_radius));
                }
            } else {
                let start = Radians(angle);
                builder.arc(canvas::path::Arc {
                    center: frame.center(),
                    radius: track_radius,
                    start_angle: start,
                    end_angle: start
                        + Radians(
                            PI * if matches!(self.shape, Shape::Circle) {
                                1.6
                            } else {
                                1.75
                            },
                        ),
                });
            }
            frame.stroke(
                &builder.build(),
                canvas::Stroke::default()
                    .with_line_cap(canvas::LineCap::Round)
                    .with_color(bar_color)
                    .with_width(bar_height),
            );
        });

        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            use iced_widget::graphics::geometry::Renderer as _;
            renderer.draw_geometry(geometry);
        });
    }
}

impl<'a, Message: 'a> From<Spinner<'a>> for Element<'a, Message, Theme, Renderer> {
    fn from(spinner: Spinner<'a>) -> Self {
        spinner._boxed()
    }
}

impl<'a, Message: 'a> From<Spinner<'a>> for crate::View<'a, Message> {
    fn from(s: Spinner<'a>) -> Self {
        crate::View::from(Element::from(s))
    }
}
