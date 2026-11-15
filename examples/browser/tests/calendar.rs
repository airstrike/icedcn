//! Calendar's date navigation, state ownership and installed recipe geometry.
use browser::component::calendar::CalendarPage;
use browser::component::calendar::recipes;
use iced::keyboard::key::Named;
use iced::widget::Id;
use iced::{Element, Point, Rectangle, Size};
use iced_test::core::Settings;
use iced_test::core::widget::operation::{self, Focusable, Operation, Outcome, Text, focusable};
use iced_test::simulator::{self, Simulator};
use icedcn::Theme;
use icedcn::widget::calendar::{Date, Selection, persian};
use icedcn::widget::{Inherited, Slot, calendar};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Pick(Option<Selection>),
    Month(Date),
}
type Preview = Simulator<'static, Message, Theme, iced::Renderer>;
fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(2026, month, day).unwrap()
}
fn picker(selected: u32, controlled: bool, disabled: bool) -> Element<'static, Message, Theme> {
    let mut picker = calendar()
        .default_month(date(1, 1))
        .default_selected(Selection::single(date(1, selected)))
        .unavailable(|d| d == date(1, 16))
        .on_select(Message::Pick)
        .on_month_change(Message::Month);
    if controlled {
        picker = picker.selected(Some(Selection::single(date(1, selected))));
    }
    let mut slot: Slot<'_, Message> = picker.into();
    slot.inherit(&Inherited::default().with_disabled(disabled));
    slot.compose()
}
fn focus(ui: &mut Preview) -> focusable::Count {
    let mut op = focusable::count();
    ui.operate(&mut operation::black_box(&mut op));
    match op.finish() {
        Outcome::Some(count) => count,
        _ => panic!("focus count completes"),
    }
}
fn advance<'a, Message: 'a>(ui: &mut Simulator<'a, Message, Theme, iced::Renderer>, reverse: bool) {
    let mut op: Box<dyn Operation> = if reverse {
        Box::new(focusable::focus_previous::<()>())
    } else {
        Box::new(focusable::focus_next::<()>())
    };
    ui.operate(op.as_mut());
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
}

