//! The keyboard shortcuts of the Input, the Textarea and the editable code
//! block on each platform, whichever the tests run on.
use std::sync::Arc;

use iced::keyboard::key::Named;
use iced::keyboard::{self, Key, Modifiers};
use iced::mouse::{self, Button};
use iced::{Element, Event, Point, Rectangle, Size, Vector};
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
use iced_test::simulator;
use icedcn::Theme;
use icedcn::widget::{Platform, code_block, container, input, scrollable, textarea};

const MAC: Platform = Platform::Mac;
const WINDOWS: Platform = Platform::Windows;
const LINUX: Platform = Platform::Linux;
const PLATFORMS: [Platform; 3] = [MAC, WINDOWS, LINUX];
const FIELDS: [&str; 3] = ["input", "textarea", "code block"];
const EDITORS: [&str; 2] = ["textarea", "code block"];

const NONE: Modifiers = Modifiers::empty();
const SHIFT: Modifiers = Modifiers::SHIFT;
const CTRL: Modifiers = Modifiers::CTRL;
const ALT: Modifiers = Modifiers::ALT;
const LOGO: Modifiers = Modifiers::LOGO;

/// A user interface with a clipboard of its own, following `platform`.
struct Desk {
    ui: UserInterface<'static, String, Theme, Renderer>,
    renderer: Renderer,
    platform: Platform,
    cursor: Point,
    clipboard: Option<String>,
    /// The primary selection the fields made.
    primary: Option<String>,
}

impl Desk {
    fn new(platform: Platform, view: impl Into<Element<'static, String, Theme>>) -> Self {
        Platform::set(Some(platform));
        let mut renderer =
            block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
        let ui = UserInterface::build(
            view.into(),
            Size::new(800., 600.),
            user_interface::Cache::default(),
            &mut renderer,
        );
        let mut desk = Self {
            ui,
            renderer,
            platform,
            cursor: Point::ORIGIN,
            clipboard: None,
            primary: None,
        };
        let (bounds, _) = desk.field();
        desk.click(Point::new(bounds.x + 14., bounds.y + 10.));
        desk
    }

    /// The command key of the platform.
    fn command(&self) -> Modifiers {
        if self.platform == MAC { LOGO } else { CTRL }
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
            events.clear();
            if let user_interface::State::Updated { clipboard, .. } = state {
                if let Some(clipboard::Content::Text(text)) = clipboard.write {
                    self.clipboard = Some(text);
                }
                if let Some(text) = clipboard.primary {
                    self.primary = Some(text);
                }
                for kind in clipboard.reads {
                    let source = if kind == clipboard::Kind::Primary {
                        &self.primary
                    } else {
                        &self.clipboard
                    };
                    events.push(Event::Clipboard(clipboard::Event::Read(
                        source
                            .clone()
                            .map(|text| Arc::new(clipboard::Content::Text(text)))
                            .ok_or(clipboard::Error::ContentNotAvailable),
                    )));
                }
            }
        }
    }

    fn middle_click(&mut self, position: Point) {
        self.cursor = position;
        self.send([
            Event::Mouse(mouse::Event::CursorMoved { position }),
            Event::Mouse(mouse::Event::ButtonPressed(Button::Middle)),
            Event::Mouse(mouse::Event::ButtonReleased(Button::Middle)),
        ]);
    }

    /// How far the first scrollable has scrolled down.
    fn scrolled(&mut self) -> f32 {
        #[derive(Default)]
        struct Offset(Option<f32>);
        impl Operation for Offset {
            fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
                visit(self);
            }
            fn scrollable(
                &mut self,
                _: Option<&Id>,
                _: Rectangle,
                _: Size,
                translation: Vector,
                _: &mut dyn operation::Scrollable,
            ) {
                self.0.get_or_insert(translation.y);
            }
        }
        let mut offset = Offset::default();
        self.ui.operate(&self.renderer, &mut offset);
        offset.0.expect("a scrollable")
    }

    fn click(&mut self, position: Point) {
        self.cursor = position;
        self.send([
            Event::Mouse(mouse::Event::CursorMoved { position }),
            Event::Mouse(mouse::Event::ButtonPressed(Button::Left)),
            Event::Mouse(mouse::Event::ButtonReleased(Button::Left)),
        ]);
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

    fn key(&mut self, key: impl Into<Key>, modifiers: Modifiers) {
        let key = key.into();
        self.send([
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)),
            with(simulator::press_key(key.clone(), None), modifiers),
            with(simulator::release_key(key), modifiers),
            Event::Keyboard(keyboard::Event::ModifiersChanged(NONE)),
        ]);
        // A frame lays the caret out, and shows it.
        self.send([Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )]);
    }

    fn letter(&mut self, c: &str, modifiers: Modifiers) {
        self.key(Key::Character(c.into()), modifiers);
    }

    fn type_text(&mut self, text: &str) {
        self.send(simulator::typewrite(text));
    }

    /// The text with `@` typed at the caret.
    fn caret(&mut self) -> String {
        self.type_text("@");
        self.text()
    }

    /// The selected text, through the platform's copy shortcut.
    fn selection(&mut self) -> Option<String> {
        self.clipboard = None;
        let command = self.command();
        self.letter("c", command);
        self.clipboard.take()
    }

    /// Put the caret at the end of the text.
    fn end(&mut self) {
        if self.platform == MAC {
            self.key(Named::ArrowDown, LOGO);
        } else {
            self.key(Named::End, CTRL);
        }
    }

    /// Put the caret at the start of the text.
    fn start(&mut self) {
        if self.platform == MAC {
            self.key(Named::ArrowUp, LOGO);
        } else {
            self.key(Named::Home, CTRL);
        }
    }
}

