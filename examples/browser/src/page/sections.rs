//! A vertical column keeping the page's visible section indices in sync with its scroll viewport.
use crate::{Message, page};
use iced::{Element, Event, Fill, Length, Rectangle, Renderer, Size, Vector, mouse, touch, window};
use iced_widget::core::layout::flex;
use iced_widget::core::widget::{Meta, Operation, Tree, tree};
use iced_widget::core::{Layout, Shell, Widget, layout, overlay, renderer};
use icedcn::Theme;
use std::collections::{BTreeSet, HashSet};
use std::ops::Range;

pub(super) struct Sections<'a> {
    items: Vec<Element<'a, Message, Theme>>,
    page_index: usize,
    notify: bool,
    height: Length,
}

impl<'a> Sections<'a> {
    pub fn new(items: Vec<Element<'a, Message, Theme>>, page_index: usize, notify: bool) -> Self {
        Self {
            items,
            page_index,
            notify,
            height: Length::Fit,
        }
    }

    fn track(
        &self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        shell: &mut Shell<'_, Message>,
    ) {
        let last = tree.state.downcast_mut::<Visibility>();
        if !self.notify {
            last.report = None;
            return;
        }
        // The vertical column's child bounds are ordered by y. The corner
        // checks preserve the original Sensor's 50px anticipation distance.
        let children = &tree.children;
        let top = viewport.y - layout.position().y;
        let end = top + viewport.height;
        let visible = rows(children, top - 50., end + 50.).filter(|&index| {
            let child = &children[index];
            let position = layout.position() + child.translation;
            viewport
                .distance(position)
                .min(viewport.distance(position + Vector::from(child.size)))
                <= 50.
        });
        if last.report.as_ref().is_some_and(|(page, indices)| {
            *page == self.page_index && indices.iter().copied().eq(visible.clone())
        }) {
            return;
        }
        let indices: BTreeSet<_> = visible.collect();
        last.report = Some((self.page_index, indices.clone()));
        shell.publish(Message::Page(
            self.page_index,
            page::Message::VisibleSections(indices),
        ));
    }
}

fn rows(children: &[Tree], top: f32, bottom: f32) -> Range<usize> {
    let first = children.partition_point(|row| row.translation.y + row.size.height < top);
    let last = children.partition_point(|row| row.translation.y <= bottom);
    first..last
}

fn pointer_rows(
    tree: &Tree,
    layout: Layout,
    cursor: mouse::Cursor,
    viewport: &Rectangle,
) -> Range<usize> {
    let state = tree.state.downcast_ref::<Visibility>();
    // A drag can retain its cursor far outside the section that captured it.
    // Preserve the full query during capture and underneath popup overlays.
    match cursor {
        mouse::Cursor::Available(point) if state.buttons.is_empty() && state.fingers.is_empty() => {
            if !viewport.contains(point) {
                return 0..0;
            }
            let y = point.y - layout.position().y;
            rows(&tree.children, y, y)
        }
        _ => 0..tree.children.len(),
    }
}

#[derive(Default)]
struct Visibility {
    layout: flex::Cache,
    buttons: HashSet<mouse::Button>,
    fingers: HashSet<touch::Finger>,
    report: Option<(usize, BTreeSet<usize>)>,
}

impl Meta for Sections<'_> {}
impl Widget<Message, Theme, Renderer> for Sections<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Visibility>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(Visibility::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.items);
        self.height = self
            .items
            .iter()
            .fold(Length::Fit, |height, item| height.stack(item.size().height));
        if !self.notify {
            tree.state.downcast_mut::<Visibility>().report = None;
        }
    }
    fn size(&self) -> Size<Length> {
        Size::new(Fill, self.height)
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let state = tree.state.downcast_mut::<Visibility>();
        tree.size = flex::resolve(
            flex::Axis::Vertical,
            renderer,
            limits,
            Fill,
            self.height,
            iced::Padding::ZERO,
            48.,
            iced::Alignment::Start,
            &mut tree.children,
            &mut self.items,
            &mut state.layout,
        );
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
        let state = tree.state.downcast_mut::<Visibility>();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(button)) => {
                state.buttons.insert(*button);
            }
            Event::Mouse(mouse::Event::ButtonReleased(button)) => {
                state.buttons.remove(button);
            }
            Event::Touch(touch::Event::FingerPressed { id, .. }) => {
                state.fingers.insert(*id);
            }
            Event::Touch(
                touch::Event::FingerLifted { id, .. } | touch::Event::FingerLost { id, .. },
            ) => {
                state.fingers.remove(id);
            }
            Event::Window(window::Event::Unfocused) => {
                state.buttons.clear();
                state.fingers.clear();
            }
            Event::Window(window::Event::RedrawRequested(_)) => {
                self.track(tree, layout, viewport, shell);
            }
            _ => {}
        }
        let range = if matches!(event, Event::Mouse(mouse::Event::WheelScrolled { .. }))
            && !shell.is_event_captured()
        {
            pointer_rows(tree, layout, cursor, viewport)
        } else {
            0..self.items.len()
        };
        for (item, (layout, child)) in self.items[range.clone()]
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children[range]))
        {
            item.update(child, event, layout, cursor, renderer, shell, viewport);
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
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            for (item, (layout, child)) in self
                .items
                .iter_mut()
                .zip(layout.iter_mut(&mut tree.children))
            {
                item.operate(child, layout, viewport, renderer, operation);
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
        let range = pointer_rows(tree, layout, cursor, viewport);
        self.items[range.clone()]
            .iter()
            .zip(layout.iter(&tree.children[range]))
            .map(|(item, (layout, child))| {
                item.mouse_interaction(child, layout, cursor, viewport, renderer)
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
        if let Some(clip) = layout.bounds().intersection(viewport) {
            let top = clip.y - layout.position().y;
            let range = rows(&tree.children, top, top + clip.height);
            for (item, (layout, child)) in self.items[range.clone()]
                .iter()
                .zip(layout.iter(&tree.children[range]))
            {
                if layout.bounds().intersects(&clip) {
                    item.draw(child, renderer, theme, style, layout, cursor, viewport);
                }
            }
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
            &mut self.items,
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
mod tests;
