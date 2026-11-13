//! Combine iced's pointer hover intent with keyboard focus in one anchored overlay.
//! Placement, clipping, escape, and pointer travel between layers stay in iced.
use crate::operation::focus;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::shell::Bus;
use iced_core::widget::{
    Meta, Operation,
    tree::{self, Tree},
};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Widget as _;
use iced_widget::{Anchored, Renderer, anchored};
use icedxdy_core::Theme;
use std::cell::Cell;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Mode {
    Hint,
    Submenu,
}
enum Input<Message> {
    App(Message),
    Toggle(bool),
}

// iced's Widget::map requires a static output message; tooltips also accept
// borrowed application messages. Map only the two child elements here.
struct Content<'a, M>(Element<'a, M, Theme, Renderer>);
impl<M> Meta for Content<'_, M> {}
impl<'a, M: 'a> Widget<Input<M>, Theme, Renderer> for Content<'a, M> {
    fn tag(&self) -> tree::Tag {
        self.0.tag()
    }
    fn state(&self) -> tree::State {
        self.0.state()
    }
    fn diff(&mut self, t: &mut Tree) {
        self.0.diff(t)
    }
    fn size(&self) -> Size<Length> {
        self.0.size()
    }
    fn layout(&mut self, t: &mut Tree, r: &Renderer, l: &layout::Limits) {
        self.0.layout(t, r, l)
    }
    fn operate(
        &mut self,
        t: &mut Tree,
        l: Layout,
        v: &Rectangle,
        r: &Renderer,
        o: &mut dyn Operation,
    ) {
        self.0.operate(t, l, v, r, o)
    }
    fn draw(
        &self,
        t: &Tree,
        r: &mut Renderer,
        theme: &Theme,
        s: &renderer::Style,
        l: Layout,
        c: mouse::Cursor,
        v: &Rectangle,
    ) {
        self.0.draw(t, r, theme, s, l, c, v)
    }
    fn update(
        &mut self,
        t: &mut Tree,
        e: &Event,
        l: Layout,
        c: mouse::Cursor,
        r: &Renderer,
        s: &mut Shell<'_, Input<M>>,
        v: &Rectangle,
    ) {
        let mut bus = Bus::new();
        let mut local = s.local(&mut bus);
        self.0.update(t, e, l, c, r, &mut local, v);
        s.merge(local, Input::App);
    }
    fn mouse_interaction(
        &self,
        t: &Tree,
        l: Layout,
        c: mouse::Cursor,
        v: &Rectangle,
        r: &Renderer,
    ) -> mouse::Interaction {
        self.0.mouse_interaction(t, l, c, v, r)
    }
    fn overlay<'b>(
        &'b mut self,
        t: &'b mut Tree,
        l: Layout,
        r: &Renderer,
        v: &Rectangle,
        d: Vector,
        w: Size,
    ) -> Vec<overlay::Element<'b, Input<M>, Theme, Renderer>> {
        self.0
            .overlay(t, l, r, v, d, w)
            .into_iter()
            .map(|layer| layer.map(&Input::App))
            .collect()
    }
}
type Child<'a, M> = Element<'a, Input<M>, Theme, Renderer>;
type Anchor<'a, M> = Anchored<'a, Child<'a, M>, Child<'a, M>, Input<M>, Theme>;

pub(crate) struct Interaction<'a, Message> {
    inner: Option<Anchor<'a, Message>>,
    mode: Mode,
    controlled: Option<bool>,
    on_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}
