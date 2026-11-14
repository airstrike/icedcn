//! Shared tracks for invoice and data tables. The first layout pass measures
//! intrinsic columns across all row groups. The second lays out each cell
//! against its final (possibly spanning) column width.
use super::{Body, Footer, Header, Row};
use crate::animation::cubic_bezier;
use crate::compose::{Compose, Inherited, Slot};
use iced_core::animation::Easing;
use iced_core::time::{Duration, Instant};
use iced_core::widget::tree;
use iced_core::widget::{Meta, Operation, Tree};
use iced_core::{Animation, window};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Renderer as _, Shell, Size, Vector, Widget, layout,
    mouse, overlay, renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Track {
    Auto,
    Fixed(f32),
    Flex(u16),
}

pub struct Grid<'a, Message> {
    tracks: Vec<Track>,
    groups: Vec<(Kind, Vec<Row<'a, Message>>)>,
}
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Header,
    Body,
    Footer,
}
pub fn grid<'a, Message>(tracks: impl IntoIterator<Item = Track>) -> Grid<'a, Message> {
    Grid {
        tracks: tracks.into_iter().collect(),
        groups: Vec::new(),
    }
}
impl<'a, Message> Grid<'a, Message> {
    pub fn header(mut self, group: Header<'a, Message>) -> Self {
        self.groups.push((Kind::Header, group.rows));
        self
    }
    pub fn body(mut self, group: Body<'a, Message>) -> Self {
        self.groups.push((Kind::Body, group.rows));
        self
    }
    pub fn footer(mut self, group: Footer<'a, Message>) -> Self {
        self.groups.push((Kind::Footer, group.rows));
        self
    }
}
impl<'a, Message: 'a> Compose<Message> for Grid<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        for (_, rows) in &mut self.groups {
            for row in rows {
                for cell in &mut row.cells {
                    cell.content.inherit(from);
                }
            }
        }
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let mut cells = Vec::new();
        let mut rows = Vec::new();
        let mut spans = Vec::new();
        let mut tracks = self.tracks;
        for (kind, group) in self.groups {
            let count = group.len();
            for (index, row) in group.into_iter().enumerate() {
                let start = cells.len();
                let mut col = 0;
                for mut cell in row.cells {
                    let span = cell.span;
                    spans.push((col, span));
                    col += span;
                    cell.width = Length::Fill;
                    cells.push(Element::from(cell));
                }
                tracks.resize(tracks.len().max(col), Track::Auto);
                rows.push(Line {
                    first: start,
                    end: cells.len(),
                    selected: row.selected,
                    kind,
                    border: row.border && (kind == Kind::Header || index + 1 < count),
                    top: kind == Kind::Footer && index == 0,
                });
            }
        }
        LayoutGrid {
            cells,
            spans,
            rows,
            tracks,
        }
        ._boxed()
    }
}
impl<'a, Message: 'a> From<Grid<'a, Message>> for Slot<'a, Message> {
    fn from(grid: Grid<'a, Message>) -> Self {
        Slot::new(grid)
    }
}
impl<'a, Message: 'a> From<Grid<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(grid: Grid<'a, Message>) -> Self {
        Box::new(grid).compose()
    }
}

struct Line {
    first: usize,
    end: usize,
    kind: Kind,
    selected: bool,
    border: bool,
    top: bool,
}
struct LayoutGrid<'a, Message> {
    cells: Vec<Element<'a, Message, Theme, Renderer>>,
    spans: Vec<(usize, usize)>,
    rows: Vec<Line>,
    tracks: Vec<Track>,
}
struct Surface {
    tint: Animation<f32>,
    now: Instant,
    hovered: bool,
}

impl Surface {
    fn new(alpha: f32) -> Self {
        Self {
            tint: Animation::new(alpha)
                .duration(Duration::from_millis(150))
                .easing(Easing::Custom(|t| cubic_bezier(0.4, 0.0, 0.2, 1.0, t))),
            now: Instant::now(),
            hovered: false,
        }
    }

