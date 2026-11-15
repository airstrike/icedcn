use iced_core::time::{Duration, Instant};
use iced_core::{Point, touch, window};

use iced_core::widget::{
    Meta, Operation,
    tree::{self, Tree},
};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

/// Opens at a right press or long touch, leaving normal pointer motion to the content.
pub(super) struct Trigger<'a, Message> {
    pub content: Element<'a, Message, Theme, Renderer>,
    pub open: fn(Point) -> Message,
}
#[derive(Default)]
struct State {
    touch: Option<(touch::Finger, Point, Instant)>,
    fired: Option<touch::Finger>,
}
impl<Message> Meta for Trigger<'_, Message> {}
impl<Message> Widget<Message, Theme, Renderer> for Trigger<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }
    fn diff(&mut self, tree: &mut Tree) {
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
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a context menu trigger has one child");

        let state = tree.state.downcast_mut::<State>();
        if let Event::Touch(
            touch::Event::FingerLifted { id, .. } | touch::Event::FingerLost { id, .. },
        ) = *event
            && state.fired == Some(id)
        {
            state.fired = None;
            shell.capture_event();
            return;
        }

        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        // An inner context menu gets the first chance to finish its long press.
        if shell.is_event_captured()
            && matches!(event, Event::Window(window::Event::RedrawRequested(_)))
        {
            state.touch = None;
            return;
        }

        match *event {
            Event::Touch(touch::Event::FingerPressed { id, position })
                if layout.bounds().contains(position) =>
            {
                if state.touch.is_none() {
                    let deadline = Instant::now() + Duration::from_millis(700);
                    state.touch = Some((id, position, deadline));
                    shell.request_redraw_at(deadline);
                } else {
                    state.touch = None;
                }
            }
            Event::Touch(touch::Event::FingerMoved { id, position }) => {
                if state
                    .touch
                    .is_some_and(|(finger, start, _)| finger == id && start.distance(position) > 8.)
                {
                    state.touch = None;
                }
            }
            Event::Touch(
                touch::Event::FingerLifted { id, .. } | touch::Event::FingerLost { id, .. },
            ) => {
                if state.touch.is_some_and(|(finger, ..)| finger == id) {
                    state.touch = None;
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                if let Some((id, start, deadline)) = state.touch
                    && now >= deadline
                {
                    state.touch = None;
                    state.fired = Some(id);
                    shell.publish((self.open)(Point::new(
                        start.x - layout.bounds().x,
                        start.y - layout.bounds().y,
                    )));
                    // Cancel any pressed descendant; its release must not invoke both actions.
                    self.content.update(
                        child_tree,
                        &Event::Touch(touch::Event::FingerLost {
                            id,
                            position: start,
                        }),
                        child_layout,
                        cursor,
                        renderer,
                        shell,
                        viewport,
                    );
                    shell.capture_event();
                }
            }
            _ => {}
        }

        if !shell.is_event_captured()
            && matches!(
                event,
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right))
            )
            && let Some(position) = cursor.position()
            && layout.bounds().contains(position)
            && viewport.contains(position)
        {
            shell.publish((self.open)(Point::new(
                position.x - layout.bounds().x,
                position.y - layout.bounds().y,
            )));
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
            .expect("a context menu trigger has one child");

        self.content.draw(
            child_tree,
            renderer,
            theme,
            style,
            child_layout,
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
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a context menu trigger has one child");

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
            .expect("a context menu trigger has one child");

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
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a context menu trigger has one child");

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
