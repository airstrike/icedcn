//! Pointer and keyboard editing of the Input, the Textarea and the editable
//! code block: hit-testing while scrolled, autoscroll, undo, word motion and
//! clipping, driven through a user interface whose clipboard the test owns.
use std::sync::Arc;

use iced::keyboard::key::Named;
use iced::keyboard::{self, Key, Modifiers};
use iced::mouse::{self, Button};
use iced::{Element, Event, Point, Rectangle, Size};
use iced_test::core::clipboard;
use iced_test::core::renderer::{self, Headless};
use iced_test::core::shell::{Bus, Waker};
use iced_test::core::time::{Duration, Instant};
use iced_test::core::widget::operation::{self, TextInput};
use iced_test::core::widget::{Id, Operation};
use iced_test::core::window;
use iced_test::futures::futures::executor::block_on;
use iced_test::renderer::Renderer;
use iced_test::runtime::user_interface::{self, UserInterface};
use iced_test::simulator;
use icedcn::Theme;
use icedcn::widget::{button, code_block, column, container, input, textarea};

/// A user interface with a clipboard of its own.
struct Desk {
    ui: UserInterface<'static, String, Theme, Renderer>,
    renderer: Renderer,
    cursor: Point,
    modifiers: Modifiers,
    clipboard: Option<String>,
    messages: Vec<String>,
}

impl Desk {
    fn new(view: impl Into<Element<'static, String, Theme>>) -> Self {
        Self::with(view, None)
    }

