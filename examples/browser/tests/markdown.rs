//! Markdown cells keep the surrounding paragraph's line box for inline code.
use std::borrow::Cow;

use iced::{Pixels, Renderer, Size};
use iced_test::Simulator;
use iced_test::core::Settings;
use icedcn::Theme;
use icedcn::widget::markdown::{self, LineHeight};

#[test]
fn code_only_and_mixed_table_cells_share_the_computed_line_height() {
    let items = markdown::parse(
        "| Builder | Type | Default |\n\
         | --- | --- | --- |\n\
         | `.size(size)` | `Size::Default` or `Size::Sm` | `Size::Default` |",
    )
    .collect::<Vec<_>>();
    let renderer = Settings {
        font: "Geist".into(),
        fonts: vec![Cow::Borrowed(include_bytes!("../fonts/Geist.ttf"))],
        ..Default::default()
    };

    for (size, line_height, expected) in [
        (16., LineHeight::Relative(1.5), 24.),
        (20., LineHeight::Relative(1.5), 30.),
        (16., LineHeight::Absolute(Pixels(28.)), 28.),
    ] {
        let mut settings = markdown::Settings::with_text_size(size, Theme::light().seed);
        settings.line_height = line_height;
        let mut ui = Simulator::<markdown::Uri, Theme, Renderer>::with_size(
            renderer.clone(),
            Size::new(800., 160.),
            markdown::view(&items, settings),
        );
        let builder = ui.find(".size(size)").unwrap().bounds();
        let mixed = ui.find("Size::Default or Size::Sm").unwrap().bounds();
        let default = ui.find("Size::Default").unwrap().bounds();
        for bounds in [builder, mixed, default] {
            assert!((bounds.height - expected).abs() < 0.01, "{bounds:?}");
            assert!((bounds.y - mixed.y).abs() < 0.01, "{bounds:?} / {mixed:?}");
        }
        ui.draw(&Theme::light());
        ui.draw(&Theme::dark());
    }
}