impl Drop for Desk {
    fn drop(&mut self) {
        Platform::set(None);
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

/// The input, the textarea or the code block, uncontrolled, holding `value`.
fn field(name: &str, value: &str) -> Element<'static, String, Theme> {
    let value = value.to_owned();
    match name {
        "input" => input("")
            .default_value(value)
            .width(400)
            .on_input(|s| s)
            .into(),
        "textarea" => container(textarea("").default_value(value).on_input(|s| s))
            .width(400)
            .into(),
        _ => container(code_block![
            code_block::content("").default_value(value).on_input(|s| s)
        ])
        .width(400)
        .into(),
    }
}

/// The modifier of word motions.
fn word(platform: Platform) -> Modifiers {
    if platform == MAC { ALT } else { CTRL }
}

#[test]
fn word_motions_follow_each_platform() {
    for platform in PLATFORMS {
        let expected = if platform == WINDOWS {
            ["foo   @bar baz", "foo   bar @baz"]
        } else {
            ["foo@   bar baz", "foo   bar@ baz"]
        };
        for name in FIELDS {
            for (count, expected) in [1, 2].into_iter().zip(expected) {
                let mut desk = Desk::new(platform, field(name, "foo   bar baz"));
                desk.start();
                for _ in 0..count {
                    desk.key(Named::ArrowRight, word(platform));
                }
                assert_eq!(desk.caret(), expected, "{platform:?} {name}, {count} words");
            }

            let mut desk = Desk::new(platform, field(name, "foo   bar baz"));
            desk.end();
            desk.key(Named::ArrowLeft, word(platform));
            desk.key(Named::ArrowLeft, word(platform));
            assert_eq!(desk.caret(), "foo   @bar baz", "{platform:?} {name} left");
        }
    }
}

#[test]
fn a_word_selection_follows_each_platform() {
    for platform in PLATFORMS {
        for name in FIELDS {
            let mut desk = Desk::new(platform, field(name, "foo bar"));
            desk.start();
            desk.key(Named::ArrowRight, word(platform) | SHIFT);
            let expected = if platform == WINDOWS { "foo " } else { "foo" };
            assert_eq!(
                desk.selection().as_deref(),
                Some(expected),
                "{platform:?} {name}"
            );
        }
    }
}

#[test]
fn line_ends_follow_each_platform() {
    for platform in PLATFORMS {
        for name in FIELDS {
            let mut desk = Desk::new(platform, field(name, "one two"));
            desk.start();
            if platform == MAC {
                desk.key(Named::ArrowRight, LOGO);
            } else {
                desk.key(Named::End, NONE);
            }
            assert_eq!(desk.caret(), "one two@", "{platform:?} {name} end");

            let mut desk = Desk::new(platform, field(name, "one two"));
            if platform == MAC {
                desk.key(Named::ArrowLeft, LOGO);
            } else {
                desk.key(Named::Home, NONE);
            }
            assert_eq!(desk.caret(), "@one two", "{platform:?} {name} start");
        }
    }
}

#[test]
fn home_and_end_leave_the_caret_on_a_mac() {
    for name in FIELDS {
        let mut desk = Desk::new(MAC, field(name, "one\ntwo"));
        desk.start();
        desk.key(Named::ArrowRight, NONE);
        desk.key(Named::End, NONE);
        desk.key(Named::PageDown, NONE);
        let text = desk.caret();
        assert!(text.starts_with("o@ne"), "{name}: {text:?}");

        let mut desk = Desk::new(MAC, field(name, "one two"));
        desk.start();
        desk.key(Named::ArrowRight, NONE);
        desk.key(Named::End, SHIFT);
        assert_eq!(desk.selection().as_deref(), Some("ne two"), "{name}");
    }
}

#[test]
fn the_mac_emacs_bindings_move_and_select() {
    for name in FIELDS {
        let mut desk = Desk::new(MAC, field(name, "abcd"));
        desk.end();
        desk.letter("a", CTRL);
        assert_eq!(desk.caret(), "@abcd", "{name} Control+A goes to the start");

        let mut desk = Desk::new(MAC, field(name, "abcd"));
        desk.start();
        desk.letter("f", CTRL);
        desk.letter("f", CTRL);
        desk.letter("b", CTRL);
        desk.letter("e", CTRL | SHIFT);
        assert_eq!(desk.selection().as_deref(), Some("bcd"), "{name}");
        desk.letter("h", CTRL);
        assert_eq!(desk.text(), "a", "{name} Control+H deletes the selection");
    }
}

#[test]
fn control_letters_run_no_commands_on_a_mac() {
    for name in FIELDS {
        let mut desk = Desk::new(MAC, field(name, "abc"));
        desk.end();
        desk.letter("z", CTRL);
        desk.letter("c", CTRL);
        assert_eq!(desk.selection(), None, "{name}");
        assert_eq!(desk.caret(), "abc@", "{name}");
    }
}

#[test]
fn the_mac_kill_buffer_cuts_to_the_end_and_yanks_it_back() {
    for name in FIELDS {
        let mut desk = Desk::new(MAC, field(name, "hello world"));
        desk.start();
        for _ in 0..5 {
            desk.key(Named::ArrowRight, NONE);
        }
        desk.letter("k", CTRL);
        assert_eq!(desk.text(), "hello", "{name}");
        desk.start();
        desk.letter("y", CTRL);
        assert_eq!(desk.caret(), " world@hello", "{name}");
    }

    for name in EDITORS {
        let mut desk = Desk::new(MAC, field(name, "ab\ncd"));
        desk.start();
        desk.letter("k", CTRL);
        desk.letter("k", CTRL);
        assert_eq!(desk.text(), "cd", "{name} kills the line, then its break");
        desk.key(Named::ArrowRight, LOGO);
        desk.letter("y", CTRL);
        assert_eq!(desk.text(), "cdab\n", "{name} yanks both kills");
    }
}

#[test]
fn control_t_transposes_on_a_mac() {
    for name in FIELDS {
        let mut desk = Desk::new(MAC, field(name, "abc"));
        desk.start();
        desk.key(Named::ArrowRight, NONE);
        desk.letter("t", CTRL);
        assert_eq!(desk.caret(), "ba@c", "{name}");

        let mut desk = Desk::new(MAC, field(name, "abc"));
        desk.end();
        desk.letter("t", CTRL);
        assert_eq!(desk.caret(), "acb@", "{name} at the end");
    }
}

#[test]
fn word_deletes_follow_each_platform() {
    for platform in PLATFORMS {
        for name in FIELDS {
            let mut desk = Desk::new(platform, field(name, "foo bar baz"));
            desk.start();
            desk.key(Named::ArrowRight, NONE);
            desk.key(Named::ArrowRight, NONE);
            desk.key(Named::Delete, word(platform));
            let expected = if platform == WINDOWS {
                "fobar baz"
            } else {
                "fo bar baz"
            };
            assert_eq!(desk.text(), expected, "{platform:?} {name}");

            desk.key(Named::Backspace, word(platform));
            assert_eq!(desk.text(), &expected[2..], "{platform:?} {name} back");
        }
    }
}

#[test]
fn shift_delete_cuts_only_on_a_pc() {
    for platform in PLATFORMS {
        for name in FIELDS {
            let mut desk = Desk::new(platform, field(name, "abc"));
            desk.start();
            desk.key(Named::ArrowRight, SHIFT);
            desk.key(Named::Delete, SHIFT);
            assert_eq!(desk.text(), "bc", "{platform:?} {name}");
            let cut = desk.clipboard.clone();
            if platform == MAC {
                assert_eq!(cut, None, "{name} deletes forward");
            } else {
                assert_eq!(cut.as_deref(), Some("a"), "{platform:?} {name} cuts");
            }
        }
    }
}

#[test]
fn alt_backspace_undoes_and_alt_shift_backspace_redoes_on_windows() {
    for name in FIELDS {
        let mut desk = Desk::new(WINDOWS, field(name, ""));
        desk.type_text("abc");
        desk.key(Named::Backspace, ALT);
        assert_eq!(desk.text(), "", "{name} undone");
        desk.key(Named::Backspace, ALT | SHIFT);
        assert_eq!(desk.text(), "abc", "{name} redone");
    }
}

#[test]
fn escape_leaves_the_field_focused() {
    for platform in PLATFORMS {
        for name in ["input", "textarea"] {
            let mut desk = Desk::new(platform, field(name, ""));
            desk.key(Named::Escape, NONE);
            desk.type_text("a");
            assert_eq!(desk.text(), "a", "{platform:?} {name}");
        }
    }
}

#[test]
fn the_code_block_indents_on_tab_and_lets_tab_leave_after_escape() {
    let mut desk = Desk::new(LINUX, field("code block", "x"));
    desk.start();
    desk.key(Named::Tab, NONE);
    assert_eq!(desk.text(), "    x");
    desk.key(Named::Escape, NONE);
    desk.key(Named::Tab, NONE);
    assert_eq!(
        desk.text(),
        "    x",
        "the Tab after Escape is left to the app"
    );
    desk.key(Named::Tab, CTRL);
    assert_eq!(desk.text(), "    x", "Control+Tab does not indent");
}

#[test]
fn up_and_down_past_the_ends_go_to_the_ends_on_a_mac_and_linux() {
    for platform in PLATFORMS {
        for name in EDITORS {
            let mut desk = Desk::new(platform, field(name, "abc\ndef"));
            desk.start();
            desk.key(Named::ArrowRight, NONE);
            desk.key(Named::ArrowUp, NONE);
            let expected = if platform == WINDOWS {
                "a@bc\ndef"
            } else {
                "@abc\ndef"
            };
            assert_eq!(desk.caret(), expected, "{platform:?} {name} up");

            let mut desk = Desk::new(platform, field(name, "abc\ndef"));
            desk.start();
            desk.key(Named::ArrowDown, NONE);
            desk.key(Named::ArrowRight, NONE);
            desk.key(Named::ArrowDown, NONE);
            let expected = if platform == WINDOWS {
                "abc\nd@ef"
            } else {
                "abc\ndef@"
            };
            assert_eq!(desk.caret(), expected, "{platform:?} {name} down");
        }
    }
}

#[test]
fn up_and_down_reach_the_ends_of_an_input_only_on_a_mac() {
    for platform in PLATFORMS {
        let mut desk = Desk::new(platform, field("input", "abc"));
        desk.start();
        desk.key(Named::ArrowRight, NONE);
        desk.key(Named::ArrowDown, NONE);
        let expected = if platform == MAC { "abc@" } else { "a@bc" };
        assert_eq!(desk.caret(), expected, "{platform:?}");
    }
}

#[test]
fn document_ends_follow_each_platform() {
    for platform in PLATFORMS {
        for name in EDITORS {
            let mut desk = Desk::new(platform, field(name, "one\ntwo\nthree"));
            desk.start();
            desk.key(Named::ArrowRight, NONE);
            if platform == MAC {
                desk.key(Named::ArrowDown, LOGO | SHIFT);
            } else {
                desk.key(Named::End, CTRL | SHIFT);
            }
            assert_eq!(
                desk.selection().as_deref(),
                Some("ne\ntwo\nthree"),
                "{platform:?} {name}"
            );
        }
    }
}

#[test]
fn paragraph_motions_follow_each_platform() {
    for platform in PLATFORMS {
        let (modifier, forward) = match platform {
            MAC => (ALT, "one two\nthree four@\nfive"),
            WINDOWS => (CTRL, "one two\nthree four\n@five"),
            LINUX => (CTRL, "one two\nthree four@\nfive"),
        };
        for name in EDITORS {
            let text = "one two\nthree four\nfive";
            let mut desk = Desk::new(platform, field(name, text));
            desk.start();
            desk.key(Named::ArrowDown, NONE);
            desk.key(Named::ArrowRight, NONE);
            desk.key(Named::ArrowDown, modifier);
            assert_eq!(desk.caret(), forward, "{platform:?} {name} forward");

            let mut desk = Desk::new(platform, field(name, text));
            desk.start();
            desk.key(Named::ArrowDown, NONE);
            desk.key(Named::ArrowRight, NONE);
            desk.key(Named::ArrowUp, modifier);
            assert_eq!(
                desk.caret(),
                "one two\n@three four\nfive",
                "{platform:?} {name} back"
            );
            desk.key(Named::Backspace, NONE);
            desk.key(Named::ArrowUp, modifier);
            assert_eq!(
                desk.caret(),
                "@one two\nthree four\nfive",
                "{platform:?} {name} back again"
            );
        }
    }
}

#[test]
fn up_and_down_leave_a_selection_from_its_edge() {
    for platform in PLATFORMS {
        for name in EDITORS {
            let mut desk = Desk::new(platform, field(name, "one\ntwo\nthree"));
            desk.start();
            desk.key(Named::ArrowDown, NONE);
            desk.key(Named::ArrowRight, SHIFT);
            desk.key(Named::ArrowDown, NONE);
            assert_eq!(desk.caret(), "one\ntwo\nt@hree", "{platform:?} {name}");
        }
    }
}

#[test]
fn home_and_end_stop_at_the_wraps_of_a_textarea_line() {
    let words = "alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu nu xi \
                 omicron pi rho sigma tau upsilon phi chi psi omega";
    for platform in [WINDOWS, LINUX, MAC] {
        let mut desk = Desk::new(platform, field("textarea", words));
        desk.start();
        if platform == MAC {
            desk.key(Named::ArrowRight, LOGO);
        } else {
            desk.key(Named::End, NONE);
        }
        let text = desk.caret();
        let at = text.find('@').expect("typed");
        assert!(
            0 < at && at < words.len(),
            "{platform:?}: the first row ends at {at}"
        );
    }
}

#[test]
fn linux_control_shift_backspace_deletes_to_the_line_start() {
    for name in EDITORS {
        let mut desk = Desk::new(LINUX, field(name, "one\ntwo three"));
        desk.end();
        desk.key(Named::Backspace, CTRL | SHIFT);
        assert_eq!(desk.text(), "one\n", "{name}");
    }
}

#[test]
fn the_menu_shortcuts_name_the_platforms_keys() {
    let mut desk = Desk::new(WINDOWS, field("input", "abc"));
    desk.key(Named::F10, SHIFT);
    let mut find = iced_test::Selector::find("Ctrl+C");
    desk.ui
        .operate(&desk.renderer, &mut operation::black_box(&mut find));
    assert!(
        matches!(find.finish(), operation::Outcome::Some(Some(_))),
        "Ctrl+C in the menu"
    );
}

#[test]
fn word_stops_skip_punctuation_but_on_windows() {
    let text = "say, \"hi\" now";
    for platform in PLATFORMS {
        let expected = if platform == WINDOWS {
            ["say@, \"hi\" now", "say, @\"hi\" now"]
        } else {
            ["say@, \"hi\" now", "say, \"hi@\" now"]
        };
        for name in FIELDS {
            for (count, expected) in [1, 2].into_iter().zip(expected) {
                let mut desk = Desk::new(platform, field(name, text));
                desk.start();
                for _ in 0..count {
                    desk.key(Named::ArrowRight, word(platform));
                }
                if platform == WINDOWS && count == 1 {
                    // Windows stops at the start of the comma.
                    assert_eq!(desk.caret(), "say@, \"hi\" now", "{name}");
                } else {
                    assert_eq!(desk.caret(), expected, "{platform:?} {name}, {count} words");
                }
            }

            let mut desk = Desk::new(platform, field(name, text));
            desk.end();
            desk.key(Named::ArrowLeft, word(platform));
            desk.key(Named::ArrowLeft, word(platform));
            let expected = if platform == WINDOWS {
                "say, \"hi@\" now"
            } else {
                "say, \"@hi\" now"
            };
            assert_eq!(desk.caret(), expected, "{platform:?} {name} left");
        }
    }
}

/// A textarea of sixty numbered lines in a scrollable 200 pixels tall.
fn tall(name: &str) -> Element<'static, String, Theme> {
    let lines: Vec<String> = (0..60).map(|line| format!("line {line}")).collect();
    let value = lines.join("\n");
    let field = match name {
        "textarea" => Element::from(textarea("").default_value(value).on_input(|s| s)),
        _ => code_block![code_block::content("").default_value(value).on_input(|s| s)].into(),
    };
    container(scrollable::scrollable(container(field).width(400)).height(200)).into()
}

