//! Clipboard shortcuts and the right-click menu of the Input, the Textarea and
//! the editable code block, driven through a user interface whose clipboard
//! the test owns.
use std::sync::Arc;

use iced::keyboard::key::Named;
use iced::keyboard::{self, Key, Modifiers};
use iced::mouse::{self, Button};
use iced::{Element, Event, Point, Rectangle, Size};
use iced_test::core::clipboard;
use iced_test::core::renderer::Headless;
use iced_test::core::shell::{Bus, Waker};
use iced_test::core::time::Instant;
use iced_test::core::widget::operation::{self, TextInput};
use iced_test::core::widget::{Id, Operation};
use iced_test::core::window;
use iced_test::futures::futures::executor::block_on;
use iced_test::renderer::Renderer;
use iced_test::runtime::user_interface::{self, UserInterface};
use iced_test::{Selector, simulator};
use icedcn::Theme;
use icedcn::widget::{Platform, code_block, column, input, textarea};

/// A user interface with a clipboard of its own: writes land in `clipboard`,
/// and reads are answered from it with the event the runtime sends.
struct Desk {
    ui: UserInterface<'static, String, Theme, Renderer>,
    renderer: Renderer,
    cursor: Point,
    clipboard: Option<String>,
    messages: Vec<String>,
}

impl Desk {
    fn new(view: impl Into<Element<'static, String, Theme>>) -> Self {
        let mut renderer =
            block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
        let ui = UserInterface::build(
            view.into(),
            Size::new(800., 600.),
            user_interface::Cache::default(),
            &mut renderer,
        );
        Self {
            ui,
            renderer,
            cursor: Point::ORIGIN,
            clipboard: None,
            messages: Vec::new(),
        }
    }

    fn send(&mut self, events: impl IntoIterator<Item = Event>) {
        let mut events: Vec<Event> = events.into_iter().collect();
        while !events.is_empty() {
            let mut bus = Bus::new();
            let (state, _) = self.ui.update(
                &window::Headless,
                &Waker::noop(),
                &events,
                mouse::Cursor::Available(self.cursor),
                &mut self.renderer,
                &mut bus,
            );
            self.messages
                .extend(bus.drain().map(|(message, _)| message));
            events.clear();
            if let user_interface::State::Updated { clipboard, .. } = state {
                if let Some(clipboard::Content::Text(text)) = clipboard.write {
                    self.clipboard = Some(text);
                }
                for _ in clipboard.reads {
                    events.push(Event::Clipboard(clipboard::Event::Read(
                        self.clipboard
                            .clone()
                            .map(|text| Arc::new(clipboard::Content::Text(text)))
                            .ok_or(clipboard::Error::ContentNotAvailable),
                    )));
                }
            }
        }
    }

    /// The bounds and text of the one text field.
    fn field(&mut self) -> (Rectangle, String) {
        #[derive(Default)]
        struct Fields(Vec<(Rectangle, String)>);
        impl Operation for Fields {
            fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
                visit(self);
            }
            fn text_input(&mut self, _: Option<&Id>, bounds: Rectangle, input: &mut dyn TextInput) {
                self.0.push((bounds, input.text().into_owned()));
            }
        }
        let mut fields = Fields::default();
        self.ui.operate(&self.renderer, &mut fields);
        assert_eq!(fields.0.len(), 1, "one text field");
        fields.0.pop().unwrap()
    }

    fn text(&mut self) -> String {
        self.field().1
    }

    fn find(&mut self, label: &'static str) -> Option<Rectangle> {
        let mut find = Selector::find(label);
        self.ui
            .operate(&self.renderer, &mut operation::black_box(&mut find));
        match find.finish() {
            operation::Outcome::Some(Some(target)) => target.visible_bounds(),
            _ => None,
        }
    }

    fn click_at(&mut self, position: Point, button: Button) {
        self.cursor = position;
        self.send([
            Event::Mouse(mouse::Event::CursorMoved { position }),
            Event::Mouse(mouse::Event::ButtonPressed(button)),
            Event::Mouse(mouse::Event::ButtonReleased(button)),
        ]);
    }

    /// Focus the field with a click at its start.
    fn focus(&mut self) -> Rectangle {
        let (bounds, _) = self.field();
        self.click_at(Point::new(bounds.x + 14., bounds.y + 10.), Button::Left);
        bounds
    }

    /// Right-click the field near its start, which opens the menu.
    fn right_click(&mut self) {
        let (bounds, _) = self.field();
        self.click_at(Point::new(bounds.x + 14., bounds.y + 10.), Button::Right);
        assert!(self.find("Select All").is_some(), "the menu opened");
    }

    fn choose(&mut self, label: &'static str) {
        let bounds = self
            .find(label)
            .unwrap_or_else(|| panic!("{label} in the menu"));
        self.click_at(bounds.center(), Button::Left);
    }

    fn key(&mut self, key: impl Into<Key>, modifiers: Modifiers) {
        let key = key.into();
        self.send(
            [
                simulator::press_key(key.clone(), None),
                simulator::release_key(key),
            ]
            .map(|event| with(event, modifiers)),
        );
    }
}

