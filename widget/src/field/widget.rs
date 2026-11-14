//! The widget behind [`Field`](super::Field): it links the field's label to
//! its control and paints the optional card chrome.
//!
//! - **Label to control.** The label sits in a [`marker`] recording a
//!   [`Label`] signal. After every event the field drains those signals and,
//!   when its label was pressed, runs [`control::activate`] over its content:
//!   a checkbox, switch or radio toggles as if clicked, an input takes focus.
//!   This is shadcn's `<FieldLabel htmlFor>`.
//! - **Card.** The complete padded surface is a label. Its tint follows the
//!   control immediately, as Nova FieldLabel has no transition class. Keyboard
//!   focus decorates the card instead of the nested Checkbox.
//!
//! Style sources stay catalog `StyleFn`s, resolved every frame, so palette
//! swaps flow through.

use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::widget::Id;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::{
    Background, Border, Color, Element, Event, Layout, Length, Padding, Rectangle, Shell, Size,
    Vector, Widget, overlay,
};
use crate::marker;
use crate::operation::{control, focus};
use std::any::Any;

use iced_widget::container::StyleFn;
use icedxdy_core::Theme;

/// The signal a field's label marker records when pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label;

/// The card chrome for the unchecked and checked states.
struct Card<'a> {
    inactive: StyleFn<'a, Theme>,
    active: StyleFn<'a, Theme>,
}

/// A laid-out field: label, control, description and error, composed by
/// the [`Field`](super::Field) builder.
///
/// Generic over `Renderer` so we don't bake in the active iced-renderer
/// fallback type — `iced_widget::Renderer` is a feature-gated alias whose
/// concrete type changes with build features (and is `()` with no
/// renderer feature).
pub struct Field<'a, Message, Renderer = iced_widget::Renderer>
where
    Renderer: crate::core::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    card: Option<Card<'a>>,
}

impl<'a, Message: 'a, Renderer> Field<'a, Message, Renderer>
where
    Renderer: crate::core::Renderer,
{
    /// Wrap the composed field. Its label must sit in a [`marker`] recording
    /// [`Label`].
    pub fn new(content: Element<'a, Message, Theme, Renderer>) -> Self {
        Self {
            content,
            card: None,
        }
    }

    /// Paint card chrome and make its complete padded surface a label.
    pub fn card(
        mut self,
        inactive: StyleFn<'a, Theme>,
        active: StyleFn<'a, Theme>,
        padding: impl Into<icedxdy_core::Padding>,
    ) -> Self {
        let padding: icedxdy_core::Padding = padding.into();
        let padding: Padding = padding.into();
        let padding = Padding {
            top: padding.top + 1.0,
            right: padding.right + 1.0,
            bottom: padding.bottom + 1.0,
            left: padding.left + 1.0,
        };
        self.content = marker::marker(
            iced_widget::container(self.content)
                .padding(padding)
                ._boxed(),
            Label,
            marker::Press::Any,
        )
        .into();
        self.card = Some(Card { inactive, active });
        self
    }
}

/// Observations of the nested control, shared by layout, input and focus operations.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
struct State {
    active: bool,
    focused: bool,
    disabled: bool,
    hovered: bool,
}

#[derive(Default)]
struct Probe {
    checked: control::Checked,
    focused: bool,
    disabled: bool,
}

impl Operation for Probe {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        if let Some(ring) = state.downcast_mut::<focus::Ring>() {
            ring.delegated = true;
            self.focused |= ring.visible;
            self.disabled |= ring.disabled;
        }
        self.checked.custom(id, bounds, state);
    }
}

impl State {
    fn observe(&mut self, probe: Probe) {
        self.active = probe.checked.get().unwrap_or(false);
        self.focused = probe.focused;
        self.disabled = probe.disabled;
    }
}

