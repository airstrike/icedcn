//! Scoped focus traversal for controls that own their keyboard navigation.
use super::scroll::Reveal;
use iced_core::Rectangle;
use iced_core::widget::operation::{self, Focusable, Outcome, focusable};
use iced_core::widget::{Id, Operation};
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

/// Enter a modal and select its initially focused text input for replacement or copying.
pub(crate) fn enter(visible: bool, mut visit: impl FnMut(&mut dyn Operation)) {
    let mut focus = Select {
        target: Some(0),
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
