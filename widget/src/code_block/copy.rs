//! A clipboard action that keeps its acknowledgement and feedback deadline in the widget tree.
use std::borrow::Cow;
use std::convert::Infallible;

use iced_core::clipboard;
use iced_core::shell::Bus;
use iced_core::time::{Duration, Instant};
use iced_core::widget::tree::{self, Tree};
use iced_core::widget::{Meta, Operation};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer, window,
};
use iced_widget::{Renderer, Widget as _};
use icedcn_core::Theme;

use super::Placement;

use super::glyph::Glyph;
use crate::compose::{Compose, Inherited, Slot};
use crate::marker::{self, Press};
use crate::{button, tooltip};

/// A ghost icon button that copies its value and shows a check for two
/// seconds after the clipboard confirms the write.
pub struct CopyButton<'a> {
    text: Cow<'a, str>,
    disabled: bool,
    placement: Option<Placement>,
}

/// A copy button for `value`: the complete text, including any hidden lines.
pub fn copy_button<'a>(value: impl Into<Cow<'a, str>>) -> CopyButton<'a> {
    CopyButton {
        text: value.into(),
        disabled: false,
        placement: None,
    }
}

impl CopyButton<'_> {
    /// Disable copying while a transcript is incomplete.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
}

impl<'src, M: 'src> Compose<M> for CopyButton<'src> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
        self.placement = from.code_actions().or(self.placement);
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
impl<'a, M: 'a> From<CopyButton<'a>> for Slot<'a, M> {
    fn from(button: CopyButton<'a>) -> Self {
        Slot::new(button)
    }
}
impl<'a, M: 'a> From<CopyButton<'a>> for Element<'a, M, Theme, Renderer> {
    fn from(button: CopyButton<'a>) -> Self {
        Box::new(button).compose()
    }
}

#[derive(Clone)]
struct Pressed;

impl CopyButton<'_> {
    fn control<'a>(&self, copied: bool) -> Element<'a, Infallible, Theme, Renderer> {
        let glyph = if copied { Glyph::Check } else { Glyph::Copy };
        let mut button = button(glyph.view())
            .size(crate::Size::Xs)
            .square()
            .style(button::Ghost)
            .disabled(self.disabled);
        if let Some(placement) = self.placement {
            button.inherit(&Inherited::default().with_code_actions(placement));
        }
        let button: Element<'a, Infallible, Theme, Renderer> = button.into();
        let marker = marker::marker(button, Pressed, Press::Captured);
        tooltip(marker.boxed(), if copied { "Copied" } else { "Copy" }).into()
    }
}

struct Control<'a> {
    button: CopyButton<'a>,
    child: Element<'a, Infallible, Theme, Renderer>,
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
impl Meta for Control<'_> {}
impl<'a, M: 'a> Widget<M, Theme, Renderer> for Control<'a> {
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
        let mut bus = Bus::new();
        let mut local = shell.local(&mut bus);
        if shell.is_event_captured() {
            local.capture_event();
        }
        self.child
            .update(child, event, layout, cursor, renderer, &mut local, viewport);
        shell.merge(local, |never| match never {});
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
            .into_iter()
            .map(|overlay| overlay.map(&|never| match never {}))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::clipboard::{Content, Error};
    use iced_core::keyboard::key::Named;
    use iced_core::renderer::Headless;
    use iced_core::shell::Waker;
    use iced_test::futures::futures::executor::block_on;
    use iced_test::simulator::{click, press_key};

    #[test]
    fn header_copy_writes_the_complete_value_of_a_collapsed_block() {
        use crate::code_block::{self, code_block, copy_button};

        let renderer =
            block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
        let source = "first\n\nlast";
        let mut element: Element<'_, (), Theme, Renderer> =
            code_block(code_block::content(source).max_height(16).collapsed(true))
                .header(code_block::header("Output").actions(copy_button(source)))
                .into();
        let mut tree = Tree::new(&element);
        tree.diff(&mut element);
        let size = Size::new(200., 200.);
        element.layout(&mut tree, &renderer, &layout::Limits::new(Size::ZERO, size));
        let mut bus = Bus::new();
        let mut write = None;
        for event in click() {
            let mut shell = Shell::new(&window::Headless, Waker::noop(), &mut bus);
            element.update(
                &mut tree,
                &event,
                Layout::new(size),
                mouse::Cursor::Available(iced_core::Point::new(182., 18.)),
                &renderer,
                &mut shell,
                &Rectangle::with_size(size),
            );
            write = shell.clipboard_mut().write.take().or(write);
        }
        assert_eq!(write, Some(Content::Text(source.into())));
    }

    #[test]
    fn copy_writes_on_release_and_shows_feedback_only_after_success() {
        fn action<'a>(source: &'a str) -> Element<'a, &'a mut str, Theme, Renderer> {
            Slot::from(copy_button(source)).compose()
        }
        let source = String::from("first\n\nlast");
        let mut control = action(&source);
        let mut tree = Tree::new(&control);
        tree.diff(&mut control);
        let renderer =
            block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
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
        assert!(send(&mut tree, press_key(Named::Enter, None)).is_none());
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
        for key in [Named::Enter, Named::Space] {
            assert_eq!(
                send(&mut tree, press_key(key, None)),
                Some(Content::Text(source.clone()))
            );
            send(
                &mut tree,
                Event::Clipboard(clipboard::Event::Written(Ok(()))),
            );
        }
        assert!(bus.is_empty());
    }
}
