use super::{State, Toaster, VISIBLE};
use crate::core::Widget;
use crate::core::keyboard::key::{Code, Named, Physical};
use crate::core::keyboard::{Event::KeyPressed, Key, Modifiers};
use crate::core::widget::{Operation, Tree};
use crate::core::{Event, Layout, Rectangle, Shell, mouse, touch};
use crate::operation::focus;
use crate::toast::Position;
use iced_widget::Renderer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Stop {
    List,
    Card(u64),
    Action(u64),
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Region {
    pub position: Position,
    pub stop: Stop,
    previous: Option<usize>,
}

impl<Message: Clone + Send + 'static> Toaster<'_, Message> {
    /// Enter the notification list on Alt+T. Ordinary Tab remains in the application
    /// until the user chooses the list. Its tab stops are each card and its action.
    pub(super) fn navigate(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) -> bool {
        let state = tree.state.downcast_mut::<State<Message>>();
        let keys = match event {
            Event::Keyboard(KeyPressed {
                key,
                physical_key,
                modifiers,
                repeat: false,
                ..
            }) => Some((key.as_ref(), physical_key, *modifiers)),
            _ => None,
        };
        let shortcut = keys.as_ref().is_some_and(|(key, physical, mods)| {
            *mods == Modifiers::ALT
                && (matches!(key,Key::Character(c) if c.eq_ignore_ascii_case("t"))
                    || matches!(physical, Physical::Code(Code::KeyT)))
        });
        if !shortcut && state.region.is_none() {
            return false;
        }

        let (content, cards) = tree.children.split_at_mut(1);
        let (bounds, content) = layout
            .iter_mut(content)
            .next()
            .expect("a toaster has content");
        let mut visit = |op: &mut dyn Operation| {
            self.content
                .operate(content, bounds, viewport, renderer, op);
        };
        let Some(newest) = state.live.last() else {
            if let Some(region) = state.region.take() {
                visit(&mut focus::select(region.previous));
                shell.request_redraw();
            }
            return false;
        };
        let mut region = if shortcut {
            let region = Region {
                previous: state
                    .region
                    .map(|r| r.previous)
                    .unwrap_or_else(|| focus::count(&mut visit).focused),
                position: newest.toast.position.unwrap_or(self.position),
                stop: Stop::List,
            };
            visit(&mut focus::select(None));
            region
        } else if let Some(region) = state.region {
            region
        } else {
            return false;
        };

        let mut active = true;
        let mut handled = true;
        if !shortcut {
            let missing = match region.stop {
                Stop::List => !state
                    .live
                    .iter()
                    .any(|l| l.toast.position.unwrap_or(self.position) == region.position),
                Stop::Card(id) | Stop::Action(id) => !state.live.iter().any(|l| l.id == id),
            };
            if missing
                || matches!(
                    event,
                    Event::Mouse(mouse::Event::ButtonPressed(_))
                        | Event::Touch(touch::Event::FingerPressed { .. })
                )
            {
                active = false;
                handled = false;
            } else if let Some((Key::Named(Named::Tab), _, mods)) = keys {
                let mut stops = vec![Stop::List];
                for live in state
                    .live
                    .iter()
                    .rev()
                    .filter(|l| l.toast.position.unwrap_or(self.position) == region.position)
                    .take(VISIBLE)
                {
                    stops.push(Stop::Card(live.id));
                    if live.toast.action.is_some() {
                        stops.push(Stop::Action(live.id));
                    }
                }
                let current = stops.iter().position(|s| *s == region.stop).unwrap_or(0);
                let next = if mods.shift() {
                    current.checked_sub(1)
                } else {
                    Some(current + 1)
                };
                if let Some(stop) = next.and_then(|n| stops.get(n)) {
                    region.stop = *stop;
                } else {
                    active = false;
                }
            } else {
                return false;
            }
        }

        for (card, tree) in self.cards.iter_mut().zip(cards) {
            let selected = (active && region.stop == Stop::Action(card.id)).then_some(0);
            card.element.operate(
                tree,
                Layout::new(tree.size),
                viewport,
                renderer,
                &mut focus::select(selected),
            );
        }
        if !active {
            visit(&mut focus::select(region.previous));
        }
        state.region = active.then_some(region);
        shell.invalidate_overlay();
        shell.request_redraw();
        if handled {
            shell.capture_event();
        }
        handled
    }
}
