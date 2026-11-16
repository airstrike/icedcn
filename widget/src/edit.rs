//! The editing commands of the text widgets and their right-click menu.
//!
//! An Input, a Textarea and an editable code block run the same commands from
//! their shortcuts and from a menu opened at the pointer by a right press, or
//! at the caret by Shift+F10 and the context menu key. The menu is an overlay
//! of the widget that opened it; it records the chosen command and leaves the
//! event uncaptured, so the widget performs the command on the same event.

use std::fmt;

use iced_core::Length::Fill;
use iced_core::Widget as _;
use iced_core::keyboard::key::{Named, Physical};
use iced_core::keyboard::{self, Key, Modifiers};
use iced_core::shell::Bus;
use iced_core::text;
use iced_core::text::editor::Cursor;
use iced_core::time::{Duration, Instant};
use iced_core::widget::Tree;
use iced_core::{
    Element, Event, Layout, Point, Rectangle, Shell, Size, Vector, layout, mouse, overlay,
    renderer, touch, window,
};
use iced_widget::anchored::{Align, Position, Side};
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;
use icedcn_core::theme::popover::content;

use crate::compose::Slot;
use crate::container::Container;
use crate::menu;
use crate::menu::list::List;

/// An editing command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Command {
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
}

impl Command {
    /// The command a key press runs in a focused text widget: the platform
    /// shortcuts, plus Shift+Delete, Ctrl+Insert and Shift+Insert.
    pub(crate) fn from_key(key: &Key, physical: Physical, modifiers: Modifiers) -> Option<Self> {
        if let Key::Named(named) = key {
            let shift = modifiers.shift() && !modifiers.control() && !modifiers.logo();
            let control = modifiers.control() && !modifiers.shift() && !modifiers.alt();

            // Alt+Backspace undoes on Windows; on macOS, Option+Backspace
            // deletes the word before the caret.
            let alt = modifiers.alt() && !modifiers.control() && !modifiers.shift();

            return match named {
                Named::Backspace if alt && cfg!(target_os = "windows") => Some(Self::Undo),
                Named::Delete if shift && !modifiers.alt() => Some(Self::Cut),
                Named::Insert if control => Some(Self::Copy),
                Named::Insert if shift && !modifiers.alt() => Some(Self::Paste),
                _ => None,
            };
        }

        if !modifiers.command() {
            return None;
        }

        match key.to_latin(physical)?.to_ascii_lowercase() {
            'x' => Some(Self::Cut),
            'c' => Some(Self::Copy),
            'v' if !modifiers.alt() => Some(Self::Paste),
            'a' => Some(Self::SelectAll),
            'z' if modifiers.shift() => Some(Self::Redo),
            'z' => Some(Self::Undo),
            'y' => Some(Self::Redo),
            _ => None,
        }
    }
}

/// Whether a key press opens the menu at the caret: Shift+F10, or the context
/// menu key.
pub(crate) fn opens_menu(key: &Key, modifiers: Modifiers) -> bool {
    match key {
        Key::Named(Named::F10) => modifiers.shift() && !modifiers.command(),
        Key::Named(Named::ContextMenu) => true,
        _ => false,
    }
}

/// Whether a right press that put the caret at `position` landed inside the
/// selection of `before`, which the press then keeps.
pub(crate) fn is_inside(before: Cursor, position: text::Position) -> bool {
    let Some(anchor) = before.selection else {
        return false;
    };
    let key = |position: text::Position| (position.line, position.index);
    let (start, end) = if key(anchor) < key(before.position) {
        (anchor, before.position)
    } else {
        (before.position, anchor)
    };

    (key(start)..=key(end)).contains(&key(position))
}

/// `x` on the nearest whole physical pixel at `scale` physical pixels per
/// logical one.
pub(crate) fn snap(x: f32, scale: f32) -> f32 {
    (x * scale).round() / scale
}

/// Which entries of the menu are enabled. Without `undo`, the menu has no
/// Undo and Redo entries.
#[derive(Debug, Clone, Copy)]
pub struct Available {
    pub undo: Option<bool>,
    pub redo: Option<bool>,
    pub cut: bool,
    pub copy: bool,
    pub paste: bool,
    pub select_all: bool,
}

