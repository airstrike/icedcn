use super::{Event, Shortcuts};
use crate::operation::focus;
use iced_core::Renderer as _;
use iced_core::keyboard::{self, Key};
use iced_core::time::Instant;
use iced_core::widget::{
    Meta, Operation,
    tree::{self, Tree},
};
use iced_core::window;
use iced_core::{
    Element, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Renderer;
use icedxdy_core::Theme;

pub(super) struct Body<'a> {
    pub content: Element<'a, Event, Theme, Renderer>,
    pub choices: usize,
    pub shortcuts: Shortcuts,
    pub step: String,
    pub animate: bool,
    pub invalid: bool,
}
struct State {
    step: String,
    start: Option<Instant>,
    phase: f32,
    focus_first: bool,
    invalid: bool,
}
impl Meta for Body<'_> {}
impl Widget<Event, Theme, Renderer> for Body<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State {
            step: self.step.clone(),
            start: None,
            phase: 1.,
            focus_first: false,
            invalid: false,
        })
    }
    fn diff(&mut self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();
        state.focus_first |= self.invalid && !state.invalid;
        state.invalid = self.invalid;
        if state.step != self.step {
            state.focus_first = true;
            state.step = self.step.clone();
            state.phase = if self.animate { 0. } else { 1. };
            state.start = None;
            // A new item gets a fresh set of controls and input caret.
            tree.children.clear();
        }
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }
    fn size(&self) -> Size<Length> {
        self.content.size()
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced_core::Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Event>,
        viewport: &Rectangle,
    ) {
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("the questionnaire item has one child");

        if std::mem::take(&mut tree.state.downcast_mut::<State>().focus_first) {
            self.content.operate(
                child_tree,
                child_layout,
                viewport,
                renderer,
                &mut focus::select(Some(0)),
            );
            shell.request_redraw();
        }
        // The focused editor consumes character input before shortcuts see it.
        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
        let state = tree.state.downcast_mut::<State>();
        if self.animate
            && state.phase < 1.
            && let iced_core::Event::Window(window::Event::RedrawRequested(now)) = event
        {
            let start = *state.start.get_or_insert(*now);
            state.phase = (now.saturating_duration_since(start).as_secs_f32() / 0.3).clamp(0., 1.);
            if state.phase < 1. {
                shell.request_redraw();
            }
        }
        if shell.is_event_captured() || self.shortcuts == Shortcuts::None {
            return;
        }
        if let iced_core::Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Character(key),
            modifiers,
            ..
        }) = event
            && !modifiers.control()
            && !modifiers.alt()
            && !modifiers.logo()
            && (cursor.is_over(layout.bounds())
                || focus::count(|op| {
                    self.content
                        .operate(child_tree, child_layout, viewport, renderer, op)
                })
                .focused
                .is_some())
            && let Some(i) = (0..self.choices).find(|&i| {
                self.shortcuts
                    .key(i)
                    .is_some_and(|ch| key.eq_ignore_ascii_case(&ch.to_string()))
            })
        {
            shell.publish(Event::Choose(i));
            shell.capture_event();
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
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("the questionnaire item has one child");

        let phase = tree.state.downcast_ref::<State>().phase;
        let dy = 8. * (1. - crate::animation::ease_out(phase));
        renderer.with_translation(Vector::new(0., dy), |renderer| {
            self.content.draw(
                child_tree,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            )
        });
        if phase < 1. {
            let background: iced_core::Color = theme.neutral.base.into();
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    ..renderer::Quad::default()
                },
                background.scale_alpha(1. - phase),
            );
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
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("the questionnaire item has one child");

        self.content
            .mouse_interaction(child_tree, child_layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("the questionnaire item has one child");

        self.content
            .operate(child_tree, child_layout, viewport, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Event, Theme, Renderer>> {
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("the questionnaire item has one child");

        self.content.overlay(
            child_tree,
            child_layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}
