//! Public behavior of the installed Date Picker examples.
use browser::Message;
use browser::component::date_picker::DatePickerPage;
use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::{Event, Rectangle, Size, window};
use iced_test::Simulator;
use iced_widget::core::widget::{Id, Operation, operation::TextInput};
use icedcn::Theme;
use icedcn::widget::calendar::Date;

type Ui<'a> = Simulator<'a, Message, Theme, iced::Renderer>;
fn frame(ui: &mut Ui<'_>) {
    ui.draw(&Theme::light());
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    ui.draw(&Theme::light());
}
struct SelectText;
impl Operation for SelectText {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self)
    }
    fn text_input(&mut self, _: Option<&Id>, _: Rectangle, input: &mut dyn TextInput) {
        input.select_all()
    }
}

#[test]
fn typing_a_date_opens_that_month_and_commit_replaces_the_edit() {
    let page = DatePickerPage::new(0);
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(390., 844.),
        page.preview("input"),
    );
    frame(&mut ui);
    ui.click("Subscription Date").unwrap();
    ui.operate(&mut SelectText);
    ui.typewrite("2024-02-29");
    ui.tap_key(Named::ArrowDown);
    frame(&mut ui);
    ui.find("February 2024").unwrap();
    ui.click("15").unwrap();
    ui = ui.rebuild(page.preview("input"));
    frame(&mut ui);
    ui.find("February 15, 2024").unwrap();
    assert!(ui.find("February 2024").is_err());
}

#[test]
fn hero_retains_its_calendar_until_escape_and_reopens_the_chosen_month() {
    let base = Date::from_ymd_opt(2025, 11, 20).unwrap();
    let page = DatePickerPage::at(0, base);
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(390., 844.),
        page.preview("demo"),
    );
    frame(&mut ui);
    ui.click("Pick a date").unwrap();
    frame(&mut ui);
    ui.click("15").unwrap();
    ui = ui.rebuild(page.preview("demo"));
    frame(&mut ui);
    ui.find("November 15th, 2025").unwrap();
    ui.find("November 2025").unwrap();
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(page.preview("demo"));
    frame(&mut ui);
    assert!(ui.find("November 2025").is_err());
    ui.tap_key(Named::Enter);
    frame(&mut ui);
    ui.find("November 2025").unwrap();
}

#[test]
fn range_stays_open_and_reflows_its_two_months() {
    let base = Date::from_ymd_opt(2025, 11, 20).unwrap();
    for width in [390., 1440.] {
        let mut page = DatePickerPage::at(0, base);
        page.window_width(width);
        let mut ui = Ui::with_size(
            Default::default(),
            Size::new(width, 844.),
            page.preview("range"),
        );
        frame(&mut ui);
        ui.find("Jan 20, 2025 - Feb 09, 2025").unwrap();
        ui.click("Date Picker Range").unwrap();
        frame(&mut ui);
        let first = ui.find("January 2025").unwrap().bounds();
        let second = ui.find("February 2025").unwrap().bounds();
        assert_eq!(second.y > first.y + 100., width < 768.);
        ui.click("15").unwrap();
        ui = ui.rebuild(page.preview("range"));
        frame(&mut ui);
        ui.find("February 2025").unwrap();
    }
}