/// The right-click menu of a text widget, kept in the widget's tree state.
pub struct Menu {
    open: Option<Open>,
    tree: Tree,
    chosen: Option<Command>,
}

#[derive(Debug, Clone, Copy)]
struct Open {
    /// The menu's corner, relative to the widget.
    at: Point,
    /// Opened from the keyboard: the first item takes the focus.
    keyboard: bool,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            open: None,
            tree: Tree::empty(),
            chosen: None,
        }
    }
}

impl Clone for Menu {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl fmt::Debug for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Menu")
            .field("open", &self.open)
            .field("chosen", &self.chosen)
            .finish_non_exhaustive()
    }
}

impl Menu {
    /// Open the menu with its corner at `at`, relative to the widget.
    pub(crate) fn open(&mut self, at: Point, keyboard: bool, shell: &mut Shell<'_, impl Sized>) {
        self.open = Some(Open { at, keyboard });
        self.tree = Tree::empty();
        self.chosen = None;
        shell.invalidate_overlay();
        shell.request_redraw();
    }

    /// Take the command chosen in the menu, if any.
    pub(crate) fn take(&mut self) -> Option<Command> {
        self.chosen.take()
    }

    /// The open menu over a widget laid out at `bounds`.
    pub(crate) fn overlay<'b, Message>(
        &'b mut self,
        available: Available,
        bounds: Rectangle,
        translation: Vector,
        renderer: &Renderer,
        window: Size,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let Menu { open, tree, chosen } = self;
        let Open { at, keyboard } = (*open)?;

        let mut panel = panel(available, keyboard);
        tree.diff(&mut panel);

        // Align the menu's top edge with the pointer and leave a two-pixel gap to its right.
        let position = Position::new(Side::Right).align(Align::Start).offset(2);
        let target = Rectangle::new(
            bounds.position() + translation + Vector::new(at.x, at.y),
            Size::ZERO,
        );
        let boundary = Rectangle::with_size(window);
        let space = position.available(target, boundary);
        // Wide enough for the longest label and its shortcut on one line.
        let width = 192f32.min(space.width);
        panel.layout(
            tree,
            renderer,
            &layout::Limits::new(Size::new(width, 0.0), Size::new(width, space.height)),
        );
        let placement = position.place(target, tree.size, boundary).bounds();

        Some(overlay::Element::new(Box::new(Overlay {
            panel,
            tree,
            open,
            chosen,
            layout: Layout::new(placement.size()).move_to(placement.position()),
            window,
        })))
    }
}

/// The theme and renderer a menu is drawn with, for the text input, which is
/// generic over both.
pub trait Themed<Message, Renderer>: Sized {
    /// The open `menu` over a widget laid out at `bounds`.
    fn menu<'b>(
        menu: &'b mut Menu,
        available: Available,
        bounds: Rectangle,
        translation: Vector,
        renderer: &Renderer,
        window: Size,
    ) -> Option<overlay::Element<'b, Message, Self, Renderer>>;
}

impl<Message> Themed<Message, Renderer> for Theme {
    fn menu<'b>(
        menu: &'b mut Menu,
        available: Available,
        bounds: Rectangle,
        translation: Vector,
        renderer: &Renderer,
        window: Size,
    ) -> Option<overlay::Element<'b, Message, Self, Renderer>> {
        menu.overlay(available, bounds, translation, renderer, window)
    }
}