fn tab_to_day<'a, Message: 'a>(
    ui: &mut Simulator<'a, Message, Theme, iced::Renderer>,
    stops: usize,
) -> String {
    #[derive(Default)]
    struct Focused {
        bounds: Option<Rectangle>,
        labels: Vec<(Rectangle, String)>,
    }
    impl Operation for Focused {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
            if state.is_focused() {
                self.bounds = Some(bounds);
            }
        }
        fn text(&mut self, _: Option<&Id>, bounds: Rectangle, state: &mut dyn Text) {
            self.labels.push((bounds, state.text().into_owned()));
        }
    }
    ui.operate(&mut focusable::unfocus::<()>());
    for _ in 0..stops {
        advance(ui, false);
    }
    let mut focused = Focused::default();
    ui.operate(&mut focused);
    let bounds = focused.bounds.expect("the date grid has keyboard focus");
    focused
        .labels
        .into_iter()
        .find_map(|(label, text)| bounds.contains(label.center()).then_some(text))
        .expect("focused date has its visible numeral")
}
#[test]
fn date_grid_has_one_tab_stop_and_keyboard_skips_disabled_dates() {
    let mut ui = Simulator::new(picker(15, false, false));
    assert_eq!(
        focus(&mut ui).total,
        3,
        "two navigation controls and one day"
    );
    ui.click("15").unwrap();
    ui.drain().for_each(drop);
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [Message::Pick(Some(Selection::single(date(1, 17))))]
    );
    ui.tap_key(Named::Home);
    ui.tap_key(Named::Space);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [Message::Pick(Some(Selection::single(date(1, 11))))]
    );
    ui.tap_key(Named::End);
    ui.tap_key(Named::PageDown);
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [
            Message::Month(date(2, 1)),
            Message::Pick(Some(Selection::single(date(2, 17))))
        ]
    );
    ui.find("February 2026").unwrap();
    ui.click("15").unwrap();
    ui.drain().for_each(drop);
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::Space);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [Message::Pick(Some(Selection::single(date(2, 17))))],
        "returning to a previously focused date after a pointer selection still moves focus"
    );
    ui.tap_key(Named::PageUp);
    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::ArrowUp);
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Space);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [
            Message::Month(date(1, 1)),
            Message::Pick(Some(Selection::single(date(1, 15))))
        ],
        "reverse month/day/week navigation skips unavailable January 16"
    );
    advance(&mut ui, false);
    assert_eq!(focus(&mut ui).focused, None, "Tab leaves the day grid");
    advance(&mut ui, true);
    assert_eq!(
        focus(&mut ui).focused,
        Some(2),
        "reverse Tab re-enters the same day"
    );
}
#[test]
fn defaults_seed_once_controlled_values_remain_authoritative_and_disabled_ancestors_win() {
    let ui = Simulator::new(picker(15, false, false));
    let mut ui = ui.rebuild(picker(25, false, false));
    ui.click("15").unwrap();
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [Message::Pick(None)],
        "rebuild does not reseed selection"
    );
    ui = ui.rebuild(picker(15, true, false));
    ui.click("17").unwrap();
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [Message::Pick(Some(Selection::single(date(1, 17))))]
    );
    ui = ui.rebuild(picker(15, true, false));
    ui.click("15").unwrap();
    assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Pick(None)]);
    ui.click("16").unwrap();
    assert_eq!(ui.drain().count(), 0);
    ui = ui.rebuild(picker(15, true, true));
    assert_eq!(focus(&mut ui).total, 0);
    ui.click("17").unwrap();
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().count(), 0);
}
#[test]
fn caption_menus_preserve_the_grid_and_return_to_the_changed_month() {
    let view = || {
        Element::<Message, Theme>::from(calendar().default_month(date(1, 1)).caption_dropdown(true))
    };
    let mut ui = Simulator::new(view());
    ui.click("Jan").unwrap();
    ui.find("15")
        .expect("days stay mounted below the caption menu");
    ui.click("Mar").unwrap();
    ui = ui.rebuild(view());
    ui.find("Mar").expect("selected month reflected by trigger");
    ui.click("2026").unwrap();
    // The scrollable resolves its bottom anchor on the requested redraw.
    ui.draw(&Theme::light());
    ui.click("2025").unwrap();
    ui = ui.rebuild(view());
    ui.find("2025").unwrap();
    ui.find("15").unwrap();
}

#[test]
fn caption_controls_match_nova_bounds_and_apply_the_month_formatter_to_options() {
    #[derive(Default)]
    struct Controls(Vec<Rectangle>);
    impl Operation for Controls {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn Focusable) {
            self.0.push(bounds);
        }
    }
    for (long, january, march) in [(false, "Jan", "Mar"), (true, "January", "March")] {
        let view = || {
            Element::<Message, Theme>::from(
                calendar()
                    .default_month(date(1, 1))
                    .caption_dropdown(true)
                    .long_months(long),
            )
        };
        let mut ui = Simulator::new(view());
        let mut controls = Controls::default();
        ui.operate(&mut controls);
        assert_eq!(controls.0.len(), 5);
        for bounds in &controls.0[1..3] {
            assert_eq!(bounds.height, 26.);
            assert_eq!(bounds.center_y(), controls.0[0].center_y());
        }
        let month = ui.find(january).unwrap().bounds();
        assert!((month.x - controls.0[1].x - 7.).abs() < 0.1);
        ui.click(january).unwrap();
        ui.click(march).unwrap();
        ui = ui.rebuild(view());
        ui.find(march).unwrap();
        ui.find("15").expect("the changed date grid stays mounted");
    }
}

