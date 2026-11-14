//! Combobox tree-time focus and active-option visibility.

use iced_core::alignment::Vertical;
use iced_core::keyboard::{self, Key, key::Named};
use iced_core::length::Sizing;
use iced_core::text::{self, Paragraph as _};
use iced_core::touch;
use iced_core::widget::operation::TextInput;
use iced_core::widget::{Id, Meta, Operation, Tree, tree};
use iced_core::{
    Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse, overlay,
    renderer,
};
use iced_widget::Renderer;
use icedcn_core::Theme;

use crate::operation::focus;
use crate::operation::scroll::Reveal;

pub(super) struct Behavior<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    width: Option<Length>,
    intrinsic: Option<(f32, f32)>,
    autofocus: bool,
    focus_epoch: u64,
    focus_index: usize,
    reveal: Option<(Id, Id)>,
    dismiss: Option<Message>,
    escape: Option<Message>,
    remove: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    empty: bool,
    open_on_input: Option<Message>,
}

impl<'a, Message> Behavior<'a, Message> {
    pub fn field(
        content: Element<'a, Message, Theme, Renderer>,
        focus_epoch: u64,
        focus_index: usize,
        width: Length,
        intrinsic: Option<(f32, f32)>,
    ) -> Self {
        Self {
            content,
            width: Some(width),
            intrinsic,
            autofocus: false,
            focus_epoch,
            focus_index,
            reveal: None,
            dismiss: None,
            escape: None,
            remove: None,
            empty: false,
            open_on_input: None,
        }
    }

    pub fn panel(
        content: Element<'a, Message, Theme, Renderer>,
        autofocus: bool,
        reveal: Option<(Id, Id)>,
        dismiss: Message,
        escape: Message,
    ) -> Self {
        Self {
            content,
            width: None,
            intrinsic: None,
            autofocus,
            focus_epoch: 0,
            focus_index: 0,
            reveal,
            dismiss: Some(dismiss),
            escape: Some(escape),
            remove: None,
            empty: false,
            open_on_input: None,
        }
    }
    pub fn open_on_input(mut self, message: Message) -> Self {
        self.open_on_input = Some(message);
        self
    }
    pub fn chips(mut self, empty: bool, remove: impl Fn(usize) -> Message + 'a) -> Self {
        self.empty = empty;
        self.remove = Some(Box::new(remove));
        self
    }
}

struct State {
    initialized: bool,
    focus_epoch: u64,
    revealed: Option<Id>,
}

