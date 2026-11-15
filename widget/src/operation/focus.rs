//! Scoped focus traversal for controls that own their keyboard navigation.
use super::scroll::Reveal;
use iced_core::widget::operation::{
    self, Focusable, Outcome, Scrollable, Text, TextInput, focusable,
};
use iced_core::widget::{Id, Operation};
use iced_core::{Rectangle, Size, Vector};
use std::any::Any;

/// Whether the focused control should show a keyboard focus indicator.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Visible(pub bool);

/// A control's focus decoration can be painted by a surrounding Field card.
#[derive(Debug, Default)]
pub(crate) struct Ring {
    pub delegated: bool,
    pub visible: bool,
    pub disabled: bool,
}

/// Read focusable descendants in the order reported by the widget tree.
pub(crate) fn count(mut visit: impl FnMut(&mut dyn Operation)) -> focusable::Count {
    let mut count = focusable::count();
    visit(&mut operation::black_box(&mut count));
    match count.finish() {
        Outcome::Some(count) => count,
        _ => focusable::Count::default(),
    }
}

/// Focus a single descendant by traversal index. `None` clears the scope.
pub(crate) fn select(target: Option<usize>) -> impl Operation {
    restore(target, true)
}

/// Return focus with the modality that opened or operated a modal.
pub(crate) fn restore(target: Option<usize>, visible: bool) -> impl Operation {
    Select {
        target,
        current: 0,
        visible,
        bounds: None,
    }
}

/// Focus an entry in a scrolled list and bring its bounds into view.
pub(crate) fn reveal(index: usize, scroll: &Id, mut visit: impl FnMut(&mut dyn Operation)) {
    let mut target = Select {
        target: Some(index),
        current: 0,
        visible: true,
        bounds: None,
    };
    visit(&mut target);
    if let Some(bounds) = target.bounds {
        visit(&mut Reveal {
            scroll: scroll.clone(),
            target: bounds,
        });
    }
}

/// Focus a descendant and select its text for replacement or copying.
pub(crate) fn enter(index: usize, visible: bool, mut visit: impl FnMut(&mut dyn Operation)) {
    let mut focus = Select {
        target: Some(index),
        current: 0,
        visible,
        bounds: None,
    };
    visit(&mut focus);
    if let Some(bounds) = focus.bounds {
        visit(&mut SelectText(bounds));
    }
}

struct SelectText(Rectangle);

impl Operation for SelectText {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn text_input(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        input: &mut dyn operation::TextInput,
    ) {
        if self.0 == bounds {
            input.select_all();
        }
    }
}

struct Select {
    target: Option<usize>,
    current: usize,
    visible: bool,
    bounds: Option<Rectangle>,
}
impl Operation for Select {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, control: &mut dyn Focusable) {
        if self.target == Some(self.current) {
            control.focus();
            self.bounds = Some(bounds);
        } else {
            control.unfocus();
        }
        self.current += 1;
    }
    fn custom(&mut self, _: Option<&Id>, _: Rectangle, state: &mut dyn Any) {
        if self.current.checked_sub(1) == self.target
            && let Some(indicator) = state.downcast_mut::<Visible>()
        {
            indicator.0 = self.visible;
        }
    }
}

/// Next index in a focus scope, wrapping at either boundary.
pub(crate) fn adjacent(count: focusable::Count, reverse: bool) -> Option<usize> {
    if count.total == 0 {
        return None;
    }
    Some(match (count.focused, reverse) {
        (None, false) => 0,
        (None, true) | (Some(0), true) => count.total - 1,
        (Some(i), true) => i - 1,
        (Some(i), false) => (i + 1) % count.total,
    })
}

/// Expose the group's tab stop to global focus traversal while forwarding other operations.
pub(crate) fn roving(
    operation: &mut dyn Operation,
    expose: impl Fn(usize) -> bool + Sync,
    mut visit: impl FnMut(&mut dyn Operation),
) -> Option<Rectangle> {
    let mut entered = None;
    visit(&mut Roving {
        operation,
        expose: &expose,
        entered: &mut entered,
        index: 0,
        hidden: false,
    });
    entered
}
struct Roving<'a> {
    operation: &'a mut dyn Operation,
    expose: &'a (dyn Fn(usize) -> bool + Sync),
    entered: &'a mut Option<Rectangle>,
    index: usize,
    hidden: bool,
}
impl Operation for Roving<'_> {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        let mut index = self.index;
        let mut hidden = self.hidden;
        self.operation.traverse(&mut |operation| {
            let mut nested = Roving {
                operation,
                expose: self.expose,
                entered: self.entered,
                index,
                hidden,
            };
            visit(&mut nested);
            index = nested.index;
            hidden = nested.hidden;
        });
        self.index = index;
        self.hidden = hidden;
    }
    fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
        self.hidden = !(self.expose)(self.index);
        if !self.hidden {
            let focused = state.is_focused();
            self.operation.focusable(id, bounds, state);
            if !focused && state.is_focused() {
                *self.entered = Some(bounds);
            }
        }
        self.index += 1;
    }
    fn container(&mut self, id: Option<&Id>, bounds: Rectangle, viewport: &Rectangle) {
        self.operation.container(id, bounds, viewport);
    }
    fn text(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Text) {
        self.operation.text(id, bounds, state);
    }
    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        if !self.hidden || !state.is::<Visible>() {
            self.operation.custom(id, bounds, state);
        }
    }
    fn scrollable(
        &mut self,
        id: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        translation: Vector,
        state: &mut dyn Scrollable,
    ) {
        self.operation
            .scrollable(id, bounds, content, translation, state);
    }
    fn text_input(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn TextInput) {
        self.operation.text_input(id, bounds, state);
    }
}
