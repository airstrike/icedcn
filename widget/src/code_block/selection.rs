//! Keep the empty code rows between selected paragraphs in native text copy.
use std::any::Any;
use std::collections::HashMap;

use iced_core::text::{Fragment, Target};
use iced_core::widget::operation::{Focusable, Scrollable, Text, TextInput};
use iced_core::widget::{Id, Operation};
use iced_core::{Point, Rectangle, Size, Vector};

pub(super) struct Selected {
    rows: Vec<Rectangle>,
    empty: Vec<bool>,
    last: Vec<Option<usize>>,
    index: usize,
}
impl Selected {
    pub(super) fn new(rows: Vec<Rectangle>) -> Self {
        Self {
            empty: vec![false; rows.len()],
            last: vec![None; rows.len()],
            rows,
            index: 0,
        }
    }
    pub(super) fn separators(&self) -> HashMap<usize, usize> {
        let selected = self
            .last
            .iter()
            .enumerate()
            .filter_map(|(row, index)| index.map(|index| (row, index)))
            .collect::<Vec<_>>();
        selected
            .windows(2)
            .filter_map(|pair| {
                let [(first, index), (last, _)] = pair else {
                    return None;
                };
                let empty = self.empty[first + 1..*last]
                    .iter()
                    .filter(|empty| **empty)
                    .count();
                (empty > 0).then_some((*index, empty))
            })
            .collect()
    }
}
impl Operation for Selected {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn text(&mut self, _: Option<&Id>, bounds: Rectangle, text: &mut dyn Text) {
        if let Some(row) = self
            .rows
            .iter()
            .position(|row| bounds.y >= row.y && bounds.y < row.y + row.height)
        {
            self.empty[row] |= text.text().is_empty();
            if text.copy().is_some_and(|text| !text.is_empty()) {
                self.last[row] = Some(self.index);
            }
        }
        self.index += 1;
    }
}

pub(super) struct Lines<'a> {
    pub(super) inner: &'a mut dyn Operation,
    pub(super) separators: &'a HashMap<usize, usize>,
    pub(super) index: &'a mut usize,
}
impl Operation for Lines<'_> {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        let Self {
            inner,
            separators,
            index,
        } = self;
        inner.traverse(&mut |inner| {
            visit(&mut Lines {
                inner,
                separators,
                index,
            })
        });
    }
    fn container(&mut self, id: Option<&Id>, bounds: Rectangle, viewport: &Rectangle) {
        self.inner.container(id, bounds, viewport);
    }
    fn scrollable(
        &mut self,
        id: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        offset: Vector,
        state: &mut dyn Scrollable,
    ) {
        self.inner.scrollable(id, bounds, content, offset, state);
    }
    fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
        self.inner.focusable(id, bounds, state);
    }
    fn text_input(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn TextInput) {
        self.inner.text_input(id, bounds, state);
    }
    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        self.inner.custom(id, bounds, state);
    }
    fn text(&mut self, id: Option<&Id>, bounds: Rectangle, text: &mut dyn Text) {
        let newlines = self.separators.get(self.index).copied().unwrap_or(0);
        *self.index += 1;
        self.inner.text(
            id,
            bounds,
            &mut Row {
                inner: text,
                newlines,
            },
        );
    }
}
struct Row<'a> {
    inner: &'a mut dyn Text,
    newlines: usize,
}
impl Text for Row<'_> {
    fn text(&self) -> Fragment<'_> {
        self.inner.text()
    }
    fn select(&mut self, start: Point, end: Point, target: Target) {
        self.inner.select(start, end, target);
    }
    fn select_all(&mut self) {
        self.inner.select_all();
    }
    fn deselect(&mut self) {
        self.inner.deselect();
    }
    fn copy(&mut self) -> Option<String> {
        self.inner.copy().map(|mut text| {
            if !text.is_empty() {
                text.extend(std::iter::repeat_n('\n', self.newlines));
            }
            text
        })
    }
}