#[test]
fn caption_arrows_start_at_the_displayed_month_and_year() {
    let view = || {
        Element::<Message, Theme>::from(
            calendar().default_month(date(11, 1)).caption_dropdown(true),
        )
    };
    let mut ui = Simulator::new(view());
    for (label, expected) in [("Nov", "Oct"), ("2026", "2025"), ("2025", "2024")] {
        ui.click(label).unwrap();
        ui.draw(&Theme::light());
        ui.tap_key(Named::ArrowUp);
        ui.tap_key(Named::Enter);
        ui = ui.rebuild(view());
        ui.find(expected)
            .expect("ArrowUp selects the previous visible choice");
        assert!(
            ui.find("Mar").is_err(),
            "committing a choice dismisses the caption"
        );
    }
}

#[test]
fn caption_escape_restores_the_trigger_for_keyboard_reopening() {
    let view = || {
        Element::<Message, Theme>::from(calendar().default_month(date(1, 1)).caption_dropdown(true))
    };
    let mut ui = Simulator::new(view());
    ui.click("Jan").unwrap();
    ui.find("Mar").unwrap();
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(view());
    assert!(ui.find("Mar").is_err());
    assert_eq!(focus(&mut ui).focused, Some(1));
    ui.tap_key(Named::Enter);
    ui = ui.rebuild(view());
    ui.find("Mar").expect("focus returns to the month trigger");
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(view());
    ui.click("2026").unwrap();
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(view());
    assert_eq!(focus(&mut ui).focused, Some(2));
    ui.tap_key(Named::Space);
    ui = ui.rebuild(view());
    ui.draw(&Theme::light());
    ui.find("2025").expect("focus returns to the year trigger");
}

#[test]
fn demo_focuses_today_and_navigation_can_repeat_in_both_directions() {
    use icedcn::widget::calendar::{Datelike, today};
    let page = CalendarPage::new(0);
    let mut ui = Simulator::new(page.preview("demo"));
    assert_eq!(tab_to_day(&mut ui, 5), today().day().to_string());

    let mut ui = Simulator::new(picker(15, false, false));
    for (point, month, caption) in [
        (Point::new(190., 22.), 2, "February 2026"),
        (Point::new(190., 22.), 3, "March 2026"),
        (Point::new(22., 22.), 2, "February 2026"),
        (Point::new(22., 22.), 1, "January 2026"),
    ] {
        ui.point_at(point);
        ui.simulate(simulator::click());
        ui.find(caption).unwrap();
        assert_eq!(
            ui.drain().collect::<Vec<_>>(),
            [Message::Month(date(month, 1))]
        );
    }
}
#[test]
fn installed_range_uses_window_breakpoint_and_aligns_month_tops() {
    use icedcn::widget::calendar::{Datelike, today};
    for (window, content) in [(1440., 640.), (700., 640.), (767., 640.), (768., 640.)] {
        let mut page = CalendarPage::new(0);
        page.window_width(window);
        let mut ui = Simulator::with_size(
            Settings::default(),
            Size::new(content, 720.),
            page.preview("range"),
        );
        let january = ui
            .find(format!("January {}", today().year()))
            .unwrap()
            .bounds();
        let february = ui
            .find(format!("February {}", today().year()))
            .unwrap()
            .bounds();
        if window >= 768. {
            assert!((january.y - february.y).abs() < 0.1);
            assert!(february.x > january.x + 190.);
        } else {
            assert!((january.center_x() - february.center_x()).abs() < 0.1);
            assert!(february.y > january.y + 200.);
        }
    }
}
#[test]
fn installed_time_constraints_presets_and_persian_grid() {
    let page = CalendarPage::new(0);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(375., 720.),
        page.preview("time"),
    );
    let start = ui.find("Start Time").unwrap().bounds();
    let end = ui.find("End Time").unwrap().bounds();
    assert!(end.y > start.y + 60.);
    ui.click("10:30:00").unwrap();
    ui.tap_key(Named::End);
    ui.tap_key(Named::Backspace);
    ui.typewrite("5");
    ui.find("10:30:05").unwrap();
    ui.find("12:30:00").unwrap();
    ui.typewrite("9");
    ui.find("10:30:05").expect("extra time digit rejected");
    for _ in 0..8 {
        ui.tap_key(Named::Backspace);
    }
    ui.typewrite("23:59:59");
    ui.find("23:59:59")
        .expect("last second of the day is valid");
    ui.tap_key(Named::Backspace);
    ui.tap_key(Named::Backspace);
    ui.typewrite("60");
    ui.find("23:59:6").expect("sixty seconds is rejected");
    ui.find("12:30:00").expect("End Time stays unchanged");
    let settings = Settings {
        font: "Geist".into(),
        fonts: vec![Cow::Borrowed(persian::FONT)],
        ..Default::default()
    };
    let mut ui = Simulator::with_size(settings, Size::new(375., 720.), page.preview("hijri"));
    ui.find("خرداد ۱۴۰۴").unwrap();
    let sat = ui.find("ش").unwrap().bounds();
    let fri = ui.find("ج").unwrap().bounds();
    assert!(sat.x > fri.x, "Persian Saturday is on the right");
    for id in [
        "demo",
        "basic",
        "caption",
        "presets",
        "booked-dates",
        "custom-days",
        "week-numbers",
    ] {
        let mut ui =
            Simulator::with_size(Settings::default(), Size::new(640., 720.), page.preview(id));
        ui.find(if id == "presets" { "In 2 weeks" } else { "15" })
            .unwrap();
    }
}

