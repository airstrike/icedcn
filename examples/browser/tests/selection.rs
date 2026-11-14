//! Exercise native paragraph selection through the same input and clipboard path as the window.
use browser::app::{App, Message};
use browser::component::scroller;
use browser::page::{self, SnippetViewer};
use iced::keyboard::{self, Key, Modifiers};
use iced::{Element, Event, Point, Rectangle, Renderer, Size, clipboard, event, mouse, window};
use iced_test::core::renderer::{Headless, Style};
use iced_test::core::shell::{Bus, Waker};
use iced_test::core::widget::{Operation, operation};
use iced_test::futures::futures::{StreamExt, executor::block_on};
use iced_test::runtime::{Action, UserInterface, task, user_interface};
use iced_test::{Selector, simulator};
use icedxdy::Theme;
use icedxdy::theme::tweak::Token;
use icedxdy::widget::markdown::highlighter::Map;
use icedxdy::widget::{
    bubble, button, column, input, markdown, message, rich_text, row, span, text,
};

type Ui<'a> = UserInterface<'a, Message, Theme, Renderer>;

fn renderer() -> Renderer {
    let backend = std::env::var("ICED_TEST_BACKEND").ok();
    block_on(Renderer::new(Default::default(), backend.as_deref())).expect("headless text renderer")
}

fn bounds(ui: &mut Ui<'_>, renderer: &Renderer, value: &str) -> Rectangle {
    let mut find = Selector::find(value);
    ui.operate(renderer, &mut operation::black_box(&mut find));
    match find.finish() {
        operation::Outcome::Some(Some(target)) => target.visible_bounds().expect("visible text"),
        _ => panic!("missing text {value:?}"),
    }
}

fn send(
    ui: &mut Ui<'_>,
    renderer: &mut Renderer,
    position: Point,
    event: Event,
) -> (event::Status, Option<clipboard::Content>, Vec<Message>) {
    let mut bus = Bus::new();
    let (state, statuses) = ui.update(
        &window::Headless,
        &Waker::noop(),
        &[event],
        mouse::Cursor::Available(position),
        renderer,
        &mut bus,
    );
    let write = match state {
        user_interface::State::Updated { clipboard, .. } => clipboard.write,
        user_interface::State::Outdated => None,
    };
    (statuses[0], write, bus.into_iter().collect())
}

fn colored_pixels(
    ui: &mut Ui<'_>,
    renderer: &mut Renderer,
    bounds: Rectangle,
    theme: &Theme,
) -> Vec<u8> {
    ui.draw(
        renderer,
        theme,
        &Style::default(),
        mouse::Cursor::Unavailable,
    );
    let size = Size::new(440, 680);
    let image = renderer.screenshot(size, 1., iced::Color::WHITE);
    let mut pixels = Vec::new();
    for y in bounds.y.ceil() as usize..(bounds.y + bounds.height).floor() as usize {
        let row = y * size.width as usize;
        pixels.extend_from_slice(
            &image[(row + bounds.x.ceil() as usize) * 4
                ..(row + (bounds.x + bounds.width).floor() as usize) * 4],
        );
    }
    pixels
}

fn command(letter: &str) -> Event {
    let mut event = simulator::press_key(Key::Character(letter.into()), None);
    if let Event::Keyboard(keyboard::Event::KeyPressed { modifiers, .. }) = &mut event {
        *modifiers = Modifiers::COMMAND;
    }
    event
}

fn drag(ui: &mut Ui<'_>, renderer: &mut Renderer, start: Point, end: Point) {
    send(
        ui,
        renderer,
        start,
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
    );
    send(
        ui,
        renderer,
        end,
        Event::Mouse(mouse::Event::CursorMoved { position: end }),
    );
    send(
        ui,
        renderer,
        end,
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    );
}

