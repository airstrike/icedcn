//! Exercise the transport through the installed, rendered chat demo.
use std::borrow::Cow;
use std::time::Duration;

use browser::component::scroller;
use browser::icon;
use iced::time::Instant;
use iced::{Event, Rectangle, Size, Vector, window};
use iced_test::core::Settings;
use iced_test::simulator::Simulator;
use iced_widget::core::widget::{Id, Operation, operation};

#[derive(Default)]
struct Transcript {
    offset: f32,
    top: f32,
    text: Vec<(String, f32)>,
}

impl Operation for Transcript {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        _: Size,
        offset: Vector,
        _: &mut dyn operation::Scrollable,
    ) {
        self.top = bounds.y;
        self.offset = offset.y;
    }

    fn text(&mut self, _: Option<&Id>, bounds: Rectangle, text: &mut dyn operation::Text) {
        self.text
            .push((text.text().into_owned(), bounds.y - self.top - self.offset));
    }
}

#[test]
fn first_prompt_then_first_reply_and_next_anchor_follow_the_live_timeline() {
    let page = scroller::Page::new(0);
    let mut settings = Settings::default();
    settings.fonts.push(Cow::Borrowed(icon::FONT));
    let mut ui = Simulator::with_size(settings, Size::new(384., 660.), page.preview("demo"));
    let send = icon::ALL_ICONS
        .iter()
        .find(|(name, _)| *name == "arrow_up")
        .unwrap()
        .1;
    let first = "I'm building a chat for our app";
    let second = "Okay, but when someone sends a new message";

    ui.click(send).unwrap();
    let start = Instant::now();
    let mut at = |ms| {
        ui.simulate([Event::Window(window::Event::RedrawRequested(
            start + Duration::from_millis(ms),
        ))]);
        let mut transcript = Transcript::default();
        ui.operate(&mut transcript);
        transcript
    };

    let pending = at(850);
    let prompt_text = pending
        .text
        .iter()
        .find(|(t, _)| t.starts_with(first))
        .unwrap();
    // Each user bubble adds 10px of internal padding above its text.
    assert!(
        (prompt_text.1 - 30.).abs() <= 2.,
        "first user text at {}",
        prompt_text.1
    );
    assert!(!pending.text.iter().any(|(t, _)| t.starts_with("That's")));
    let typing = at(1150);
    let reply = typing
        .text
        .iter()
        .find(|(t, _)| t.starts_with("That's"))
        .unwrap();
    assert!(
        reply.0.starts_with("That's the classic streaming") && reply.0.len() < 90,
        "{}",
        reply.0
    );
    let done = at(5000);
    let finished = done
        .text
        .iter()
        .find(|(t, _)| t.starts_with("That's"))
        .unwrap();
    assert!(finished.0.ends_with("land in place."));
    assert_eq!(done.offset, 0.);

    ui.click(send).unwrap();
    let start = Instant::now();
    ui.simulate([Event::Window(window::Event::RedrawRequested(
        start + Duration::from_millis(850),
    ))]);
    let mut pending = Transcript::default();
    ui.operate(&mut pending);
    let next = pending
        .text
        .iter()
        .find(|(t, _)| t.starts_with(second))
        .unwrap();
    assert!((next.1 - 94.).abs() <= 2., "new user text at {}", next.1);
    assert!(
        !pending
            .text
            .iter()
            .any(|(t, _)| t.starts_with("MessageScrollerItem fixes"))
    );
}