/// The menu panel: `min-w-36 p-1 rounded-lg bg-popover shadow-md ring-1`.
fn panel<'b>(available: Available, keyboard: bool) -> Element<'b, Command, Theme, Renderer> {
    let mac = cfg!(target_os = "macos");
    let item = |label: &'static str,
                shortcut: (&'static str, &'static str),
                command: Command,
                enabled: bool| {
        Slot::from(
            menu::item(label)
                .shortcut(if mac { shortcut.0 } else { shortcut.1 })
                .on_select(command)
                .disabled(!enabled),
        )
    };

    let mut items = Vec::new();
    if let Some(undo) = available.undo {
        let redo = available.redo.unwrap_or(undo);
        items.push(item("Undo", ("⌘Z", "Ctrl+Z"), Command::Undo, undo));
        items.push(item("Redo", ("⇧⌘Z", "Ctrl+Y"), Command::Redo, redo));
        items.push(menu::separator());
    }
    items.extend([
        item("Cut", ("⌘X", "Ctrl+X"), Command::Cut, available.cut),
        item("Copy", ("⌘C", "Ctrl+C"), Command::Copy, available.copy),
        item("Paste", ("⌘V", "Ctrl+V"), Command::Paste, available.paste),
        menu::separator(),
        item(
            "Select All",
            ("⌘A", "Ctrl+A"),
            Command::SelectAll,
            available.select_all,
        ),
    ]);

    let panel = Container::new(menu::group(items))
        .padding(4)
        .width(Fill)
        .style(content);

    List {
        content: Slot::from(panel).compose(),
        horizontal: Some(false),
        autofocus: keyboard.into(),
        on_move: None,
    }
    .boxed()
}

struct Overlay<'b> {
    panel: Element<'b, Command, Theme, Renderer>,
    tree: &'b mut Tree,
    open: &'b mut Option<Open>,
    chosen: &'b mut Option<Command>,
    layout: Layout,
    window: Size,
}

impl<Message> overlay::Overlay<Message, Theme, Renderer> for Overlay<'_> {
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        let viewport = Rectangle::with_size(self.window);

        renderer::Renderer::with_layer(renderer, viewport, |renderer| {
            self.panel.draw(
                self.tree,
                renderer,
                theme,
                style,
                self.layout,
                cursor,
                &viewport,
            );
        });
    }

    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn iced_core::widget::Operation) {
        self.panel.operate(
            self.tree,
            self.layout,
            &Rectangle::with_size(self.window),
            renderer,
            operation,
        );
    }

    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        let is_over = cursor.is_over(self.layout.bounds());

        // The menu is modal: Escape or a press anywhere else only closes it.
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Escape),
                ..
            })
            | Event::Mouse(mouse::Event::ButtonPressed(_))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if !is_over || matches!(event, Event::Keyboard(_)) =>
            {
                *self.open = None;
                shell.invalidate_overlay();
                shell.request_redraw();
                shell.capture_event();
                return;
            }
            Event::Window(window::Event::Unfocused) => {
                *self.open = None;
                shell.invalidate_overlay();
                shell.request_redraw();
                return;
            }
            _ => {}
        }

        let mut bus = Bus::new();
        let mut local = shell.local(&mut bus);

        self.panel.update(
            self.tree,
            event,
            self.layout,
            cursor,
            renderer,
            &mut local,
            &Rectangle::with_size(self.window),
        );

        let is_captured = local.is_event_captured();
        shell.request_redraw_at(local.redraw_request());
        shell.invalidate(local.invalidation());

        if let Some((command, _)) = bus.drain().last() {
            *self.chosen = Some(command);
            *self.open = None;
            shell.invalidate_overlay();
            shell.request_redraw();
            return;
        }

        if is_captured
            || matches!(
                event,
                Event::Mouse(_)
                    | Event::Touch(_)
                    | Event::Keyboard(
                        keyboard::Event::KeyPressed { .. } | keyboard::Event::KeyReleased { .. }
                    )
            )
        {
            shell.capture_event();
        }
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        let interaction = self.panel.mouse_interaction(
            self.tree,
            self.layout,
            cursor,
            &Rectangle::with_size(self.window),
            renderer,
        );

        // A modal layer covers the window.
        if interaction == mouse::Interaction::None {
            mouse::Interaction::Idle
        } else {
            interaction
        }
    }
}