#[test]
fn drag_copies_prose_transcript_and_highlighted_code_in_reading_order() {
    let mut renderer = renderer();
    let theme = Theme::light();
    let viewer = SnippetViewer::new(Map::from(&theme), &theme);
    let items = markdown::parse("Read **this** first.\n\n```rust\nfn answer() {\n\n    42\n}\n```")
        .collect();
    let code = "fn answer() {\n\n    42\n}";
    let article: Element<'_, Message, Theme> = column![
        text::h1("Heading"),
        message(bubble("Transcript paragraph")),
        rich_text([span("Colored ").color(Token::Warning), span("runs")]),
        button(row!["An action"]).on_press(Message::ToggleTheme),
        page::snippet(0, &items, &viewer),
        text("After the listing")
    ]
    .spacing(12)
    .width(440)
    .into();
    let mut ui = Ui::build(
        article,
        Size::new(440., 680.),
        Default::default(),
        &mut renderer,
    );
    let title = bounds(&mut ui, &renderer, "Heading");
    let listing = bounds(&mut ui, &renderer, code);
    let colored = bounds(&mut ui, &renderer, "Colored runs");
    let before = colored_pixels(&mut ui, &mut renderer, colored, &theme);
    assert!(
        before
            .as_chunks::<4>()
            .0
            .iter()
            .any(|rgba| rgba[0].abs_diff(rgba[2]) > 32),
        "the renderer retains the colored glyphs"
    );
    let start = Point::new(title.x, title.center_y());
    let end = Point::new(listing.x + listing.width, listing.y + listing.height - 1.);
    drag(&mut ui, &mut renderer, start, end);
    let (_, clipboard, _) = send(&mut ui, &mut renderer, end, command("c"));
    assert_eq!(
        clipboard,
        Some(
            format!("Heading\nTranscript paragraph\nColored runs\nRead this first.\n{code}").into()
        )
    );

    let selected = colored_pixels(&mut ui, &mut renderer, colored, &theme);
    assert_ne!(
        before, selected,
        "the selected rich paragraph paints its highlight"
    );

    let action = bounds(&mut ui, &renderer, "An action").center();
    send(
        &mut ui,
        &mut renderer,
        action,
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
    );
    let (_, _, messages) = send(
        &mut ui,
        &mut renderer,
        action,
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    );
    assert!(
        messages
            .iter()
            .any(|message| matches!(message, Message::ToggleTheme))
    );
}

#[test]
fn select_all_targets_the_article_and_focused_inputs_keep_their_shortcuts() {
    let mut renderer = renderer();
    let theme = Theme::light();
    let mut app = App::new(Map::from(&theme), Map::from(&theme)).0;
    let task = app.update(Message::SelectArticle);
    let mut actions = task::into_stream(task).expect("article operation");
    let Some(Action::Widget(mut select)) = block_on(actions.next()) else {
        panic!("expected a scoped text operation");
    };
    let size = Size::new(1440., 900.);
    let mut ui = Ui::build(app.view(), size, Default::default(), &mut renderer);
    ui.operate(&renderer, &mut select);
    let (_, clipboard, _) = send(&mut ui, &mut renderer, Point::ORIGIN, command("c"));
    let Some(clipboard::Content::Text(article)) = clipboard else {
        panic!("empty article selection")
    };
    assert!(article.starts_with("Accordion\nA vertically stacked set"));
    assert!(article.contains("\nBasic\n"));
    for chrome in [
        "On This Page",
        "Sections",
        "Components",
        "Home",
        "View Code",
    ] {
        assert!(!article.lines().any(|line| line == chrome), "{chrome}");
    }

    let field: Element<'_, Message, Theme> =
        column![text("Other text"), input("").default_value("field only")].into();
    let mut ui = Ui::build(field, size, Default::default(), &mut renderer);
    let point = bounds(&mut ui, &renderer, "field only").center();
    for event in simulator::click() {
        send(&mut ui, &mut renderer, point, event);
    }
    send(
        &mut ui,
        &mut renderer,
        point,
        Event::Keyboard(keyboard::Event::ModifiersChanged(Modifiers::COMMAND)),
    );
    assert_eq!(
        send(&mut ui, &mut renderer, point, command("a")).0,
        event::Status::Captured
    );
    let (status, content, _) = send(&mut ui, &mut renderer, point, command("c"));
    assert_eq!(status, event::Status::Captured);
    assert_eq!(content, Some("field only".to_owned().into()));
}

#[test]
fn drag_backwards_across_turns_in_a_scrolled_transcript() {
    let page = scroller::Page::new(0);
    let mut renderer = renderer();
    let mut ui = Ui::build(
        page.preview("opening-position"),
        Size::new(384., 680.),
        Default::default(),
        &mut renderer,
    );
    let user = "This is the last message the user sent in the conversation.";
    let reply = "Start with the invite step. Teams are creating workspaces but waiting to add collaborators.";
    let prompt = bounds(&mut ui, &renderer, user);
    let answer = bounds(&mut ui, &renderer, reply);
    let start = Point::new(answer.x + answer.width, answer.y + answer.height - 1.);
    let end = Point::new(prompt.x, prompt.y + 1.);
    drag(&mut ui, &mut renderer, start, end);
    let (_, clipboard, _) = send(&mut ui, &mut renderer, end, command("c"));
    assert_eq!(clipboard, Some(format!("{user}\n{reply}").into()));
}
