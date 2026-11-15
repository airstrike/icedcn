//! Mounted article metrics and its inline navigation link.
use std::borrow::Cow;

use browser::Message;
use browser::component::text::TextPage;
use browser::page;
use iced::{Font, Size};
use iced_test::core::Settings;
use iced_test::{Simulator, simulator};
use icedcn::Theme;

#[test]
fn the_article_balances_headings_and_keeps_its_inline_link_interactive() {
    let page = TextPage::new(7);
    let settings = Settings {
        font: Font::new("Geist"),
        fonts: vec![Cow::Borrowed(include_bytes!("../fonts/Geist.ttf"))],
        ..Default::default()
    };
    let title = "Taxing Laughter: The Joke Tax Chronicles";
    for (width, height) in [(384., 80.), (230., 160.)] {
        let mut ui = Simulator::<Message, Theme, iced::Renderer>::with_size(
            settings.clone(),
            Size::new(width, 200.),
            page.preview("h1"),
        );
        // The shaped paragraph also reserves the font’s ascender/descent overhang.
        let actual = ui.find(title).unwrap().bounds().height;
        assert!((height..height + 8.).contains(&actual), "{actual}");
    }

    let mut code = Simulator::<Message, Theme, iced::Renderer>::with_size(
        settings.clone(),
        Size::new(230., 120.),
        page.preview("inline-code"),
    );
    assert!(
        code.find("widget::alert_dialog::AlertDialog")
            .unwrap()
            .bounds()
            .height
            > 30.
    );

    let mut ui = Simulator::<Message, Theme, iced::Renderer>::with_size(
        settings,
        Size::new(384., 2200.),
        page.preview("demo"),
    );
    let heading = ui.find("The King's Plan").unwrap().bounds();
    ui.point_at([90., heading.y + 112.]);
    ui.draw(&Theme::light());
    ui.simulate(simulator::click());
    assert!(
        ui.drain()
            .any(|event| matches!(event, Message::Page(7, page::Message::ScrollTo(0))))
    );
    ui.find("The People's Rebellion").unwrap();
    ui.find("Ecstatic").unwrap();
}
