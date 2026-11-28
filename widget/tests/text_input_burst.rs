//! A controlled input keeps every keystroke of a burst.
//!
//! The runtime hands a window all the events that arrived since the last
//! frame in one `UserInterface::update` call, and only then runs the
//! application's `update` and `view`. Each of these tests drives that same
//! cycle: one batch of events against one widget tree, then every published
//! message through the application, then a fresh `view` from the new state.
//!
//! A [`Component`] (a dialog, a popover, a select...) rebuilds its view as
//! soon as something inside it publishes, from the values the application
//! gave it when it was built: the input inside is a new widget, holding the
//! value from before the burst, while the rest of the burst arrives.
use iced_core::clipboard;
use iced_core::edit;
use iced_core::event;
use iced_core::input_method;
use iced_core::keyboard::{self, key};
use iced_core::mouse;
use iced_core::renderer;
use iced_core::shell;
use iced_core::widget::{self, operation};
use iced_core::window;
use iced_core::{Clipboard, Element, Event, Rectangle, Size};
use iced_runtime::UserInterface;
use iced_runtime::user_interface;
use iced_test::simulator;
use iced_widget::{Component, Renderer, Widget, component};
use icedcn_core::Theme;
use icedcn_widget::{View, dialog, input, input_group, space};

use std::sync::Arc;

const INPUT: widget::Id = widget::Id::new("input");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Message {
    Input(String),
    Open(bool),
}

/// Where the input sits in the view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Host {
    /// Directly in the view.
    Plain,
    /// Inside a [`Component`], which rebuilds its view from the value the
    /// application gave it as soon as the input publishes, before the
    /// application has seen the message.
    Component,
    /// In an open [`dialog`], inside an input group, as the command palette
    /// and the connection forms of an application hold it.
    Dialog,
}

fn field(value: &str) -> View<'_, Message> {
    input("")
        .id(INPUT)
        .value(value)
        .on_input(Message::Input)
        .into()
}

struct Wrapper<'a> {
    value: &'a str,
}

impl<'a> Component<'a, Message, Theme, Renderer> for Wrapper<'a> {
    type State = ();
    type Event = Message;

    fn update(&self, _state: &mut (), event: Message, _renderer: &Renderer) -> Option<Message> {
        Some(event)
    }

    fn view(&self, _state: &()) -> impl Widget<Message, Theme, Renderer> + 'a {
        field(self.value).compose()
    }
}

fn view(host: Host, value: &str) -> Element<'_, Message, Theme, Renderer> {
    match host {
        Host::Plain => field(value).compose(),
        Host::Component => component(Wrapper { value }).boxed(),
        Host::Dialog => dialog(
            |_| space().width(10).height(10).into(),
            move |_| input_group(input("").id(INPUT).value(value).on_input(Message::Input)).into(),
        )
        .open(true)
        .on_open_change(Message::Open)
        .into(),
    }
}

/// A tiny application runtime: the value it owns, how it accepts an edit,
/// and the widget cache kept between frames.
struct App {
    host: Host,
    value: String,
    accept: fn(&str, String) -> String,
    renderer: Renderer,
    cache: user_interface::Cache,
    /// Every message the application received, in order.
    received: Vec<String>,
    /// What the last burst asked of the clipboard; nothing when the burst
    /// left the widget tree outdated.
    clipboard: Option<Clipboard>,
}

impl App {
    fn new(host: Host, value: &str) -> Self {
        Self::with(host, value, |_, new| new)
    }

    fn with(host: Host, value: &str, accept: fn(&str, String) -> String) -> Self {
        let renderer = iced_runtime::futures::futures::executor::block_on(
            <Renderer as renderer::Headless>::new(renderer::Settings::default(), None),
        )
        .expect("create headless renderer");

        let mut app = Self {
            host,
            value: value.to_owned(),
            accept,
            renderer,
            cache: user_interface::Cache::default(),
            received: Vec::new(),
            clipboard: None,
        };

        app.frame(|ui, renderer| {
            ui.operate(renderer, &mut operation::focusable::focus(INPUT));
            ui.operate(
                renderer,
                &mut operation::text_input::move_cursor_to_end(INPUT),
            );
        });

        app
    }