fn with(event: Event, modifiers: Modifiers) -> Event {
    match event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            modified_key,
            physical_key,
            location,
            text,
            repeat,
            ..
        }) => Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            modified_key,
            physical_key,
            location,
            modifiers,
            text,
            repeat,
        }),
        Event::Keyboard(keyboard::Event::KeyReleased {
            key,
            modified_key,
            physical_key,
            location,
            ..
        }) => Event::Keyboard(keyboard::Event::KeyReleased {
            key,
            modified_key,
            physical_key,
            location,
            modifiers,
        }),
        event => event,
    }
}

fn character(c: &str) -> Key {
    Key::Character(c.into())
}

/// The three widgets, uncontrolled, each publishing its text.
fn views(value: &'static str) -> [(&'static str, Element<'static, String, Theme>); 3] {
    [
        (
            "input",
            input("").default_value(value).on_input(|s| s).into(),
        ),
        (
            "textarea",
            textarea("").default_value(value).on_input(|s| s).into(),
        ),
        (
            "code block",
            code_block![code_block::content("").default_value(value).on_input(|s| s)].into(),
        ),
    ]
}

#[test]
fn command_v_pastes_and_command_c_x_copy_and_cut_the_selection() {
    for (name, view) in views("hello") {
        let mut desk = Desk::new(view);
        desk.focus();
        desk.key(character("a"), Modifiers::COMMAND);
        desk.key(character("c"), Modifiers::COMMAND);
        assert_eq!(desk.clipboard.as_deref(), Some("hello"), "{name} copies");
        desk.key(character("x"), Modifiers::COMMAND);
        assert_eq!(desk.text(), "", "{name} cuts");
        desk.clipboard = Some("pasted".into());
        desk.key(character("v"), Modifiers::COMMAND);
        assert_eq!(desk.text(), "pasted", "{name} pastes");
        assert_eq!(desk.messages.last().map(String::as_str), Some("pasted"));
    }
}

#[test]
fn shift_delete_cuts_ctrl_insert_copies_and_shift_insert_pastes_on_a_pc() {
    for (platform, (name, view)) in [Platform::Windows, Platform::Linux]
        .into_iter()
        .flat_map(|platform| views("hello").into_iter().map(move |view| (platform, view)))
    {
        Platform::set(Some(platform));
        let mut desk = Desk::new(view);
        desk.focus();
        desk.key(character("a"), Modifiers::CTRL);
        desk.key(Named::Insert, Modifiers::CTRL);
        assert_eq!(desk.clipboard.as_deref(), Some("hello"), "{name} copies");
        assert_eq!(desk.text(), "hello");
        desk.key(Named::Delete, Modifiers::SHIFT);
        assert_eq!(desk.text(), "", "{name} cuts");
        desk.clipboard = Some("back".into());
        desk.key(Named::Insert, Modifiers::SHIFT);
        desk.key(Named::Insert, Modifiers::SHIFT);
        assert_eq!(desk.text(), "backback", "{name} pastes");
    }
    Platform::set(None);
}

#[test]
fn the_right_click_menu_selects_copies_cuts_and_pastes() {
    for (name, view) in views("hello") {
        let mut desk = Desk::new(view);
        desk.right_click();
        // Nothing is selected yet: Cut and Copy do nothing and keep the menu open.
        desk.choose("Copy");
        desk.choose("Cut");
        assert_eq!(
            desk.clipboard, None,
            "{name}: Cut and Copy need a selection"
        );
        assert_eq!(desk.text(), "hello");
        desk.choose("Select All");
        assert!(desk.find("Select All").is_none(), "{name}: choosing closes");
        // A right press inside the selection keeps it.
        desk.right_click();
        desk.choose("Copy");
        assert_eq!(desk.clipboard.as_deref(), Some("hello"), "{name} copies");
        desk.right_click();
        desk.choose("Cut");
        assert_eq!(desk.text(), "", "{name} cuts");
        desk.clipboard = Some("menu".into());
        desk.right_click();
        desk.choose("Paste");
        assert_eq!(desk.text(), "menu", "{name} pastes");
    }
}

