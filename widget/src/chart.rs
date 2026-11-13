//! Grouped bars with optional grid lines, x-axis labels, legend and a tooltip.
//! Keyboard navigation and touch expose the same data as pointer hover.
use crate::compose::{Compose, Slot};
use iced_core::alignment::Vertical;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::text::{self, Renderer as _};
use iced_core::widget::{Meta, Operation, Tree, operation::Focusable, tree};
use iced_core::{
    Color, Element, Event, Layout, Length, Point, Rectangle, Renderer as _, Shell, Size, Widget,
    border, layout, mouse, renderer, touch,
};
use iced_widget::Renderer;
use icedxdy_core::Theme;

#[derive(Debug, Clone, Copy)]
pub struct Series<'a> {
    pub name: &'a str,
    pub color: Color,
    pub values: &'a [f32],
}
impl<'a> Series<'a> {
    pub fn new(name: &'a str, color: Color, values: &'a [f32]) -> Self {
        Self {
            name,
            color,
            values,
        }
    }
}
pub struct Chart<'a> {
    labels: &'a [&'a str],
    series: Vec<Series<'a>>,
    grid: bool,
    axis: bool,
    legend: bool,
    tooltip: bool,
    height: Length,
    radius: f32,
    maximum: f32,
}
pub fn chart<'a>(labels: &'a [&'a str], series: impl IntoIterator<Item = Series<'a>>) -> Chart<'a> {
    let series: Vec<_> = series.into_iter().collect();
    let max = series
        .iter()
        .flat_map(|s| s.values.iter().take(labels.len()))
        .copied()
        .filter(|v| v.is_finite())
        .fold(0.0, f32::max);
    let power = 10_f32.powf(max.max(1.0).log10().floor());
    let maximum = (max / power).ceil().max(1.0) * power;
    Chart {
        labels,
        series,
        grid: false,
        axis: false,
        legend: false,
        tooltip: false,
        height: Length::Fixed(200.0),
        radius: 4.0,
        maximum,
    }
}
impl Chart<'_> {
    pub fn grid(mut self, visible: bool) -> Self {
        self.grid = visible;
        self
    }
    pub fn axis(mut self, visible: bool) -> Self {
        self.axis = visible;
        self
    }
    pub fn legend(mut self, visible: bool) -> Self {
        self.legend = visible;
        self
    }
    pub fn tooltip(mut self, visible: bool) -> Self {
        self.tooltip = visible;
        self
    }
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius.max(0.0);
        self
    }
    fn plot(&self, b: Rectangle) -> Rectangle {
        Rectangle {
            x: b.x + 8.0,
            y: b.y + 6.0,
            width: (b.width - 16.0).max(0.0),
            height: (b.height
                - 12.0
                - if self.axis { 22.0 } else { 0.0 }
                - if self.legend { 24.0 } else { 0.0 })
            .max(0.0),
        }
    }
    fn index(&self, p: Point, plot: Rectangle) -> Option<usize> {
        if self.labels.is_empty() || plot.width <= 0.0 || !plot.contains(p) {
            return None;
        }
        Some(
            (((p.x - plot.x) / plot.width * self.labels.len() as f32) as usize)
                .min(self.labels.len() - 1),
        )
    }
}
impl<'a, Message: 'a> Compose<Message> for Chart<'a> {
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self)._boxed()
    }
}
impl<'a, Message: 'a> From<Chart<'a>> for Slot<'a, Message> {
    fn from(chart: Chart<'a>) -> Self {
        Slot::new(chart)
    }
}
impl<'a, Message: 'a> From<Chart<'a>> for Element<'a, Message, Theme, Renderer> {
    fn from(chart: Chart<'a>) -> Self {
        chart._boxed()
    }
}
#[derive(Default)]
struct State {
    hover: Option<usize>,
    key: Option<usize>,
    focus: bool,
}
impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focus
    }
    fn focus(&mut self) {
        self.focus = true;
        self.key.get_or_insert(0);
    }
    fn unfocus(&mut self) {
        self.focus = false;
        self.key = None;
    }
}
impl Meta for Chart<'_> {}