impl<Message, Renderer> Meta for Field<'_, Message, Renderer> where Renderer: crate::core::Renderer {}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Field<'_, Message, Renderer>
where
    Renderer: crate::core::Renderer,
{
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
        let child = &mut tree.children[0];
        self.content.layout(child, renderer, limits);
        tree.size = child.size;
        if self.card.is_some() {
            let mut probe = Probe::default();
            self.content.operate(
                child,
                Layout::new(child.size),
                &Rectangle::with_size(tree.size),
                renderer,
                &mut probe,
            );
            tree.state.downcast_mut::<State>().observe(probe);
        }
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
            .expect("a field has one child");

        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        let mut labels = marker::drain::<Label>();
        self.content
            .operate(child_tree, child_layout, viewport, renderer, &mut labels);

        if !labels.into_signals().is_empty() {
            // The control acts on the activation at its next event.
            let mut activate = control::activate();
            self.content
                .operate(child_tree, child_layout, viewport, renderer, &mut activate);

            if activate.is_done() {
                shell.request_redraw();
            }
        }

        if self.card.is_none() {
            return;
        }

        let mut probe = Probe::default();
        self.content
            .operate(child_tree, child_layout, viewport, renderer, &mut probe);
        let state = tree.state.downcast_mut::<State>();
        let before = *state;
        state.observe(probe);
        state.hovered =
            !state.disabled && cursor.is_over(layout.bounds()) && cursor.is_over(*viewport);
        if *state != before {
            shell.request_redraw();
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
            .expect("a field has one child");

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
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            let (child_layout, child_tree) = layout
                .iter_mut(&mut tree.children)
                .next()
                .expect("a field has one child");

            self.content
                .operate(child_tree, child_layout, viewport, renderer, operation);
        });
        if self.card.is_some() {
            let (child_layout, child_tree) = layout
                .iter_mut(&mut tree.children)
                .next()
                .expect("a field has one child");
            let mut probe = Probe::default();
            self.content
                .operate(child_tree, child_layout, viewport, renderer, &mut probe);
            tree.state.downcast_mut::<State>().observe(probe);
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        renderer_style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        let Some(card) = &self.card else {
            let (child_layout, child_tree) = layout
                .iter(&tree.children)
                .next()
                .expect("a field has one child");

            self.content.draw(
                child_tree,
                renderer,
                theme,
                renderer_style,
                child_layout,
                cursor,
                viewport,
            );
            return;
        };

        let mut style = if state.active {
            (card.active)(theme)
        } else {
            (card.inactive)(theme)
        };
        if state.hovered && !state.disabled {
            style.background = Some(Background::Color(theme.muted.base.alpha(0.5).into()));
        }
        if state.focused {
            style.border.color = theme.neutral.ring.into();
            let mut radius = style.border.radius;
            radius.top_left += 3.0;
            radius.top_right += 3.0;
            radius.bottom_left += 3.0;
            radius.bottom_right += 3.0;
            renderer.fill_quad(
                renderer::Quad {
                    bounds: bounds.expand(3.0),
                    border: Border {
                        color: theme.neutral.ring.alpha(0.5).into(),
                        width: 3.0,
                        radius,
                    },
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }

        if let Some(clipped_viewport) = bounds.intersection(viewport) {
            // Mirror iced's container draw_background — paint the bg fill
            // and border via a single quad. Matches what `style.background`
            // expects.
            if let Some(background) = style.background {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        border: style.border,
                        ..renderer::Quad::default()
                    },
                    background,
                );
            } else if style.border.width > 0.0 {
                // Border-only (no fill) — paint a transparent quad just for
                // its border. iced's `draw_background` does the same when
                // `background` is `None` but the border is non-zero.
                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        border: style.border,
                        ..renderer::Quad::default()
                    },
                    Background::Color(Color::TRANSPARENT),
                );
            }

            let (child_layout, child_tree) = layout
                .iter(&tree.children)
                .next()
                .expect("a field has one child");

            self.content.draw(
                child_tree,
                renderer,
                theme,
                &renderer::Style {
                    text_color: style.text_color.unwrap_or(renderer_style.text_color),
                },
                child_layout,
                cursor,
                &clipped_viewport,
            );
        }
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
            .expect("a field has one child");

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

