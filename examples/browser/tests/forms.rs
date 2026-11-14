//! Exercise the installed browser previews through the same component entries as the sidebar.
use std::borrow::Cow;
use std::path::PathBuf;

use browser::component::calendar::CalendarPage;
use browser::component::date_picker::DatePickerPage;
use browser::component::input::InputPage;
use browser::component::input_otp::InputOtpPage;
use browser::page::Page;
use browser::{Message, icon};
use iced::keyboard::key::Named;
use iced::{Point, Size};
use iced_test::core::Settings;
use iced_test::simulator::{self, Simulator};
use icedcn::Theme;
use icedcn::widget::calendar::{Datelike, today};

type Preview<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

fn preview<'a>(page: &'a dyn Page, id: &str, width: f32) -> Preview<'a> {
    let mut settings = Settings::default();
    settings.fonts.push(Cow::Borrowed(icon::FONT));
    Simulator::with_size(settings, Size::new(width, 720.), page.preview(id))
}

fn snapshot(ui: &mut Preview<'_>, name: &str) {
    if let Some(dir) = std::env::var_os("ICEDCN_FORM_SNAPSHOTS") {
        let path = PathBuf::from(dir).join(format!("{name}.png"));
        let image = ui.snapshot(&Theme::light()).expect("render preview");
        assert!(image.matches_image(path).expect("write or compare preview"));
    }
}

#[test]
fn hijri_demo_displays_its_fixed_persian_date_and_navigates_months() {
    let page = CalendarPage::new(1);
    let mut ui = preview(&*page, "hijri", 375.);
    ui.find("خرداد ۱۴۰۴")
        .expect("June 12, 2025 is Khordad 22, 1404");
    ui.find("۲۲").expect("selected Persian day");
    snapshot(&mut ui, "calendar-persian-375");
    ui.point_at(Point::new(29., 29.));
    ui.simulate(simulator::click()); // Next month is on the left in the Persian grid.
    ui.find("تیر ۱۴۰۴").expect("caption changes to Tir");
}

#[test]
fn api_key_stays_masked_while_typing_in_the_uncontrolled_input() {
    let page = InputPage::new(0);
    let mut ui = preview(&*page, "demo", 375.);
    ui.click("API Key").expect("focus the input from its label");
    ui.typewrite("aB90");
    ui.find("••••")
        .expect("each typed character is masked before the next rebuild");
    assert!(ui.find("aB90").is_err());
}

#[test]
fn otp_controlled_preview_updates_the_visible_code_and_filters_digits() {
    let page = InputOtpPage::new(2);
    let mut ui = preview(&*page, "controlled", 375.);
    ui.point_at(Point::new(14., 16.));
    let _ = ui.simulate(simulator::click());
    ui.typewrite("A4B7C9EXTRA");
    ui.find("You entered: A4B7C9")
        .expect("caption reflects length-limited code");
    snapshot(&mut ui, "otp-controlled-375");
}

#[test]
fn date_picker_commits_the_selected_day_and_dismisses_the_popover() {
    let page = DatePickerPage::new(3);
    let mut ui = preview(&*page, "dob", 375.);
    ui.click("Select date").expect("open DOB picker");
    snapshot(&mut ui, "date-picker-dob-open-375");
    let base = today();
    let month = base.format("%B").to_string();
    ui.click(month).expect("open DOB month choices");
    let alternate = if base.format("%m").to_string() == "03" {
        "April"
    } else {
        "March"
    };
    ui.click(alternate).expect("choose a different DOB month");
    ui.click(base.format("%Y").to_string())
        .expect("open DOB years");
    snapshot(&mut ui, "date-picker-years-375");
    ui.click((base.format("%Y").to_string().parse::<i32>().unwrap() - 1).to_string())
        .expect("choose a DOB year");
    snapshot(&mut ui, "date-picker-selected-month");
    ui.click("15").expect("select a day in the visible month");
    // Overlay messages invalidate the widget tree. The application shell rebuilds it.
    ui = ui.rebuild(page.preview("dob"));
    let expected = format!(
        "{}/15/{}",
        if alternate == "March" { 3 } else { 4 },
        base.year() - 1
    );
    ui.find(expected).expect("selected DOB updates the trigger");
    assert!(ui.find("Select date").is_err());
    assert!(ui.find("15").is_err());
}

#[test]
fn time_and_range_previews_render_at_phone_and_desktop_widths() {
    let page = CalendarPage::new(4);
    for (width, id) in [(375., "range"), (760., "range"), (375., "time")] {
        let mut ui = preview(&*page, id, width);
        if id == "range" {
            let year = today().format("%Y").to_string();
            let january = ui
                .find(format!("January {year}"))
                .expect("January")
                .bounds();
            let february = ui
                .find(format!("February {year}"))
                .expect("February")
                .bounds();
            if width < 500. {
                assert!(february.y > january.y + 100., "phone stacks months");
            } else {
                assert!(
                    february.x > january.x + 100.,
                    "desktop keeps months side by side"
                );
            }
        } else {
            ui.find("Start Time").expect("start time");
            ui.click("10:30:00").expect("focus editable start time");
            ui.tap_key(Named::End);
            ui.tap_key(Named::Backspace);
            let _ = ui.typewrite("5");
            ui.find("10:30:05").expect("start time changes");
            ui.find("12:30:00").expect("end time unaffected");
        }
        snapshot(&mut ui, &format!("calendar-{id}-{width}"));
    }
}