    /// A desk drawn by the given `backend`.
    fn with(view: impl Into<Element<'static, String, Theme>>, backend: Option<&str>) -> Self {
        let mut renderer =
            block_on(Renderer::new(Default::default(), backend)).expect("headless renderer");
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
            modifiers: Modifiers::empty(),
            clipboard: None,
            messages: Vec::new(),
        }
    }

    /// Build `view` anew over the same state, as the app does after it
    /// handles a message.
    fn rebuild(&mut self, view: impl Into<Element<'static, String, Theme>>) {
        let ui = std::mem::replace(
            &mut self.ui,
            UserInterface::build(
                Element::from(input::<String>("")),
                Size::new(800., 600.),
                user_interface::Cache::default(),
                &mut self.renderer,
            ),
        );
        self.ui = UserInterface::build(
            view.into(),
            Size::new(800., 600.),
            ui.into_cache(),
            &mut self.renderer,
        );
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

    /// The frame as gray levels, one row per pixel row, at 1×.
    fn pixels(&mut self) -> Vec<Vec<u8>> {
        let theme = Theme::light();
        self.redraw();
        self.ui.draw(
            &mut self.renderer,
            &theme,
            &renderer::Style::default(),
            mouse::Cursor::Available(self.cursor),
        );
        let rgba = self
            .renderer
            .screenshot(Size::new(800, 600), 1., theme.neutral.base.into());
        rgba.chunks(800 * 4)
            .map(|row| row.chunks(4).map(|px| px[1]).collect())
            .collect()
    }

    fn redraw(&mut self) {
        self.send([Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )]);
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

    fn modifiers(&mut self, modifiers: Modifiers) {
        self.modifiers = modifiers;
        self.send([Event::Keyboard(keyboard::Event::ModifiersChanged(
            modifiers,
        ))]);
    }

    fn move_to(&mut self, position: Point) {
        self.cursor = position;
        self.send([Event::Mouse(mouse::Event::CursorMoved { position })]);
    }

    fn press(&mut self, position: Point) {
        self.move_to(position);
        self.send([Event::Mouse(mouse::Event::ButtonPressed(Button::Left))]);
    }

    fn release(&mut self) {
        self.send([Event::Mouse(mouse::Event::ButtonReleased(Button::Left))]);
    }

    fn click_with(&mut self, position: Point, button: Button) {
        self.move_to(position);
        self.send([
            Event::Mouse(mouse::Event::ButtonPressed(button)),
            Event::Mouse(mouse::Event::ButtonReleased(button)),
        ]);
    }

    /// Click the menu entry labeled `label`.
    fn choose(&mut self, label: &'static str) {
        let mut find = iced_test::Selector::find(label);
        self.ui
            .operate(&self.renderer, &mut operation::black_box(&mut find));
        let bounds = match find.finish() {
            operation::Outcome::Some(Some(target)) => target.visible_bounds(),
            _ => None,
        }
        .unwrap_or_else(|| panic!("{label} in the menu"));
        self.click(bounds.center());
    }

    /// Focus the field with a click near its start.
    fn focus(&mut self) {
        let (bounds, _) = self.field();
        self.click(Point::new(bounds.x + 14., bounds.y + 10.));
    }

    fn click(&mut self, position: Point) {
        self.press(position);
        self.release();
    }

    fn key(&mut self, key: impl Into<Key>, modifiers: Modifiers) {
        let key = key.into();
        let held = self.modifiers;
        self.modifiers(modifiers);
        self.send(
            [
                simulator::press_key(key.clone(), None),
                simulator::release_key(key),
            ]
            .map(|event| with(event, modifiers)),
        );
        self.modifiers(held);
    }

    /// The selected text, through Command+C.
    fn selection(&mut self) -> Option<String> {
        self.clipboard = None;
        self.key(character("c"), Modifiers::COMMAND);
        self.clipboard.take()
    }

    /// The selection a drag made so far, copied without ending the drag.
    fn dragged(&mut self) -> String {
        self.selection().unwrap_or_default()
    }

    /// The caret's index: the length of the text before it, by selecting to
    /// the start.
    fn caret(&mut self) -> usize {
        self.key(Named::Home, Modifiers::SHIFT);
        let before = self.selection().map_or(0, |text| text.chars().count());
        self.key(Named::ArrowRight, Modifiers::empty());
        before
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
            text: if modifiers.command() { None } else { text },
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

/// Twenty-six distinct letters, four times over: any run of the text tells
/// where it came from.
const LONG: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// An uncontrolled input 200 pixels wide holding `value`.
fn narrow(value: &'static str) -> Element<'static, String, Theme> {
    input("")
        .default_value(value)
        .width(200)
        .on_input(|s| s)
        .into()
}

/// A redraw at `at` milliseconds past `start`.
fn frame(start: Instant, at: u64) -> Event {
    Event::Window(window::Event::RedrawRequested(
        start + Duration::from_millis(at),
    ))
}

#[test]
fn a_press_on_a_scrolled_input_keeps_the_text_under_the_pointer() {
    let mut desk = Desk::new(narrow(LONG));
    let (bounds, _) = desk.field();
    desk.click(Point::new(bounds.x + 20., bounds.center_y()));
    desk.key(Named::End, Modifiers::empty());
    for x in [
        bounds.x + 30.,
        bounds.x + 120.,
        bounds.x + bounds.width - 14.,
    ] {
        desk.key(Named::End, Modifiers::empty());
        // Nudging the pointer while it is held selects nothing: the press
        // did not scroll the text away from it.
        desk.press(Point::new(x, bounds.center_y()));
        desk.move_to(Point::new(x + 0.5, bounds.center_y()));
        desk.release();
        assert_eq!(desk.selection(), None, "at {x}");
        assert!(desk.caret() > 40, "{x} hits the scrolled text");
    }
}

#[test]
fn a_press_in_the_padding_of_a_scrolled_input_hits_the_first_visible_character() {
    let mut desk = Desk::new(narrow(LONG));
    let (bounds, _) = desk.field();
    desk.click(Point::new(bounds.x + 20., bounds.center_y()));
    desk.key(Named::End, Modifiers::empty());
    desk.click(Point::new(bounds.x + 13., bounds.center_y()));
    let first = desk.caret();
    desk.key(Named::End, Modifiers::empty());
    desk.click(Point::new(bounds.x + 2., bounds.center_y()));
    assert_eq!(desk.caret(), first, "the padding press keeps the scroll");
    assert!(first > 40);
}

#[test]
fn a_press_on_a_blurred_input_hits_the_text_drawn_from_its_start() {
    let view = column![
        input("").default_value(LONG).width(200).on_input(|s| s),
        button("Elsewhere").height(40),
    ]
    .spacing(20);
    let mut desk = Desk::new(view);
    let fresh = {
        let mut desk = Desk::new(narrow(LONG));
        let (bounds, _) = desk.field();
        desk.click(Point::new(bounds.x + 100., bounds.center_y()));
        desk.caret()
    };
    let (bounds, _) = desk.field();
    desk.click(Point::new(bounds.x + 20., bounds.center_y()));
    desk.key(Named::End, Modifiers::empty());
    desk.click(Point::new(bounds.x + 20., bounds.y + bounds.height + 40.));
    desk.redraw();
    desk.click(Point::new(bounds.x + 100., bounds.center_y()));
    assert_eq!(desk.caret(), fresh);
}

#[test]
fn dragging_past_the_left_edge_scrolls_and_selects_to_the_start() {
    let mut desk = Desk::new(narrow(LONG));
    let (bounds, _) = desk.field();
    desk.click(Point::new(bounds.x + 20., bounds.center_y()));
    desk.key(Named::End, Modifiers::empty());
    desk.press(Point::new(bounds.x + 100., bounds.center_y()));
    desk.move_to(Point::new(bounds.x - 40., bounds.center_y()));
    // The pointer stays put: each frame scrolls further on its own.
    let start = Instant::now();
    let mut scrolled = Vec::new();
    for at in (0..=40).map(|n| n * 100) {
        desk.send([frame(start, at)]);
        scrolled.push(desk.dragged());
    }
    desk.release();
    assert!(
        scrolled.windows(2).take(3).all(|w| w[1].len() > w[0].len()),
        "every frame extends the selection: {scrolled:?}"
    );
    let selected = desk.selection().expect("a selection");
    assert!(
        LONG.starts_with(&selected) && selected.len() > 60,
        "{selected:?}"
    );
}

#[test]
fn dragging_past_the_right_edge_scrolls_and_selects_to_the_end() {
    let mut desk = Desk::new(narrow(LONG));
    let (bounds, _) = desk.field();
    desk.press(Point::new(bounds.x + 100., bounds.center_y()));
    desk.move_to(Point::new(bounds.x + bounds.width + 60., bounds.center_y()));
    let start = Instant::now();
    for at in (0..=40).map(|n| n * 100) {
        desk.send([frame(start, at)]);
    }
    desk.release();
    let selected = desk.selection().expect("a selection");
    assert!(
        LONG.ends_with(&selected) && selected.len() > 80,
        "{selected:?}"
    );
}

#[test]
fn a_double_click_drag_past_the_edge_selects_whole_words() {
    let words = "alpha bravo charlie delta echo foxtrot golf hotel india juliet kilo lima mike";
    let mut desk = Desk::new(narrow(words));
    let (bounds, _) = desk.field();
    desk.click(Point::new(bounds.x + 20., bounds.center_y()));
    desk.key(Named::End, Modifiers::empty());
    let at = Point::new(bounds.x + bounds.width - 30., bounds.center_y());
    desk.click(at);
    desk.press(at);
    desk.move_to(Point::new(bounds.x - 30., bounds.center_y()));
    let start = Instant::now();
    for at in (0..=40).map(|n| n * 100) {
        desk.send([frame(start, at)]);
    }
    desk.release();
    let selected = desk.selection().expect("a selection");
    assert!(
        words.starts_with(&selected),
        "{selected:?} runs from the start"
    );
    assert!(
        words[selected.len()..].starts_with(' ') || selected.len() == words.len(),
        "{selected:?} ends on a word"
    );
}

#[test]
fn a_triple_click_on_a_scrolled_input_selects_everything() {
    let mut desk = Desk::new(narrow(LONG));
    let (bounds, _) = desk.field();
    desk.click(Point::new(bounds.x + 20., bounds.center_y()));
    desk.key(Named::End, Modifiers::empty());
    let at = Point::new(bounds.x + 90., bounds.center_y());
    desk.click(at);
    desk.click(at);
    desk.click(at);
    assert_eq!(desk.selection().as_deref(), Some(LONG));
}

#[test]
fn a_shift_press_on_a_scrolled_input_extends_from_the_caret() {
    let mut desk = Desk::new(narrow(LONG));
    let (bounds, _) = desk.field();
    desk.click(Point::new(bounds.x + 20., bounds.center_y()));
    desk.key(Named::End, Modifiers::empty());
    desk.click(Point::new(bounds.x + 60., bounds.center_y()));
    desk.modifiers(Modifiers::SHIFT);
    desk.click(Point::new(bounds.x + 150., bounds.center_y()));
    desk.modifiers(Modifiers::empty());
    let selected = desk.selection().expect("a selection");
    // About 90 pixels of the scrolled end of the text.
    let at = LONG.rfind(&selected).expect("a run of the text");
    assert!((10..=16).contains(&selected.len()), "{selected:?}");
    assert!(at > 70, "{selected:?} at {at}");
}

#[test]
fn aligned_and_secure_inputs_hit_where_they_draw() {
    // A short value sits at the end or the middle of the field.
    for (alignment, value) in [
        (iced::Alignment::End, "short"),
        (iced::Alignment::Center, "short"),
    ] {
        let view = input("")
            .default_value(value)
            .width(200)
            .align_x(alignment)
            .on_input(|s| s);
        let mut desk = Desk::new(view);
        let (bounds, _) = desk.field();
        desk.click(Point::new(bounds.x + 14., bounds.center_y()));
        assert_eq!(desk.caret(), 0, "{alignment:?}: before the text");
        desk.click(Point::new(bounds.x + bounds.width - 2., bounds.center_y()));
        assert_eq!(desk.caret(), value.len(), "{alignment:?}: after the text");
    }
    // A scrolled password field types where the press lands.
    let view = input("")
        .default_value(LONG)
        .secure(true)
        .width(200)
        .on_input(|s| s);
    let mut desk = Desk::new(view);
    let (bounds, _) = desk.field();
    desk.click(Point::new(bounds.x + 20., bounds.center_y()));
    desk.key(Named::End, Modifiers::empty());
    desk.click(Point::new(bounds.x + 13., bounds.center_y()));
    desk.send(simulator::typewrite("!"));
    let typed = desk.messages.last().expect("an edit").find('!').unwrap();
    assert!(
        typed > 40 && typed < LONG.len() - 5,
        "typed at {typed}, inside the scrolled text"
    );
}

const CODE: &str = "fn main() { println!(\"a line that is much longer than the code block is wide, so it has to scroll\"); }";

/// An editable code block 320 pixels wide holding `value`.
fn block(value: &str) -> Element<'static, String, Theme> {
    container(code_block![
        code_block::content("")
            .language("rust")
            .default_value(value.to_owned())
            .on_input(|s| s)
    ])
    .width(320)
    .into()
}

impl Desk {
    fn wheel(&mut self, at: Point, x: f32) {
        self.move_to(at);
        self.send([Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Pixels { x, y: 0. },
        })]);
        // The scroller eases to its offset over the next frames.
        let start = Instant::now();
        self.send([frame(start, 0)]);
        self.send([frame(start, 2000)]);
    }

    /// Where `@` lands when typed at the caret, which it then takes back.
    fn typed(&mut self) -> usize {
        self.send(simulator::typewrite("@"));
        let at = self.field().1.find('@').expect("typed");
        self.key(Named::Backspace, Modifiers::empty());
        at
    }
}