#[test]
fn time_fields_validate_each_clock_boundary_and_accept_empty_edits() {
    for (typed, accepted) in [
        ("00:00:00", "00:00:00"),
        ("23:59:59", "23:59:59"),
        ("24", "2"),
        ("09:60", "09:6"),
        ("09:59:60", "09:59:6"),
        ("12:34:56:7", "12:34:56"),
        ("12:34x:56", "12:34:56"),
    ] {
        let mut ui = Simulator::new(recipes::time::<Message>(true));
        ui.click("10:30:00").unwrap();
        ui.tap_key(Named::End);
        for _ in 0..8 {
            ui.tap_key(Named::Backspace);
        }
        ui.find("HH:mm:ss").expect("the start time can be cleared");
        ui.typewrite(typed);
        ui.find(accepted).unwrap();
        ui.find("12:30:00").expect("End Time stays independent");
    }
}

#[test]
fn nested_calendar_caption_can_change_month_and_single_date_picker_still_closes() {
    use icedcn::widget::calendar::today;
    use icedcn::widget::date_picker;
    let view = || {
        Element::<Message, Theme>::from(
            date_picker()
                .caption_dropdown(true)
                .on_value_change(Message::Pick),
        )
    };
    let mut ui = Simulator::new(view());
    ui.click("Pick a date").unwrap();
    ui.click(today().format("%b").to_string()).unwrap();
    ui.click("Mar")
        .expect("nested month dropdown is actionable");
    ui = ui.rebuild(view());
    ui.find("Mar").unwrap();
    ui.click("15").unwrap();
    ui = ui.rebuild(view());
    assert!(
        ui.find("15").is_err(),
        "single selection closes the calendar"
    );
    assert!(matches!(ui.drain().last(), Some(Message::Pick(Some(_)))));
}

#[test]
fn all_preset_actions_update_both_selection_and_visible_month() {
    use icedcn::widget::calendar::{Datelike, Duration, today};
    let page = CalendarPage::new(0);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(640., 720.),
        page.preview("presets"),
    );
    for (label, offset) in [
        ("Today", 0),
        ("Tomorrow", 1),
        ("In 3 days", 3),
        ("In a week", 7),
        ("In 2 weeks", 14),
    ] {
        ui.click(label).unwrap();
        let day = today() + Duration::days(offset);
        ui.find(day.format("%B %Y").to_string())
            .expect("preset changes visible month");
        assert_eq!(tab_to_day(&mut ui, 3), day.day().to_string());
    }
}