    /// Builds the view, runs `f` against it, keeps its cache.
    fn frame(
        &mut self,
        f: impl FnOnce(&mut UserInterface<'_, Message, Theme, Renderer>, &mut Renderer),
    ) {
        let cache = std::mem::take(&mut self.cache);
        let mut ui = UserInterface::build(
            view(self.host, &self.value),
            Size::new(400.0, 300.0),
            cache,
            &mut self.renderer,
        );

        f(&mut ui, &mut self.renderer);

        self.cache = ui.into_cache();
    }

    /// One frame of the runtime: every event against one widget tree, then
    /// every message through `update`, as `iced_winit` does.
    fn burst(&mut self, events: impl IntoIterator<Item = Event>) -> &str {
        let events: Vec<Event> = events.into_iter().collect();
        let mut messages = shell::Bus::new();
        let mut clipboard = None;

        self.frame(|ui, renderer| {
            let (state, _statuses) = ui.update(
                &window::Headless,
                &shell::Waker::noop(),
                &events,
                mouse::Cursor::Unavailable,
                renderer,
                &mut messages,
            );
            if let user_interface::State::Updated {
                clipboard: requests,
                ..
            } = state
            {
                clipboard = Some(requests);
            }
        });
        self.clipboard = clipboard;

        for (message, _receipt) in messages.drain() {
            let Message::Input(value) = message else {
                continue;
            };
            self.received.push(value.clone());
            self.value = (self.accept)(&self.value, value);
        }

        // The next frame: the view reflects the new state.
        self.frame(|_, _| {});

        &self.value
    }

    /// What the input shows.
    fn shown(&mut self) -> String {
        let mut shown = Shown(None);

        self.frame(|ui, renderer| ui.operate(renderer, &mut shown));

        shown.0.expect("the input is in the view")
    }
}

/// Reads what the input shows.
struct Shown(Option<String>);

impl widget::Operation for Shown {
    fn text_input(
        &mut self,
        id: Option<&widget::Id>,
        _bounds: Rectangle,
        state: &mut dyn operation::TextInput,
    ) {
        if id == Some(&INPUT) {
            self.0 = Some(state.text().into_owned());
        }
    }

    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn widget::Operation)) {
        operate(self);
    }
}

const HOSTS: [Host; 3] = [Host::Plain, Host::Component, Host::Dialog];

fn key(named: key::Named) -> Event {
    simulator::press_key(keyboard::Key::Named(named), None)
}

/// Command-V, then the clipboard's reply.
fn paste(text: &str) -> [Event; 2] {
    let v = keyboard::Key::Character("v".into());

    [
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: v.clone(),
            modified_key: v,
            physical_key: keyboard::key::Physical::Code(keyboard::key::Code::KeyV),
            location: keyboard::Location::Standard,
            modifiers: keyboard::Modifiers::COMMAND,
            repeat: false,
            text: None,
        }),
        Event::Clipboard(clipboard::Event::Read(Ok(Arc::new(
            clipboard::Content::Text(text.to_owned()),
        )))),
    ]
}

/// What `f` gives for every host, to compare with one expected answer.
fn each<T>(f: impl Fn(Host) -> T) -> Vec<(Host, T)> {
    HOSTS.into_iter().map(|host| (host, f(host))).collect()
}

fn all<T: Clone>(expected: T) -> Vec<(Host, T)> {
    each(|_| expected.clone())
}

#[test]
fn a_burst_of_keystrokes_reaches_the_application() {
    let outcome = each(|host| {
        let mut app = App::new(host, "a");
        let value = app.burst(simulator::typewrite("bcd")).to_owned();

        (value, app.received.clone(), app.shown())
    });

    assert_eq!(
        outcome,
        all((
            String::from("abcd"),
            vec![
                String::from("ab"),
                String::from("abc"),
                String::from("abcd")
            ],
            String::from("abcd"),
        ))
    );
}

#[test]
fn native_undo_and_redo_use_the_focused_inputs_history() {
    let outcome = each(|host| {
        let mut app = App::new(host, "a");
        assert_eq!(app.burst(simulator::typewrite("b")), "ab");
        assert_eq!(app.burst([Event::Edit(edit::Command::Undo)]), "a");
        assert_eq!(app.shown(), "a");
        assert_eq!(app.burst([Event::Edit(edit::Command::Redo)]), "ab");
        app.shown()
    });

    assert_eq!(outcome, all(String::from("ab")));
}