/// A drag selection in an editor that keeps going while the pointer is held
/// past the visible text: each frame drags again at the pointer, which moves
/// the caret past the edge and scrolls it into view.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Autoscroll {
    /// The pointer of a drag that started in the editor, if one is on,
    /// relative to the viewport: where it stays while a scroller moves the
    /// content under it.
    pointer: Option<Vector>,
    /// The frame that last dragged.
    last: Option<Instant>,
    /// The next move is the drag a frame made, not the pointer's.
    replaying: bool,
}

impl Autoscroll {
    /// Follow `event`. A drag sees the pointer wherever it goes, even where
    /// a scroller hides it from its content; returns the cursor to update
    /// the editor with.
    pub(crate) fn track(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        bounds: Rectangle,
        viewport: &Rectangle,
    ) -> mouse::Cursor {
        let origin = viewport.position();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                self.pointer = cursor.position_over(bounds).map(|p| p - origin);
                self.last = None;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Window(window::Event::Unfocused) => {
                *self = Self::default();
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) if self.replaying => {
                self.replaying = false;
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) if self.pointer.is_some() => {
                let pointer = cursor.observe().position().map(|p| p - origin);
                if pointer.is_some() && pointer != self.pointer {
                    self.pointer = pointer;
                    self.last = None;
                }
            }
            _ => {}
        }

        if self.pointer.is_some() {
            cursor.observe()
        } else {
            cursor
        }
    }

    /// On a frame of a drag held past the edges of `visible`, the pointer to
    /// drag to again, when enough time has passed: the farther past, the
    /// sooner. Asks for the next frame meanwhile.
    pub(crate) fn tick<Message>(
        &mut self,
        event: &Event,
        visible: Rectangle,
        viewport: &Rectangle,
        shell: &mut Shell<'_, Message>,
    ) -> Option<Point> {
        let Event::Window(window::Event::RedrawRequested(now)) = event else {
            return None;
        };
        let pointer = viewport.position() + self.pointer?;
        let past = Vector::new(
            (visible.x - pointer.x).max(pointer.x - visible.x - visible.width),
            (visible.y - pointer.y).max(pointer.y - visible.y - visible.height),
        );
        let past = past.x.max(past.y);
        if past <= 0.0 {
            self.last = None;
            return None;
        }
        shell.request_redraw();

        let Some(last) = self.last else {
            self.last = Some(*now);
            return None;
        };
        let interval = Duration::from_secs_f32((2.0 / past).clamp(0.016, 0.2));
        if *now < last + interval {
            return None;
        }
        self.last = Some(*now);
        self.replaying = true;
        Some(pointer)
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, snap};
    use iced_core::keyboard::key::{Code, Named, Physical};
    use iced_core::keyboard::{Key, Modifiers};

    #[test]
    fn undo_and_redo_keys_follow_the_platform() {
        let press = |key: Key, code: Code, modifiers: Modifiers| {
            Command::from_key(&key, Physical::Code(code), modifiers)
        };
        let z = || Key::Character("z".into());
        assert_eq!(
            press(z(), Code::KeyZ, Modifiers::COMMAND),
            Some(Command::Undo)
        );
        assert_eq!(
            press(
                Key::Character("Z".into()),
                Code::KeyZ,
                Modifiers::COMMAND | Modifiers::SHIFT
            ),
            Some(Command::Redo)
        );
        assert_eq!(
            press(Key::Character("y".into()), Code::KeyY, Modifiers::COMMAND),
            Some(Command::Redo)
        );
        // Option+Backspace deletes a word on macOS; Alt+Backspace undoes on
        // Windows.
        assert_eq!(
            press(
                Key::Named(Named::Backspace),
                Code::Backspace,
                Modifiers::ALT
            ),
            cfg!(target_os = "windows").then_some(Command::Undo)
        );
    }

    #[test]
    fn snapping_lands_on_whole_physical_pixels() {
        for scale in [1.0, 1.25, 1.5, 2.0, 3.0] {
            for x in [0.0, 0.3, 10.5, 17.49, 101.77, -4.6] {
                let physical = snap(x, scale) * scale;
                assert!((physical - physical.round()).abs() < 1e-3, "{x} at {scale}");
                assert!((snap(x, scale) - x).abs() <= 0.5 / scale + 1e-4);
            }
        }
    }
}