#[test]
fn booked_dates_ignore_unavailable_days_and_select_available_days() {
    let page = CalendarPage::new(0);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(640., 720.),
        page.preview("booked-dates"),
    );
    assert_eq!(tab_to_day(&mut ui, 3), "3");
    ui.click("12").unwrap();
    assert_eq!(tab_to_day(&mut ui, 3), "3", "booked day is ignored");
    ui.click("11").unwrap();
    assert_eq!(tab_to_day(&mut ui, 3), "11");
}

#[test]
fn controlled_month_rebuild_restores_the_keyboard_destination() {
    let view = |month| {
        Element::<Message, Theme>::from(
            calendar()
                .month(month)
                .default_selected(Selection::single(date(1, 31)))
                .on_select(Message::Pick)
                .on_month_change(Message::Month),
        )
    };
    let mut ui = Simulator::new(view(date(1, 1)));
    advance(&mut ui, false);
    advance(&mut ui, false);
    advance(&mut ui, false);
    ui.tap_key(Named::PageDown);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Month(date(2, 1))]);
    ui = ui.rebuild(view(date(2, 1)));
    ui.tap_key(Named::Space);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [Message::Pick(Some(Selection::single(date(2, 28))))]
    );
}

#[test]
fn constrained_custom_cells_keep_their_requested_width() {
    for (window, width, cell) in [(700., 260., 40.), (1440., 260., 48.), (1440., 560., 48.)] {
        let mut page = CalendarPage::new(0);
        page.window_width(window);
        let mut ui = Simulator::with_size(
            Settings::default(),
            Size::new(width, 720.),
            page.preview("custom-days"),
        );
        let a = ui.find("15").unwrap().bounds();
        let b = ui.find("16").unwrap().bounds();
        assert!(
            (b.center_x() - a.center_x() - cell).abs() < 0.1,
            "window={window}, content={width}: {a:?} {b:?}"
        );
    }
}

#[test]
fn installed_range_rejects_future_and_pre_1900_days() {
    use icedcn::widget::calendar::{Datelike, today};
    #[derive(Default)]
    struct Controls(Vec<Rectangle>);
    impl Operation for Controls {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn Focusable) {
            self.0.push(bounds);
        }
    }
    let mut ui = Simulator::new(recipes::range::<Message>(Some(true)));
    let mut controls = Controls::default();
    ui.operate(&mut controls);
    let previous = controls.0[0].center();
    let next = controls
        .0
        .iter()
        .find(|bounds| bounds.y == controls.0[0].y && bounds.x > controls.0[0].x)
        .unwrap()
        .center();
    let historic = (today().year() - 1900) * 12 + 2;
    for (direction, steps) in [(previous, historic), (next, historic)] {
        for _ in 0..steps {
            ui.point_at(direction);
            ui.simulate(simulator::click());
        }
        if direction == previous {
            ui.find("November 1899").unwrap();
            ui.find("December 1899").unwrap();
            assert_eq!(focus(&mut ui).total, 2, "only navigation is enabled");
            ui.click("15").unwrap();
        }
    }
    assert_eq!(
        tab_to_day(&mut ui, 2),
        "12",
        "old dates leave the range intact"
    );
    for _ in 0..12 {
        ui.point_at(next);
        ui.simulate(simulator::click());
    }
    ui.find(format!("January {}", today().year() + 1)).unwrap();
    assert_eq!(focus(&mut ui).total, 2, "future days are disabled");
    ui.click("15").unwrap();
    for _ in 0..12 {
        ui.point_at(previous);
        ui.simulate(simulator::click());
    }
    assert_eq!(
        tab_to_day(&mut ui, 2),
        "12",
        "future dates leave the range intact"
    );
}