#[derive(Default, Clone, Copy)]
struct State {
    hovered: bool,
    keyboard: bool,
    focused: bool,
    pointer: bool,
    enter: bool,
}
impl State {
    fn open(self) -> bool {
        self.hovered || self.keyboard
    }
}
impl<'a, M: 'a> Interaction<'a, M> {
    pub(crate) fn new(
        trigger: Element<'a, M, Theme, Renderer>,
        panel: Element<'a, M, Theme, Renderer>,
        position: anchored::Position,
        delays: (Duration, Duration),
        mode: Mode,
        controlled: Option<bool>,
        on_change: Option<Box<dyn Fn(bool) -> M + 'a>>,
    ) -> Self {
        let mut inner = anchored(Content(trigger).boxed(), Content(panel).boxed())
            .position(position)
            .hover(delays.0, delays.1)
            .on_toggle(Input::Toggle)
            .open(false);
        inner = match mode {
            Mode::Hint => inner.arrow([10., 5.], |theme: &Theme| theme.neutral.foreground.into()),
            Mode::Submenu => inner.min_width(96),
        };
        Self {
            inner: Some(inner),
            controlled,
            on_change,
            mode,
        }
    }
    fn inner(&self) -> &Anchor<'a, M> {
        self.inner
            .as_ref()
            .expect("anchor is installed between updates")
    }
    fn inner_mut(&mut self) -> &mut Anchor<'a, M> {
        self.inner
            .as_mut()
            .expect("anchor is installed between updates")
    }
    fn sync(&mut self, state: State) {
        self.inner = self
            .inner
            .take()
            .map(|inner| inner.open(self.controlled.unwrap_or(state.open())));
    }
}
fn notify<M>(
    previous: bool,
    state: State,
    change: Option<&dyn Fn(bool) -> M>,
    shell: &mut Shell<'_, M>,
) {
    if previous != state.open() {
        if let Some(change) = change {
            shell.publish(change(state.open()));
        }
        shell.invalidate_overlay();
        shell.request_redraw();
    }
}
/// Preserve forwarded messages' receipts and all side effects of the child shell.
fn dispatch<M>(
    shell: &mut Shell<'_, M>,
    mut handle: impl FnMut(&mut Shell<'_, Input<M>>),
    mut toggle: impl FnMut(bool),
) {
    let mut bus = Bus::new();
    {
        let mut local = shell.local(&mut bus);
        handle(&mut local);
        if local.is_event_captured() {
            shell.capture_event();
        }
        shell.invalidate(local.invalidation());
        shell.request_redraw_at(local.redraw_request());
        shell.request_input_method(local.input_method());
        shell.clipboard_mut().merge(local.clipboard_mut());
    }
    for (event, receipt) in bus.drain() {
        match event {
            Input::App(message) => shell.forward(message, receipt),
            Input::Toggle(open) => toggle(open),
        }
    }
}
impl<M> Meta for Interaction<'_, M> {}
impl<'a, M: 'a> Widget<M, Theme, Renderer> for Interaction<'a, M> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Cell<State>>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(Cell::new(State::default()))
    }
    fn diff(&mut self, tree: &mut Tree) {
        self.sync(tree.state.downcast_ref::<Cell<State>>().get());
        tree.diff_children(std::slice::from_mut(self.inner_mut()));
    }
    fn size(&self) -> Size<Length> {
        self.inner().size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.inner_mut()
            .layout(&mut tree.children[0], renderer, limits);
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
        let cell = tree.state.downcast_ref::<Cell<State>>();
        let mut state = cell.get();
        let previous = state.open();
        let tree = &mut tree.children[0];
        let focus = focus::count(|op| {
            self.inner_mut()
                .operate(tree, layout, viewport, renderer, op)
        })
        .focused
        .is_some();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(_)) | Event::Touch(_) => state.pointer = true,
            Event::Keyboard(_) => state.pointer = false,
            _ => {}
        }
        if self.mode == Mode::Hint && focus != state.focused {
            state.keyboard = focus && !state.pointer;
        }
        state.focused = focus;
        if self.mode == Mode::Submenu
            && focus
            && !shell.is_event_captured()
            && matches!(
                event,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(Named::ArrowRight | Named::Enter | Named::Space),
                    ..
                })
            )
            && !self.controlled.unwrap_or(previous)
        {
            state.keyboard = true;
            state.enter = true;
            shell.capture_event();
        } else {
            dispatch(
                shell,
                |shell| {
                    self.inner_mut()
                        .update(tree, event, layout, cursor, renderer, shell, viewport)
                },
                |v| state.hovered = v,
            );
        }
        notify(previous, state, self.on_change.as_deref(), shell);
        cell.set(state);
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        op: &mut dyn Operation,
    ) {
        self.inner_mut()
            .operate(&mut tree.children[0], layout, viewport, renderer, op);
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
        self.inner().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.inner()
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
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
        let cell = tree.state.downcast_ref::<Cell<State>>();
        let mut state = cell.get();
        self.sync(state);
        let Some(inner) = self.inner.as_mut() else {
            return Vec::new();
        };
        let mut overlays = inner.overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
            window,
        );
        if state.enter
            && let Some(last) = overlays.last_mut()
        {
            last.as_overlay_mut()
                .operate(renderer, &mut focus::select(Some(0)));
            state.enter = false;
            cell.set(state);
        }
        overlays
            .into_iter()
            .map(|inner| {
                overlay::Element::new(Box::new(Layer {
                    inner,
                    cell,
                    mode: self.mode,
                    change: self.on_change.as_deref(),
                }))
            })
            .collect()
    }
}
struct Layer<'a, M> {
    inner: overlay::Element<'a, Input<M>, Theme, Renderer>,
    cell: &'a Cell<State>,
    mode: Mode,
    change: Option<&'a dyn Fn(bool) -> M>,
}
impl<M> overlay::Overlay<M, Theme, Renderer> for Layer<'_, M> {
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        self.inner.as_overlay().draw(renderer, theme, style, cursor);
    }
    fn operate(&mut self, renderer: &Renderer, op: &mut dyn Operation) {
        self.inner.as_overlay_mut().operate(renderer, op);
    }
    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, M>,
    ) {
        let mut state = self.cell.get();
        let previous = state.open();
        if self.mode == Mode::Submenu
            && matches!(
                event,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(Named::ArrowLeft),
                    ..
                })
            )
        {
            state.hovered = false;
            state.keyboard = false;
            shell.capture_event();
        } else {
            dispatch(
                shell,
                |local| {
                    self.inner
                        .as_overlay_mut()
                        .update(event, cursor, renderer, local)
                },
                |open| {
                    state.hovered = open;
                    if !open
                        && matches!(
                            event,
                            Event::Keyboard(_)
                                | Event::Mouse(mouse::Event::ButtonPressed(_))
                                | Event::Touch(_)
                        )
                    {
                        state.keyboard = false;
                    }
                },
            );
        }
        notify(previous, state, self.change, shell);
        self.cell.set(state);
    }
    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        self.inner.as_overlay().mouse_interaction(cursor, renderer)
    }
    fn index(&self) -> f32 {
        self.inner.as_overlay().index()
    }
    fn overlay<'b>(
        &'b mut self,
        renderer: &Renderer,
    ) -> Vec<overlay::Element<'b, M, Theme, Renderer>> {
        self.inner
            .as_overlay_mut()
            .overlay(renderer)
            .into_iter()
            .map(|inner| {
                overlay::Element::new(Box::new(Layer {
                    inner,
                    cell: self.cell,
                    mode: self.mode,
                    change: self.change,
                }))
            })
            .collect()
    }
}