#[test]
fn a_press_on_a_scrolled_code_block_lands_under_the_pointer() {
    // Two blocks, so the second press is not a double click on the same spot.
    let mut desk = Desk::new(block(CODE));
    let (line, _) = desk.field();
    let y = line.y + 8.;
    desk.click(Point::new(line.x + 150., y));
    let unscrolled = desk.typed();
    let mut desk = Desk::new(block(CODE));
    desk.wheel(Point::new(line.x + 100., y), -90.);
    desk.click(Point::new(line.x + 60., y));
    assert_eq!(
        desk.typed(),
        unscrolled,
        "90 pixels scrolled, 90 pixels left"
    );
}

#[test]
fn dragging_past_the_edge_of_a_code_block_scrolls_and_selects_to_the_end() {
    let mut desk = Desk::new(block(CODE));
    let (line, _) = desk.field();
    let y = line.y + 8.;
    desk.press(Point::new(line.x + 100., y));
    desk.move_to(Point::new(line.x + 300., y));
    // Past the block's right edge, where its scroller hides the pointer.
    desk.move_to(Point::new(line.x + 360., y));
    let start = Instant::now();
    let mut selections = Vec::new();
    for at in (0..=30).map(|n| n * 50) {
        desk.send([frame(start, at)]);
        selections.push(desk.dragged().len());
    }
    desk.release();
    assert!(
        selections.windows(2).filter(|w| w[1] > w[0]).count() > 3,
        "frames extend the selection: {selections:?}"
    );
    let selected = desk.selection().expect("a selection");
    assert!(CODE.ends_with(&selected), "{selected:?}");
}