#[test]
fn native_history_commands_propagate_unless_the_input_is_active_and_editable() {
    let commands = [
        Event::Edit(edit::Command::Undo),
        Event::Edit(edit::Command::Redo),
    ];
    let mut ui = simulator::simulator(field("a").compose());
    assert_eq!(ui.simulate(commands.clone()), [event::Status::Ignored; 2]);

    ui.operate(&mut operation::focusable::focus(INPUT));
    // An active editor owns these commands even when its history is empty.
    assert_eq!(ui.simulate(commands.clone()), [event::Status::Captured; 2]);

    let _ = ui.simulate([Event::Window(window::Event::Unfocused)]);
    assert_eq!(ui.simulate(commands.clone()), [event::Status::Ignored; 2]);

    let _ = ui.simulate([Event::Window(window::Event::Focused)]);
    let disabled: Element<'_, Message, Theme, Renderer> = input("")
        .id(INPUT)
        .value("a")
        .on_input(Message::Input)
        .disabled(true)
        .into();
    let mut ui = ui.rebuild(disabled);
    assert_eq!(ui.simulate(commands), [event::Status::Ignored; 2]);
}

/// The text the last burst wrote to the clipboard.
fn written(app: &App) -> Option<String> {
    match app.clipboard.as_ref()?.write.as_ref()? {
        clipboard::Content::Text(text) => Some(text.clone()),
        _ => None,
    }
}

/// The clipboard reads the last burst asked for.
fn reads(app: &App) -> Vec<clipboard::Kind> {
    app.clipboard
        .as_ref()
        .map(|clipboard| clipboard.reads.clone())
        .unwrap_or_default()
}

fn reply(text: &str) -> Event {
    Event::Clipboard(clipboard::Event::Read(Ok(Arc::new(
        clipboard::Content::Text(text.to_owned()),
    ))))
}

#[test]
fn native_clipboard_commands_edit_the_focused_input() {
    let outcome = each(|host| {
        let mut app = App::new(host, "abc");
        assert_eq!(app.burst([Event::Edit(edit::Command::SelectAll)]), "abc");
        assert_eq!(app.burst([Event::Edit(edit::Command::Copy)]), "abc");
        assert_eq!(written(&app).as_deref(), Some("abc"));
        assert_eq!(app.burst([Event::Edit(edit::Command::Cut)]), "");
        assert_eq!(app.shown(), "");
        assert_eq!(app.burst([Event::Edit(edit::Command::Paste)]), "");
        assert_eq!(reads(&app), [clipboard::Kind::Text]);
        assert_eq!(app.burst([reply("xyz")]), "xyz");
        app.shown()
    });

    assert_eq!(outcome, all(String::from("xyz")));
}

#[test]
fn native_cut_writes_the_selection_it_removes() {
    let mut app = App::new(Host::Plain, "abc");
    let _ = app.burst([Event::Edit(edit::Command::SelectAll)]);
    assert_eq!(app.burst([Event::Edit(edit::Command::Cut)]), "");
    assert_eq!(written(&app).as_deref(), Some("abc"));
}

#[test]
fn native_copy_and_cut_leave_a_secure_input_and_the_clipboard_alone() {
    let secret: Element<'_, Message, Theme, Renderer> = input("")
        .id(INPUT)
        .value("secret")
        .secure(true)
        .on_input(Message::Input)
        .into();
    let mut renderer = iced_runtime::futures::futures::executor::block_on(
        <Renderer as renderer::Headless>::new(renderer::Settings::default(), None),
    )
    .expect("create headless renderer");
    let mut ui = UserInterface::build(
        secret,
        Size::new(400.0, 300.0),
        user_interface::Cache::default(),
        &mut renderer,
    );
    ui.operate(&renderer, &mut operation::focusable::focus(INPUT));
    let mut messages = shell::Bus::new();

    let (state, statuses) = ui.update(
        &window::Headless,
        &shell::Waker::noop(),
        &[
            Event::Edit(edit::Command::SelectAll),
            Event::Edit(edit::Command::Copy),
            Event::Edit(edit::Command::Cut),
        ],
        mouse::Cursor::Unavailable,
        &mut renderer,
        &mut messages,
    );

    assert_eq!(statuses, [event::Status::Captured; 3]);
    let user_interface::State::Updated { clipboard, .. } = state else {
        panic!("a secure input publishes nothing, so the tree stays current");
    };
    assert!(
        clipboard.write.is_none(),
        "a secret never reaches the clipboard"
    );
    assert_eq!(messages.drain().count(), 0, "a secure input cuts nothing");
}