/// The line the caret is on, by typing `@` there.
fn caret_line(desk: &mut Desk) -> usize {
    let text = desk.caret();
    text[..text.find('@').expect("typed")].matches('\n').count()
}

#[test]
fn page_down_moves_by_the_visible_part_of_a_tall_editor() {
    for platform in [WINDOWS, LINUX] {
        for name in EDITORS {
            let mut desk = Desk::new(platform, tall(name));
            desk.start();
            desk.key(Named::PageDown, NONE);
            let line = caret_line(&mut desk);
            assert!((4..=12).contains(&line), "{platform:?} {name}: line {line}");
            assert!(
                desk.scrolled() > 0.0,
                "{platform:?} {name} scrolls to the caret"
            );
        }
    }
}

#[test]
fn a_caret_moved_past_the_view_scrolls_the_page() {
    for name in EDITORS {
        let mut desk = Desk::new(LINUX, tall(name));
        desk.key(Named::End, CTRL);
        let bottom = desk.scrolled();
        assert!(bottom > 600.0, "{name} shows the end: {bottom}");
        desk.key(Named::Home, CTRL);
        assert_eq!(desk.scrolled(), 0.0, "{name} shows the start");
    }
}

#[test]
fn control_l_centers_the_caret_and_page_keys_scroll_on_a_mac() {
    for name in EDITORS {
        let mut desk = Desk::new(MAC, tall(name));
        for _ in 0..20 {
            desk.key(Named::ArrowDown, NONE);
        }
        let before = desk.scrolled();
        desk.letter("l", CTRL);
        let after = desk.scrolled();
        assert!(after > before + 40.0, "{name}: {before} -> {after}");

        desk.key(Named::Home, NONE);
        assert_eq!(desk.scrolled(), 0.0, "{name} Home shows the top");
        desk.key(Named::PageDown, NONE);
        let page = desk.scrolled();
        assert!((100.0..=200.0).contains(&page), "{name} pages by {page}");
        desk.key(Named::End, NONE);
        assert!(desk.scrolled() > 600.0, "{name} End shows the bottom");
        assert_eq!(caret_line(&mut desk), 20, "{name}: the caret stays");
    }
}