impl<'a, Message: 'a, Renderer> From<Field<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer + 'a,
{
    fn from(field: Field<'a, Message, Renderer>) -> Self {
        field._boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::{Field, State};
    use crate::checkbox::widget::Checkbox;
    use crate::operation::focus;
    use iced_core::keyboard::key::Named;
    use iced_core::shell::{Bus, Waker};
    use iced_core::widget::Tree;
    use iced_core::{Event, Layout, Point, Rectangle, Shell, Size, Widget, layout, mouse, window};
    use iced_test::simulator;
    use icedxdy_core::Theme;
    use icedxdy_core::theme::field::{Card, CardActive};

    fn card(checked: bool, disabled: bool) -> (Field<'static, (), ()>, Tree) {
        let checkbox: Checkbox<'_, (), Theme, ()> = Checkbox::new(checked)
            .uncontrolled(checked)
            .disabled(disabled);
        let mut field = Field::new(checkbox.into()).card(Card.into(), CardActive.into(), 10);
        let mut tree = Tree::new(&field);
        tree.diff(&mut field);
        field.layout(
            &mut tree,
            &(),
            &layout::Limits::new(Size::ZERO, Size::new(200.0, 100.0)),
        );
        (field, tree)
    }

    fn step(field: &mut Field<'_, (), ()>, tree: &mut Tree, event: Event, position: Point) {
        let mut messages = Bus::new();
        let mut shell = Shell::new(&window::Headless, Waker::noop(), &mut messages);
        let layout = Layout::new(tree.size);
        field.update(
            tree,
            &event,
            layout,
            mouse::Cursor::Available(position),
            &(),
            &mut shell,
            &Rectangle::with_size(Size::new(200.0, 100.0)),
        );
    }

    #[test]
    fn card_mount_and_pointer_change_have_no_stale_tint() {
        for initial in [false, true] {
            let (mut field, mut tree) = card(initial, false);
            assert_eq!(tree.state.downcast_ref::<State>().active, initial);
            assert_eq!(tree.size, Size::new(38.0, 38.0));
            for event in simulator::click() {
                step(&mut field, &mut tree, event, Point::new(19.0, 19.0));
            }
            assert_eq!(tree.state.downcast_ref::<State>().active, !initial);
            assert!(tree.state.downcast_ref::<State>().hovered);
            assert!(!tree.state.downcast_ref::<State>().focused);
        }
    }

    #[test]
    fn card_observes_keyboard_focus_and_space_then_pointer_modality() {
        let (mut field, mut tree) = card(false, false);
        let layout = Layout::new(tree.size);
        field.operate(
            &mut tree,
            layout,
            &Rectangle::with_size(Size::new(200.0, 100.0)),
            &(),
            &mut focus::select(Some(0)),
        );
        assert!(tree.state.downcast_ref::<State>().focused);
        for event in [
            simulator::press_key(Named::Space, None),
            simulator::release_key(Named::Space),
        ] {
            step(&mut field, &mut tree, event, Point::new(150.0, 80.0));
        }
        assert!(tree.state.downcast_ref::<State>().active);
        for event in simulator::click() {
            step(&mut field, &mut tree, event, Point::new(19.0, 19.0));
        }
        let state = tree.state.downcast_ref::<State>();
        assert!(!state.focused);
        assert!(!state.active);
    }

    #[test]
    fn disabled_nested_checkbox_prevents_card_hover_and_focus() {
        let (mut field, mut tree) = card(false, true);
        let layout = Layout::new(tree.size);
        field.operate(
            &mut tree,
            layout,
            &Rectangle::with_size(Size::new(200.0, 100.0)),
            &(),
            &mut focus::select(Some(0)),
        );
        for event in simulator::click() {
            step(&mut field, &mut tree, event, Point::new(19.0, 19.0));
        }
        let state = tree.state.downcast_ref::<State>();
        assert!(state.disabled);
        assert!(!state.focused && !state.hovered && !state.active);
    }
}
