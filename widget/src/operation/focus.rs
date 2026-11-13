//! Scoped focus traversal for controls that own their keyboard navigation.
use iced_core::Rectangle;
use iced_core::widget::operation::{self, Focusable, Outcome, focusable};
use iced_core::widget::{Id, Operation};

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
    struct Select {
        target: Option<usize>,
        current: usize,
    }
    impl Operation for Select {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn focusable(&mut self, _: Option<&Id>, _: Rectangle, control: &mut dyn Focusable) {
            if self.target == Some(self.current) {
                control.focus();
            } else {
                control.unfocus();
            }
            self.current += 1;
        }
    }
    Select { target, current: 0 }
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