#[test]
fn a_right_press_outside_the_selection_moves_the_caret_there() {
    for (name, view) in views("hello world") {
        let mut desk = Desk::new(view);
        let bounds = desk.focus();
        desk.key(character("a"), Modifiers::COMMAND);
        for _ in 0.." world".len() {
            desk.key(Named::ArrowLeft, Modifiers::SHIFT);
        }
        // Past the end of the text, outside the selection of "hello".
        desk.click_at(
            Point::new(bounds.x + bounds.width - 20., bounds.y + 10.),
            Button::Right,
        );
        desk.choose("Copy");
        assert_eq!(desk.clipboard, None, "{name}: the selection is gone");
        desk.key(Named::Escape, Modifiers::empty());
        assert!(desk.find("Select All").is_none(), "{name}: Escape closes");
    }
}

#[test]
fn a_read_only_field_disables_cut_and_paste() {
    // A code block without `on_input` or a default value is a listing, not an editor.
    let views: [Element<'static, String, Theme>; 2] = [
        input("").value("fixed").into(),
        textarea("").value("fixed").into(),
    ];
    for (index, view) in views.into_iter().enumerate() {
        let mut desk = Desk::new(view);
        desk.right_click();
        desk.choose("Select All");
        desk.clipboard = Some("other".into());
        desk.right_click();
        desk.choose("Paste");
        desk.choose("Cut");
        assert!(
            desk.find("Select All").is_some(),
            "{index}: disabled items keep it open"
        );
        assert_eq!(desk.text(), "fixed", "{index}: read-only");
        desk.choose("Copy");
        assert_eq!(desk.clipboard.as_deref(), Some("fixed"), "{index}: copies");
    }
}

#[test]
fn shift_f10_opens_the_menu_at_the_caret_and_escape_closes_it() {
    for (name, view) in views("hello") {
        let mut desk = Desk::new(view);
        let bounds = desk.focus();
        desk.send([Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )]);
        desk.key(Named::F10, Modifiers::SHIFT);
        let item = desk
            .find("Undo")
            .or_else(|| desk.find("Cut"))
            .expect("menu");
        assert!(
            item.y > bounds.y && item.y < bounds.y + bounds.height + 40.,
            "{name}: the menu opens below the caret, at {item:?} for {bounds:?}"
        );
        assert!(item.x < bounds.x + 80., "{name}: at the caret, {item:?}");
        desk.key(Named::Escape, Modifiers::empty());
        assert!(desk.find("Select All").is_none(), "{name}: Escape closes");
        desk.key(Named::ContextMenu, Modifiers::empty());
        assert!(
            desk.find("Select All").is_some(),
            "{name}: the menu key opens it"
        );
    }
}

#[test]
fn a_right_press_on_another_field_moves_the_focus() {
    let view: Element<'static, String, Theme> = column![
        input("First").on_input(|s| format!("first {s}")),
        textarea("Second").on_input(|s| format!("second {s}")),
    ]
    .spacing(40)
    .into();
    let mut desk = Desk::new(view);
    let mut fields = Vec::new();
    {
        struct All<'a>(&'a mut Vec<Rectangle>);
        impl Operation for All<'_> {
            fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
                visit(self);
            }
            fn text_input(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn TextInput) {
                self.0.push(bounds);
            }
        }
        desk.ui.operate(&desk.renderer, &mut All(&mut fields));
    }
    let [first, second] = fields[..] else {
        panic!("two fields");
    };
    desk.click_at(Point::new(first.x + 10., first.center_y()), Button::Left);
    desk.click_at(Point::new(second.x + 10., second.y + 10.), Button::Right);
    desk.key(Named::Escape, Modifiers::empty());
    desk.send(simulator::typewrite("x"));
    assert_eq!(
        desk.messages,
        ["second x"],
        "only the right-clicked field types"
    );
}