#[test]
fn native_clipboard_commands_propagate_unless_the_input_is_active() {
    let commands = [
        Event::Edit(edit::Command::Copy),
        Event::Edit(edit::Command::Cut),
        Event::Edit(edit::Command::Paste),
        Event::Edit(edit::Command::SelectAll),
    ];
    let mut ui = simulator::simulator(field("a").compose());
    assert_eq!(ui.simulate(commands.clone()), [event::Status::Ignored; 4]);

    ui.operate(&mut operation::focusable::focus(INPUT));
    assert_eq!(ui.simulate(commands.clone()), [event::Status::Captured; 4]);

    let _ = ui.simulate([Event::Window(window::Event::Unfocused)]);
    assert_eq!(ui.simulate(commands.clone()), [event::Status::Ignored; 4]);

    let _ = ui.simulate([Event::Window(window::Event::Focused)]);
    // A read-only input copies and selects, but cuts and pastes nothing.
    let read_only: Element<'_, Message, Theme, Renderer> = input("").id(INPUT).value("a").into();
    let mut ui = ui.rebuild(read_only);
    assert_eq!(
        ui.simulate(commands.clone()),
        [
            event::Status::Captured,
            event::Status::Ignored,
            event::Status::Ignored,
            event::Status::Captured,
        ]
    );

    let disabled: Element<'_, Message, Theme, Renderer> = input("")
        .id(INPUT)
        .value("a")
        .on_input(Message::Input)
        .disabled(true)
        .into();
    let mut ui = ui.rebuild(disabled);
    assert_eq!(ui.simulate(commands), [event::Status::Ignored; 4]);
}

#[test]
fn a_burst_with_backspace_and_cursor_moves_edits_its_own_result() {
    let outcome = each(|host| {
        let mut app = App::new(host, "abc");

        let events = [key(key::Named::Backspace), key(key::Named::Backspace)]
            .into_iter()
            .chain(simulator::typewrite("xy"))
            .chain([key(key::Named::ArrowLeft)])
            .chain(simulator::typewrite("z"));

        app.burst(events).to_owned()
    });

    assert_eq!(outcome, all(String::from("axzy")));
}

#[test]
fn a_burst_with_a_paste_keeps_what_came_before_and_after() {
    let outcome = each(|host| {
        let mut app = App::new(host, "a");

        let events = simulator::typewrite("b")
            .chain(paste("XY"))
            .chain(simulator::typewrite("c"));

        app.burst(events).to_owned()
    });

    assert_eq!(outcome, all(String::from("abXYc")));
}

#[test]
fn a_burst_with_an_input_method_commit_keeps_it() {
    let outcome = each(|host| {
        let mut app = App::new(host, "a");

        let events = simulator::typewrite("b")
            .chain([Event::InputMethod(input_method::Event::Commit(
                String::from("é"),
            ))])
            .chain(simulator::typewrite("c"));

        app.burst(events).to_owned()
    });

    assert_eq!(outcome, all(String::from("abéc")));
}

#[test]
fn a_burst_replaces_a_selection_once() {
    let outcome = each(|host| {
        let mut app = App::new(host, "old");

        app.frame(|ui, renderer| {
            ui.operate(renderer, &mut operation::text_input::select_all(INPUT));
        });

        app.burst(simulator::typewrite("new")).to_owned()
    });

    assert_eq!(outcome, all(String::from("new")));
}

#[test]
fn an_application_that_rewrites_the_value_wins_after_the_burst() {
    let outcome = each(|host| {
        let mut app = App::with(host, "a", |_, value| value.to_uppercase());

        let first = app.burst(simulator::typewrite("bc")).to_owned();
        let shown = app.shown();
        // The next burst edits the rewritten value.
        let second = app.burst(simulator::typewrite("d")).to_owned();

        (first, shown, second)
    });

    assert_eq!(
        outcome,
        all((
            String::from("ABC"),
            String::from("ABC"),
            String::from("ABCD")
        ))
    );
}

#[test]
fn an_application_that_limits_the_value_wins_after_the_burst() {
    let outcome = each(|host| {
        let mut app = App::with(host, "a", |_, value| value.chars().take(3).collect());

        let first = app.burst(simulator::typewrite("bcde")).to_owned();
        let shown = app.shown();
        let second = app.burst([key(key::Named::Backspace)]).to_owned();

        (first, shown, second)
    });

    assert_eq!(
        outcome,
        all((String::from("abc"), String::from("abc"), String::from("ab")))
    );
}

#[test]
fn an_application_that_rejects_the_edit_restores_its_value() {
    let outcome = each(|host| {
        let mut app = App::with(host, "a", |old, _| old.to_owned());

        let first = app.burst(simulator::typewrite("bc")).to_owned();
        let shown = app.shown();
        let second = app.burst(simulator::typewrite("z")).to_owned();

        (first, shown, second)
    });

    assert_eq!(
        outcome,
        all((String::from("a"), String::from("a"), String::from("a")))
    );
}
