//! A clipboard action that keeps its acknowledgement and feedback deadline in the widget tree.
use std::borrow::Cow;

use iced_core::clipboard;
use iced_core::time::{Duration, Instant};
use iced_core::widget::tree::{self, Tree};
use iced_core::widget::{Meta, Operation};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer, window,
};
use iced_widget::{Renderer, Widget as _};
use icedcn_core::Theme;

use super::glyph::Glyph;
use crate::compose::{Compose, Inherited, Slot};
use crate::marker::{self, Press};
use crate::{button, tooltip};

/// A ghost icon button that copies text and acknowledges successful clipboard writes for two seconds.
pub struct CopyButton<'a> {
    text: Cow<'a, str>,
    disabled: bool,
}

/// Create a copy action for a complete source string, including any hidden lines.
pub fn copy_button<'a>(text: impl Into<Cow<'a, str>>) -> CopyButton<'a> {
    CopyButton {
        text: text.into(),
        disabled: false,
    }
}

impl CopyButton<'_> {
    /// Disable copying while a transcript is incomplete.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
}

impl<'src, M: Clone + 'src> Compose<M> for CopyButton<'src> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }
    fn compose<'a>(self: Box<Self>) -> Element<'a, M, Theme, Renderer>
    where
        Self: 'a,
    {
        let child = self.control(false);
        Control {
            button: *self,
            child,
        }
        .boxed()
    }
}
impl<'a, M: Clone + 'a> From<CopyButton<'a>> for Slot<'a, M> {
    fn from(button: CopyButton<'a>) -> Self {
        Slot::new(button)
    }
}
impl<'a, M: Clone + 'a> From<CopyButton<'a>> for Element<'a, M, Theme, Renderer> {
    fn from(button: CopyButton<'a>) -> Self {
        Box::new(button).compose()
    }
}

#[derive(Clone)]
struct Pressed;

impl CopyButton<'_> {
    fn control<'a, M: Clone + 'a>(&self, copied: bool) -> Element<'a, M, Theme, Renderer> {
        let glyph = if copied { Glyph::Check } else { Glyph::Copy };
        let button: Element<'a, M, Theme, Renderer> = button(glyph.view())
            .size(crate::Size::Xs)
            .square()
            .style(button::Ghost)
            .disabled(self.disabled)
            .into();
        let marker = marker::marker(button, Pressed, Press::Captured);
        tooltip(marker.boxed(), if copied { "Copied" } else { "Copy" }).into()
    }
}

struct Control<'a, M> {
    button: CopyButton<'a>,
    child: Element<'a, M, Theme, Renderer>,
}
#[derive(Default)]
struct State {
    pending: bool,
    until: Option<Instant>,
}

impl State {
    fn complete(&mut self, success: bool, now: Instant) {
        if std::mem::take(&mut self.pending) && success {
            self.until = Some(now + Duration::from_secs(2));
        }
    }
}
impl<M> Meta for Control<'_, M> {}
impl<'a, M: Clone + 'a> Widget<M, Theme, Renderer> for Control<'a, M> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn size(&self) -> Size<Length> {
        self.child.size()
    }
    fn diff(&mut self, tree: &mut Tree) {
        let copied = tree.state.downcast_ref::<State>().until.is_some();
        self.child = self.button.control(copied);
        tree.diff_children(std::slice::from_mut(&mut self.child));
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.child.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, M>,
        viewport: &Rectangle,
    ) {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("copy button child");
        self.child
            .update(child, event, layout, cursor, renderer, shell, viewport);
        let mut drain = marker::drain::<Pressed>();
        self.child
            .operate(child, layout, viewport, renderer, &mut drain);
        let state = tree.state.downcast_mut::<State>();
        let was_copied = state.until.is_some();
        if !drain.into_signals().is_empty() && !state.pending {
            state.pending = true;
            shell.write_clipboard(self.button.text.to_string());
        }
        match event {
            Event::Clipboard(clipboard::Event::Written(result)) => {
                state.complete(result.is_ok(), Instant::now())
            }
            Event::Window(window::Event::RedrawRequested(now))
                if state.until.is_some_and(|end| *now >= end) =>
            {
                state.until = None
            }
            _ => {}
        }
        if let Some(until) = state.until {
            shell.request_redraw_at(until);
        }
        if was_copied != state.until.is_some() {
            self.child = self.button.control(state.until.is_some());
            child.diff(&mut self.child);
            shell.invalidate_layout();
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
        let (layout, child) = layout
            .iter(&tree.children)
            .next()
            .expect("copy button child");
        self.child
            .draw(child, renderer, theme, style, layout, cursor, viewport)
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        op: &mut dyn Operation,
    ) {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("copy button child");
        self.child.operate(child, layout, viewport, renderer, op)
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (layout, child) = layout
            .iter(&tree.children)
            .next()
            .expect("copy button child");
        self.child
            .mouse_interaction(child, layout, cursor, viewport, renderer)
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, M, Theme, Renderer>> {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("copy button child");
        self.child
            .overlay(child, layout, renderer, viewport, translation, window)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::clipboard::{Content, Error};
    use iced_core::renderer::Headless;
    use iced_core::shell::{Bus, Waker};
    use iced_test::futures::futures::executor::block_on;

    #[test]
    fn copy_writes_on_release_and_shows_feedback_only_after_success() {
        let button = copy_button("first\n\nlast");
        let mut control = Control::<()> {
            child: button.control(false),
            button,
        };
        let mut tree = Tree::new(&control);
        tree.diff(&mut control);
        let renderer = block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
        let size = Size::new(100., 100.);
        control.layout(&mut tree, &renderer, &layout::Limits::new(Size::ZERO, size));
        let viewport = Rectangle::with_size(size);
        let mut bus = Bus::new();
        let mut send = |tree: &mut Tree, event: Event| {
            let mut shell = Shell::new(&window::Headless, Waker::noop(), &mut bus);
            control.update(
                tree,
                &event,
                Layout::new(tree.size),
                mouse::Cursor::Available(iced_core::Point::new(5., 5.)),
                &renderer,
                &mut shell,
                &viewport,
            );
            shell.clipboard_mut().write.take()
        };
        let press = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let release = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        let ok = Event::Clipboard(clipboard::Event::Written(Ok(())));
        let fail = Event::Clipboard(clipboard::Event::Written(Err(Error::ClipboardUnavailable)));
        send(&mut tree, ok.clone());
        assert!(tree.state.downcast_ref::<State>().until.is_none());
        for completion in [fail, ok.clone(), ok] {
            assert!(send(&mut tree, press.clone()).is_none());
            assert_eq!(
                send(&mut tree, release.clone()),
                Some(Content::Text("first\n\nlast".into()))
            );
            let old = tree.state.downcast_ref::<State>().until;
            send(&mut tree, completion.clone());
            let new = tree.state.downcast_ref::<State>().until;
            if let Event::Clipboard(clipboard::Event::Written(Ok(()))) = completion {
                let end = new.expect("success shows the check");
                if let Some(previous) = old {
                    assert!(end >= previous);
                    send(
                        &mut tree,
                        Event::Window(window::Event::RedrawRequested(
                            previous - Duration::from_millis(1),
                        )),
                    );
                    assert_eq!(tree.state.downcast_ref::<State>().until, Some(end));
                }
            } else {
                assert_eq!(new, old);
            }
        }
        let until = tree.state.downcast_ref::<State>().until.unwrap();
        send(
            &mut tree,
            Event::Window(window::Event::RedrawRequested(until)),
        );
        assert!(tree.state.downcast_ref::<State>().until.is_none());
        assert!(bus.is_empty());
    }
}