/// Thirty numbered lines.
fn numbered() -> String {
    (0..30)
        .map(|n| format!("line {n:02}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// A textarea holding thirty lines, dragged by its grip down to 100 pixels
/// tall, so it scrolls.
fn short_textarea() -> (Desk, Rectangle) {
    let mut desk = Desk::new(textarea("").default_value(numbered()).on_input(|s| s));
    let (bounds, _) = desk.field();
    let grip = Point::new(bounds.x + bounds.width - 4., bounds.y + bounds.height - 4.);
    desk.press(grip);
    desk.move_to(Point::new(grip.x, bounds.y + 96.));
    desk.release();
    let (bounds, _) = desk.field();
    assert!(bounds.height < 110., "resized to {bounds:?}");
    (desk, bounds)
}

#[test]
fn dragging_below_a_scrolling_textarea_scrolls_and_selects_to_the_end() {
    let (mut desk, bounds) = short_textarea();
    desk.press(Point::new(bounds.x + 14., bounds.y + 14.));
    desk.move_to(Point::new(bounds.x + 40., bounds.y + bounds.height + 40.));
    let start = Instant::now();
    let mut selections = Vec::new();
    for at in (0..=60).map(|n| n * 50) {
        desk.send([frame(start, at)]);
        selections.push(desk.dragged().len());
    }
    desk.release();
    assert!(
        selections.windows(2).filter(|w| w[1] > w[0]).count() > 5,
        "frames extend the selection: {selections:?}"
    );
    let selected = desk.selection().expect("a selection");
    assert!(
        selected.contains("line 28\nline"),
        "reaches the last line: {selected:?}"
    );
}

#[test]
fn a_press_on_a_scrolled_textarea_lands_on_the_line_under_the_pointer() {
    let (mut desk, bounds) = short_textarea();
    desk.click(Point::new(bounds.x + 14., bounds.y + 14.));
    for _ in 0..30 {
        desk.key(Named::ArrowDown, Modifiers::empty());
    }
    desk.redraw();
    desk.click(Point::new(
        bounds.x + bounds.width - 30.,
        bounds.y + bounds.height - 14.,
    ));
    desk.send(simulator::typewrite("@"));
    assert!(desk.field().1.ends_with("line 29@"), "{:?}", desk.field().1);
}

#[test]
fn the_input_caret_sits_on_whole_pixels() {
    for (alignment, value) in [
        (iced::Alignment::Center, "ab"),
        (iced::Alignment::Center, "abc"),
        (iced::Alignment::End, "wxyz"),
        (iced::Alignment::Start, LONG),
    ] {
        let view = input("")
            .default_value(value)
            .width(201)
            .align_x(alignment)
            .on_input(|s| s);
        let mut desk = Desk::with(view, Some("tiny-skia"));
        let (bounds, _) = desk.field();
        // The caret goes to the end, past the text.
        desk.click(Point::new(bounds.x + bounds.width - 3., bounds.center_y()));
        desk.move_to(Point::new(700., 500.));
        let pixels = desk.pixels();
        let row = &pixels[bounds.center_y() as usize];
        let inside = bounds.x as usize + 4..(bounds.x + bounds.width) as usize - 4;
        let background = row[inside.end - 1];
        let caret = inside
            .clone()
            .rev()
            .find(|&x| row[x] != background)
            .expect("a caret");
        let ink = row[caret];
        // The caret is the darkest ink: one column fully covered, not two
        // half-covered ones.
        let darkest = *row[inside].iter().min().unwrap();
        assert!(
            ink <= darkest + 8,
            "{alignment:?} {value:?}: caret column {caret} is {ink}, ink is {darkest}: {:?}",
            &row[caret - 3..caret + 3]
        );
    }
}

fn type_text(desk: &mut Desk, text: &str) {
    desk.send(simulator::typewrite(text));
}

fn undo(desk: &mut Desk) {
    desk.key(character("z"), Modifiers::COMMAND);
}

fn redo(desk: &mut Desk) {
    desk.key(character("z"), Modifiers::COMMAND | Modifiers::SHIFT);
}

#[test]
fn an_input_undoes_a_run_of_typing_as_one_step_and_redoes_it() {
    let mut desk = Desk::new(input("").on_input(|s| s));
    desk.focus();
    type_text(&mut desk, "hello");
    desk.key(Named::ArrowLeft, Modifiers::empty());
    desk.key(Named::ArrowRight, Modifiers::empty());
    type_text(&mut desk, " world");
    undo(&mut desk);
    assert_eq!(desk.field().1, "hello", "the second run is one step");
    assert_eq!(desk.messages.last().map(String::as_str), Some("hello"));
    // The caret is back where the run started.
    type_text(&mut desk, "!");
    assert_eq!(desk.field().1, "hello!");
    undo(&mut desk);
    undo(&mut desk);
    assert_eq!(desk.field().1, "");
    redo(&mut desk);
    assert_eq!(desk.field().1, "hello");
    desk.key(character("y"), Modifiers::COMMAND);
    assert_eq!(desk.field().1, "hello!", "Command+Y redoes too");
    redo(&mut desk);
    assert_eq!(desk.field().1, "hello!", "nothing more to redo");
}

#[test]
fn an_input_undoes_a_paste_a_cut_and_a_word_deletion_as_steps_of_their_own() {
    let mut desk = Desk::new(input("").on_input(|s| s));
    desk.focus();
    type_text(&mut desk, "one two");
    desk.clipboard = Some(" pasted".into());
    desk.key(character("v"), Modifiers::COMMAND);
    type_text(&mut desk, " three");
    // Option+Backspace on macOS, Ctrl+Backspace elsewhere.
    let jump = if cfg!(target_os = "macos") {
        Modifiers::ALT
    } else {
        Modifiers::CTRL
    };
    desk.key(Named::Backspace, jump);
    assert_eq!(desk.field().1, "one two pasted ");
    desk.key(character("a"), Modifiers::COMMAND);
    desk.key(character("x"), Modifiers::COMMAND);
    assert_eq!(desk.field().1, "");
    undo(&mut desk);
    assert_eq!(desk.field().1, "one two pasted ", "the cut");
    assert_eq!(
        desk.selection().as_deref(),
        Some("one two pasted "),
        "with its selection"
    );
    desk.key(Named::End, Modifiers::empty());
    undo(&mut desk);
    assert_eq!(desk.field().1, "one two pasted three", "the word deletion");
    undo(&mut desk);
    assert_eq!(desk.field().1, "one two pasted", "the typing");
    undo(&mut desk);
    assert_eq!(desk.field().1, "one two", "the paste");
}

#[test]
fn the_input_menu_undoes_and_redoes() {
    let mut desk = Desk::new(input("").on_input(|s| s));
    let (bounds, _) = desk.field();
    desk.focus();
    type_text(&mut desk, "menu");
    desk.click_with(Point::new(bounds.x + 14., bounds.center_y()), Button::Right);
    desk.choose("Undo");
    assert_eq!(desk.field().1, "");
    desk.click_with(Point::new(bounds.x + 14., bounds.center_y()), Button::Right);
    desk.choose("Redo");
    assert_eq!(desk.field().1, "menu");
}

/// A controlled input and textarea showing the app's `value`.
fn controlled(index: usize, value: &str) -> Element<'static, String, Theme> {
    match index {
        0 => input("").value(value.to_owned()).on_input(|s| s).into(),
        1 => textarea("").value(value.to_owned()).on_input(|s| s).into(),
        _ => code_block![code_block::content(value.to_owned()).on_input(|s| s)].into(),
    }
}

#[test]
fn a_controlled_field_undoes_its_edits_but_not_the_apps() {
    for index in 0..3 {
        let mut value = String::new();
        let mut desk = Desk::new(controlled(index, &value));
        desk.focus();
        for c in ["a", "b", "c"] {
            type_text(&mut desk, c);
            // iced can rebuild before the app handles the edit.
            desk.rebuild(controlled(index, &value));
            value = desk.messages.last().cloned().unwrap_or_default();
            desk.rebuild(controlled(index, &value));
        }
        assert_eq!(value, "abc", "{index}");
        undo(&mut desk);
        value = desk.messages.last().cloned().unwrap_or_default();
        desk.rebuild(controlled(index, &value));
        assert!(value.len() < 3, "{index}: undone to {value:?}");
        redo(&mut desk);
        value = desk.messages.last().cloned().unwrap_or_default();
        desk.rebuild(controlled(index, &value));
        assert_eq!(value, "abc", "{index}: redone");

        // The app replaces the text: that is no step, and the steps before
        // it no longer apply.
        value = "reset".into();
        desk.rebuild(controlled(index, &value));
        let published = desk.messages.len();
        undo(&mut desk);
        assert_eq!(desk.field().1, "reset", "{index}");
        assert_eq!(desk.messages.len(), published, "{index}: nothing to undo");
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
        "textarea" => textarea("").default_value(value).on_input(|s| s).into(),
        _ => code_block![code_block::content("").default_value(value).on_input(|s| s)].into(),
    }
}

#[test]
fn a_double_click_drag_in_any_field_selects_whole_words() {
    let words = "aa bbbbbbbbbbbbbbbbbbbbbbbb cc";
    for name in ["input", "textarea", "code block"] {
        let mut desk = Desk::new(field(name, words));
        let (bounds, _) = desk.field();
        let y = bounds.y + 10.;
        // Well inside the long word.
        let word = Point::new(bounds.x + 70., y);
        desk.click(word);
        desk.press(word);
        desk.move_to(Point::new(bounds.x + 380., y));
        assert_eq!(
            desk.dragged(),
            "bbbbbbbbbbbbbbbbbbbbbbbb cc",
            "{name}: forward, from the start of the word"
        );
        desk.move_to(Point::new(bounds.x + 1., y));
        desk.release();
        assert_eq!(
            desk.selection().as_deref(),
            Some("aa bbbbbbbbbbbbbbbbbbbbbbbb"),
            "{name}: backward, to the end of the word"
        );
    }
}