#[test]
fn insert_types_over_the_text_on_a_pc() {
    for platform in PLATFORMS {
        for name in FIELDS {
            let mut desk = Desk::new(platform, field(name, "abc"));
            desk.start();
            desk.key(Named::Insert, NONE);
            desk.type_text("XYZW");
            let expected = if platform == MAC { "XYZWabc" } else { "XYZW" };
            assert_eq!(desk.text(), expected, "{platform:?} {name}");
            desk.key(Named::Insert, NONE);
            desk.start();
            desk.type_text("-");
            assert_eq!(
                desk.text(),
                format!("-{expected}"),
                "{platform:?} {name} inserts again"
            );
        }
    }
}

#[test]
fn a_selection_is_the_primary_selection_a_middle_click_pastes_on_linux() {
    for platform in PLATFORMS {
        for name in FIELDS {
            let mut desk = Desk::new(platform, field(name, "abc"));
            desk.start();
            desk.key(Named::ArrowRight, SHIFT);
            desk.key(Named::ArrowRight, SHIFT);
            if platform != LINUX {
                assert_eq!(desk.primary, None, "{platform:?} {name}");
                continue;
            }
            assert_eq!(desk.primary.as_deref(), Some("ab"), "{name}");
            let (bounds, _) = desk.field();
            desk.middle_click(Point::new(bounds.x + bounds.width - 20., bounds.y + 10.));
            assert_eq!(desk.text(), "abcab", "{name} pastes it");
        }
    }
}