impl<Message> Meta for Behavior<'_, Message> {}
impl<Message: Clone> Widget<Message, Theme, Renderer> for Behavior<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(State {
            initialized: false,
            focus_epoch: 0,
            revealed: None,
        })
    }
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }
    fn size(&self) -> Size<Length> {
        let mut size = self.content.size();
        if let Some(width) = self.width {
            size.width = width;
        }
        size
    }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let limits = if let Some(width) = self.width {
            let limits = limits.width(width);
            if let Some((text_size, extra)) = self.intrinsic
                && matches!(
                    width,
                    Length::Shrink
                        | Length::Bounded {
                            sizing: Sizing::Shrink,
                            ..
                        }
                )
            {
                // HTML inputs default to size=20. Derive ch from the renderer's
                // actual font instead of copying a screenshot's pixel width.
                let paragraph = <Renderer as text::Renderer>::Paragraph::with_text(text::Text {
                    content: "0",
                    bounds: Size::new(f32::INFINITY, f32::INFINITY),
                    size: text_size.into(),
                    line_height: text::LineHeight::default(),
                    font: text::Renderer::font(renderer),
                    align_x: text::Alignment::Left,
                    align_y: Vertical::Top,
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                    ellipsis: text::Ellipsis::default(),
                    letter_spacing: Default::default(),
                    font_features: Vec::new(),
                    font_variations: Vec::new(),
                    hint_factor: iced_core::Renderer::hint_factor(renderer),
                    weight: None,
                });
                let ch = paragraph.min_width();
                // Null renderer has no font metrics; retain a usable test layout.
                limits.width(20.0 * if ch > 0.0 { ch } else { text_size * 0.5 } + extra)
            } else {
                limits
            }
        } else {
            *limits
        };
        self.content
            .layout(&mut tree.children[0], renderer, &limits);
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
        let (child_layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("combobox behavior has one child");
        let state = tree.state.downcast_mut::<State>();
        let mut visit = |op: &mut dyn Operation| {
            self.content
                .operate(child, child_layout, viewport, renderer, op)
        };
        if (!state.initialized && self.autofocus) || state.focus_epoch != self.focus_epoch {
            visit(&mut focus::select(Some(self.focus_index)));
            state.focus_epoch = self.focus_epoch;
            shell.request_redraw();
        }
        state.initialized = true;
        if let Some((scroll, target)) = &self.reveal {
            if state.revealed.as_ref() != Some(target) {
                let mut find = Find {
                    target: target.clone(),
                    bounds: None,
                };
                visit(&mut find);
                if let Some(bounds) = find.bounds {
                    visit(&mut Reveal {
                        scroll: scroll.clone(),
                        target: bounds,
                    });
                    state.revealed = Some(target.clone());
                    shell.request_redraw();
                }
            }
        } else {
            state.revealed = None;
        }
        if let Some(open) = &self.open_on_input
            && matches!(
                event,
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    | Event::Touch(touch::Event::FingerPressed { .. })
            )
            && cursor.is_over(layout.bounds())
        {
            let mut hit = InputHit {
                cursor,
                inside: false,
            };
            visit(&mut hit);
            if hit.inside {
                shell.publish(open.clone());
            }
        }
        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Named(key),
            ..
        }) = event
        {
            if *key == Named::Escape
                && let Some(message) = &self.escape
            {
                shell.publish(message.clone());
                shell.capture_event();
                return;
            }
            if *key == Named::Tab
                && let Some(message) = &self.dismiss
            {
                shell.publish(message.clone());
            }
            if let Some(remove) = &self.remove {
                let count = focus::count(&mut visit);
                if let Some(index) = count.focused {
                    let target = match key {
                        Named::ArrowLeft
                            if index > 0 && (index < self.focus_index || self.empty) =>
                        {
                            Some(index - 1)
                        }
                        Named::ArrowRight if index < self.focus_index => Some(index + 1),
                        _ => None,
                    };
                    if let Some(target) = target {
                        visit(&mut focus::select(Some(target)));
                        shell.request_redraw();
                        shell.capture_event();
                        return;
                    }
                    if (*key == Named::Backspace || *key == Named::Delete)
                        && index < self.focus_index
                    {
                        shell.publish(remove(index));
                        shell.capture_event();
                        return;
                    }
                    if *key == Named::Backspace
                        && self.empty
                        && index == self.focus_index
                        && index > 0
                    {
                        shell.publish(remove(index - 1));
                        shell.capture_event();
                        return;
                    }
                }
            }
        }
        self.content.update(
            child,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
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
        let (child_layout, child) = layout
            .iter(&tree.children)
            .next()
            .expect("combobox behavior has one child");
        self.content.draw(
            child,
            renderer,
            theme,
            style,
            child_layout,
            cursor,
            viewport,
        );
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("combobox behavior has one child");
        self.content
            .operate(child, layout, viewport, renderer, operation);
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
            .expect("combobox behavior has one child");
        self.content
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
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let (layout, child) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("combobox behavior has one child");
        self.content
            .overlay(child, layout, renderer, viewport, translation, window)
    }
}

struct Find {
    target: Id,
    bounds: Option<Rectangle>,
}
impl Operation for Find {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn container(&mut self, id: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
        if id == Some(&self.target) {
            self.bounds = Some(bounds);
        }
    }
}

struct InputHit {
    cursor: mouse::Cursor,
    inside: bool,
}
impl Operation for InputHit {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn text_input(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn TextInput) {
        self.inside |= self.cursor.is_over(bounds);
    }
}