    fn update(&mut self, alpha: f32, now: Instant) {
        self.now = now;
        if self.tint.value() != alpha {
            self.tint.go_mut(alpha, now);
        }
    }
}

impl Line {
    fn alpha(&self, hovered: bool) -> f32 {
        if self.selected {
            1.0
        } else if self.kind == Kind::Footer || hovered {
            0.5
        } else {
            0.0
        }
    }
}

impl<Message> Meta for LayoutGrid<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for LayoutGrid<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Vec<Surface>>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(
            self.rows
                .iter()
                .map(|row| Surface::new(row.alpha(false)))
                .collect::<Vec<_>>(),
        )
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.cells);
        let surfaces = tree.state.downcast_mut::<Vec<Surface>>();
        surfaces.truncate(self.rows.len());
        for row in self.rows.iter().skip(surfaces.len()) {
            surfaces.push(Surface::new(row.alpha(false)));
        }
        let now = Instant::now();
        for (row, surface) in self.rows.iter().zip(surfaces) {
            surface.update(row.alpha(surface.hovered), now);
        }
    }
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let width = limits.bounds().width;
        let mut widths: Vec<f32> = self
            .tracks
            .iter()
            .map(|t| match t {
                Track::Fixed(w) => w.max(0.0),
                _ => 0.0,
            })
            .collect();
        let natural =
            layout::Limits::new(Size::ZERO, Size::new(width, f32::INFINITY)).width(Length::Shrink);
        for ((cell, state), &(col, span)) in self
            .cells
            .iter_mut()
            .zip(&mut tree.children)
            .zip(&self.spans)
        {
            if span == 1 && self.tracks[col] == Track::Auto {
                cell.layout(state, renderer, &natural);
                widths[col] = widths[col].max(state.size.width);
            }
        }
        // A horizontal scroller preserves its viewport in max.width while
        // making bounds() infinite. Fill that hint, and overflow only when
        // the intrinsic columns need more room.
        let width = if width.is_finite() {
            width
        } else {
            let intrinsic = widths.iter().sum::<f32>();
            if limits.max.width.is_finite() {
                intrinsic.max(limits.max.width)
            } else {
                intrinsic
            }
        };
        let rigid: f32 = widths.iter().sum();
        let remaining = (width - rigid).max(0.0);
        let units: u32 = self
            .tracks
            .iter()
            .filter_map(|t| {
                if let Track::Flex(n) = t {
                    Some(u32::from(*n))
                } else {
                    None
                }
            })
            .sum();
        let automatic: f32 = widths
            .iter()
            .zip(&self.tracks)
            .filter_map(|(w, t)| (*t == Track::Auto).then_some(*w))
            .sum();
        for (w, t) in widths.iter_mut().zip(&self.tracks) {
            if let Track::Flex(n) = t {
                *w = remaining * f32::from(*n) / units.max(1) as f32;
            } else if rigid > width {
                *w *= width / rigid;
            } else if units == 0 && *t == Track::Auto && automatic > 0.0 {
                *w += remaining * *w / automatic;
            }
        }
        let mut x = vec![0.0];
        for w in widths {
            x.push(x.last().copied().unwrap_or_default() + w);
        }
        let mut y = 0.0;
        for row in &self.rows {
            if row.top {
                y += 1.0;
            }
            let mut height = 0.0_f32;
            for i in row.first..row.end {
                let (col, span) = self.spans[i];
                let w = x[col + span] - x[col];
                let child = &mut tree.children[i];
                self.cells[i].layout(
                    child,
                    renderer,
                    &layout::Limits::new(Size::new(w, 0.0), Size::new(w, f32::INFINITY)),
                );
                height = height.max(child.size.height);
            }
            for i in row.first..row.end {
                let child = &mut tree.children[i];
                child.translation =
                    Vector::new(x[self.spans[i].0], y + (height - child.size.height) * 0.5);
            }
            y += height + f32::from(row.border);
        }
        tree.size = Size::new(width, y);
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
        for (cell, (layout, state)) in self
            .cells
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            cell.update(state, event, layout, cursor, renderer, shell, viewport);
        }
        let now = match event {
            Event::Window(window::Event::RedrawRequested(now)) => *now,
            _ => Instant::now(),
        };
        let mut y = layout.bounds().y;
        let surfaces = tree.state.downcast_mut::<Vec<Surface>>();
        for (row, surface) in self.rows.iter().zip(surfaces) {
            y += f32::from(row.top);
            let height = tree.children[row.first..row.end]
                .iter()
                .map(|node| node.size.height)
                .fold(0.0, f32::max);
            let bounds = Rectangle {
                x: layout.bounds().x,
                y,
                width: layout.bounds().width,
                height,
            };
            surface.hovered = cursor.is_over(bounds) && cursor.is_over(*viewport);
            surface.update(row.alpha(surface.hovered), now);
            if surface.tint.is_animating(now) {
                shell.request_redraw();
            }
            y += height + f32::from(row.border);
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
        for (cell, (layout, state)) in self
            .cells
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            cell.operate(state, layout, viewport, renderer, operation);
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
        self.cells
            .iter()
            .zip(layout.iter(&tree.children))
            .map(|(cell, (layout, state))| {
                cell.mouse_interaction(state, layout, cursor, viewport, renderer)
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
        let mut y = layout.bounds().y;
        let surfaces = tree.state.downcast_ref::<Vec<Surface>>();
        for (row, surface) in self.rows.iter().zip(surfaces) {
            let height = tree.children[row.first..row.end]
                .iter()
                .map(|node| node.size.height)
                .fold(0.0, f32::max);
            let band = |y, height| renderer::Quad {
                bounds: Rectangle {
                    x: layout.bounds().x,
                    y,
                    width: layout.bounds().width,
                    height,
                },
                ..Default::default()
            };
            if row.top {
                renderer.fill_quad(band(y, 1.0), theme.neutral.border);
                y += 1.0;
            }
            let background = band(y, height);
            let alpha = surface.tint.interpolate(surface.now);
            if alpha > 0.0 {
                renderer.fill_quad(background, theme.muted.base.alpha(alpha));
            }
            y += height;
            if row.border {
                renderer.fill_quad(band(y, 1.0), theme.neutral.border);
                y += 1.0;
            }
        }
        // Focus rings may extend into a cell's padding or neighboring cell.
        // The scroll viewport, not the individual cell, owns their clipping.
        if let Some(visible) = viewport.intersection(&layout.bounds().expand(3.0)) {
            renderer.with_layer(visible, |renderer| {
                for (cell, (layout, state)) in self.cells.iter().zip(layout.iter(&tree.children)) {
                    cell.draw(state, renderer, theme, style, layout, cursor, &visible);
                }
            });
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
            &mut self.cells,
            tree,
            layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Surface;
    use iced_core::time::{Duration, Instant};

    #[test]
    fn row_hover_and_selection_start_after_idle_and_reverse_continuously() {
        let mut surface = Surface::new(0.0);
        let idle = Instant::now();
        surface.update(0.0, idle);
        let start = idle + Duration::from_secs(10);
        surface.update(0.5, start);
        assert!(surface.tint.interpolate(start) < 0.001);
        let halfway = start + Duration::from_millis(75);
        let before = surface.tint.interpolate(halfway);
        assert!((before - 0.3878).abs() < 0.01);
        surface.update(1.0, halfway);
        assert!((surface.tint.interpolate(halfway) - before).abs() < 0.001);
        let selected = halfway + Duration::from_millis(151);
        assert!((surface.tint.interpolate(selected) - 1.0).abs() < 0.001);
        surface.update(0.0, selected);
        assert!((surface.tint.interpolate(selected) - 1.0).abs() < 0.001);
        let settled = selected + Duration::from_millis(151);
        assert!(surface.tint.interpolate(settled) < 0.001);
        assert!(!surface.tint.is_animating(settled));
    }
}