fn label(
    renderer: &mut Renderer,
    content: impl Into<String>,
    position: Point,
    align: text::Alignment,
    color: Color,
    clip: Rectangle,
) {
    renderer.fill_text(
        text::Text {
            content: content.into(),
            bounds: clip.size(),
            size: 12.0.into(),
            line_height: 1.0.into(),
            font: renderer.font(),
            align_x: align,
            align_y: Vertical::Center,
            shaping: text::Shaping::Advanced,
            wrapping: text::Wrapping::None,
            ellipsis: text::Ellipsis::None,
            letter_spacing: Default::default(),
            font_features: Vec::new(),
            font_variations: Vec::new(),
            weight: None,
            hint_factor: None,
        },
        position,
        color,
        clip,
    );
}
fn rect(renderer: &mut Renderer, bounds: Rectangle, color: Color, radius: f32) {
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border: border::rounded(radius),
            ..Default::default()
        },
        color,
    );
}
impl<Message> Widget<Message, Theme, Renderer> for Chart<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, self.height)
    }
    fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
        tree.size = limits.resolve(Length::Fill, self.height, Size::new(200.0, 200.0));
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        _: &Rectangle,
        _: &Renderer,
        op: &mut dyn Operation,
    ) {
        if self.tooltip {
            op.focusable(None, layout.bounds(), tree.state.downcast_mut::<State>());
        }
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        _: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if !self.tooltip || self.labels.is_empty() {
            return;
        }
        let plot = self.plot(layout.bounds());
        let s = tree.state.downcast_mut::<State>();
        let previous = (s.hover, s.key, s.focus);
        let at = |p: Point| {
            if viewport.contains(p) {
                self.index(p, plot)
            } else {
                None
            }
        };
        match event {
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                s.hover = at(*position);
                s.key = None;
            }
            Event::Mouse(mouse::Event::CursorLeft) => s.hover = None,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                s.hover = cursor.position().and_then(at);
                s.focus = s.hover.is_some();
                s.key = None;
            }
            Event::Touch(
                touch::Event::FingerPressed { position, .. }
                | touch::Event::FingerMoved { position, .. },
            ) => {
                s.hover = at(*position);
                if s.hover.is_some() {
                    s.focus = true;
                    s.key = None;
                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(k), ..
            }) if s.focus => {
                let current = s.key.or(s.hover).unwrap_or(0);
                let last = self.labels.len() - 1;
                s.key = match k {
                    Named::ArrowRight => Some((current + 1).min(last)),
                    Named::ArrowLeft => Some(current.saturating_sub(1)),
                    Named::Home => Some(0),
                    Named::End => Some(last),
                    Named::Escape => None,
                    _ => return,
                };
                s.hover = None;
                shell.capture_event();
            }
            _ => {}
        }
        if previous != (s.hover, s.key, s.focus) {
            shell.request_redraw();
        }
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _: &renderer::Style,
        layout: Layout,
        _: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let b = layout.bounds();
        let Some(clip) = viewport.intersection(&b) else {
            return;
        };
        let plot = self.plot(b);
        if plot.width <= 0.0
            || plot.height <= 0.0
            || self.labels.is_empty()
            || self.series.is_empty()
        {
            return;
        }
        let muted = theme.muted.foreground.into();
        let fg = theme.neutral.foreground.into();
        let group = plot.width / self.labels.len() as f32;
        let gap = if self.series.len() > 1 {
            (group * 0.04).min(4.0)
        } else {
            0.0
        };
        let total = group * 0.8;
        let bar =
            ((total - gap * (self.series.len() - 1) as f32) / self.series.len() as f32).max(0.0);
        renderer.with_layer(clip, |renderer| {
            if self.grid {
                for i in 0..=4 {
                    rect(
                        renderer,
                        Rectangle {
                            x: plot.x,
                            y: plot.y + plot.height * i as f32 / 4.0,
                            width: plot.width,
                            height: 1.0,
                        },
                        theme.neutral.border.into(),
                        0.0,
                    );
                }
            }
            let state = tree.state.downcast_ref::<State>();
            let active = state
                .key
                .or(state.hover)
                .filter(|&i| i < self.labels.len() && self.tooltip);
            if let Some(i) = active {
                rect(
                    renderer,
                    Rectangle {
                        x: plot.x + group * i as f32,
                        width: group,
                        ..plot
                    },
                    theme.muted.base.into(),
                    2.0,
                );
            }
            let stride = (42.0 / group).ceil().max(1.0) as usize;
            for (i, &tick) in self.labels.iter().enumerate() {
                let x = plot.x + group * i as f32;
                for (j, series) in self.series.iter().enumerate() {
                    let Some(&v) = series.values.get(i) else {
                        continue;
                    };
                    if !v.is_finite() || v <= 0.0 {
                        continue;
                    }
                    let h = (v / self.maximum) * plot.height;
                    rect(
                        renderer,
                        Rectangle {
                            x: x + (group - total) * 0.5 + (bar + gap) * j as f32,
                            y: plot.y + plot.height - h,
                            width: bar,
                            height: h,
                        },
                        series.color,
                        self.radius.min(bar * 0.5),
                    );
                }
                if self.axis && i % stride == 0 {
                    label(
                        renderer,
                        tick,
                        Point::new(x + group * 0.5, plot.y + plot.height + 12.0),
                        text::Alignment::Center,
                        muted,
                        clip,
                    );
                }
            }
            if self.legend {
                let lengths: Vec<_> = self
                    .series
                    .iter()
                    .map(|s| s.name.chars().count() as f32 * 7.0 + 24.0)
                    .collect();
                let mut x = b.center_x() - lengths.iter().sum::<f32>() / 2.0;
                let y = b.y + b.height - 12.0;
                for (s, w) in self.series.iter().zip(lengths) {
                    rect(
                        renderer,
                        Rectangle {
                            x,
                            y: y - 4.0,
                            width: 8.0,
                            height: 8.0,
                        },
                        s.color,
                        2.0,
                    );
                    label(
                        renderer,
                        s.name,
                        Point::new(x + 12.0, y),
                        text::Alignment::Left,
                        fg,
                        clip,
                    );
                    x += w;
                }
            }
            if let Some(i) = active {
                let w = 145.0_f32.min(b.width);
                let h = 28.0 + 19.0 * self.series.len() as f32;
                let x = (plot.x + group * (i + 1) as f32 + 6.0)
                    .min(b.x + b.width - w)
                    .max(b.x);
                let y = plot.y + 8.0;
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x,
                            y,
                            width: w,
                            height: h,
                        },
                        border: border::rounded(6).width(1).color(theme.neutral.border),
                        ..Default::default()
                    },
                    theme.neutral.base,
                );
                label(
                    renderer,
                    self.labels[i],
                    Point::new(x + 9.0, y + 13.0),
                    text::Alignment::Left,
                    fg,
                    clip,
                );
                for (j, s) in self.series.iter().enumerate() {
                    let row_y = y + 31.0 + j as f32 * 19.0;
                    rect(
                        renderer,
                        Rectangle {
                            x: x + 9.0,
                            y: row_y - 4.0,
                            width: 8.0,
                            height: 8.0,
                        },
                        s.color,
                        2.0,
                    );
                    label(
                        renderer,
                        s.name,
                        Point::new(x + 23.0, row_y),
                        text::Alignment::Left,
                        muted,
                        clip,
                    );
                    if let Some(value) = s.values.get(i) {
                        label(
                            renderer,
                            value.to_string(),
                            Point::new(x + w - 9.0, row_y),
                            text::Alignment::Right,
                            fg,
                            clip,
                        );
                    }
                }
            }
        });
    }
}
