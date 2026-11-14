//! The Table recipes expose real horizontally scrollable columns on narrow screens.
use browser::Message;
use browser::component::table::TablePage;
use iced::keyboard::key::Named;
use iced::mouse::ScrollDelta;
use iced::{Font, Point, Size};
use iced_test::core::Settings;
use iced_test::core::widget::operation::{Operation, Outcome, focusable};
use iced_test::{Simulator, simulator};
use iced_widget::graphics::text::font_system;
use icedcn::Theme;
use std::{borrow::Cow, sync::Once};

fn settings() -> Settings {
    static FONT: Once = Once::new();
    FONT.call_once(|| {
        font_system()
            .write()
            .unwrap()
            .load_font(Cow::Borrowed(include_bytes!("../fonts/Geist.ttf")));
    });
    Settings {
        font: Font::new("Geist"),
        text_size: 14.into(),
        ..Default::default()
    }
}

#[test]
fn invoice_amounts_scroll_into_view_without_wrapping_methods() {
    let page = TablePage::new(0);
    for (id, last) in [("demo", "$300.00"), ("footer", "$350.00")] {
        let mut ui = Simulator::with_size(settings(), Size::new(260., 480.), page.preview(id));
        let method = ui.find("Bank Transfer").unwrap().bounds();
        let amount = ui.find(last).unwrap().bounds();
        assert!(method.height <= 21., "{method:?}");
        assert!(amount.x + amount.width > 260., "{amount:?}");
        ui.point_at(method.center());
        ui.scroll(ScrollDelta::Pixels { x: -500., y: 0. });
        ui.draw(&Theme::light());
        let end = ui.find(last).unwrap().visible_bounds().unwrap();
        assert!((end.width - amount.width).abs() <= 1., "{end:?} {amount:?}");
        assert!(end.x + end.width <= 260.);
        assert!(ui.find("$2,500.00").unwrap().visible_bounds().is_some());
    }
}

#[test]
fn product_menu_opens_in_the_scrolled_last_column() {
    let page = TablePage::new(0);
    let mut ui: Simulator<'_, Message, _, _> =
        Simulator::with_size(settings(), Size::new(260., 300.), page.preview("actions"));
    ui.draw(&Theme::light());
    let product = ui.find("Mechanical Keyboard").unwrap().bounds();
    assert!(product.height <= 21., "{product:?}");
    ui.point_at(product.center());
    ui.scroll(ScrollDelta::Pixels { x: -500., y: 0. });
    ui.draw(&Theme::light());
    ui.point_at(Point::new(236., product.center_y()));
    ui.simulate(simulator::click());
    ui.draw(&Theme::light());
    ui.find("Duplicate").unwrap();
    ui.click("Delete").unwrap();
    ui = ui.rebuild(page.preview("actions"));
    ui.draw(&Theme::light());
    assert!(ui.find("Duplicate").is_err());
}

#[test]
fn focused_narrow_table_reaches_the_final_column_with_end() {
    let page = TablePage::new(0);
    let mut ui = Simulator::with_size(settings(), Size::new(260., 480.), page.preview("demo"));
    let mut focus = focusable::focus_next::<()>();
    ui.operate(&mut focus);
    if let Outcome::Chain(mut step) = focus.finish() {
        ui.operate(step.as_mut());
    }
    ui.simulate([simulator::press_key(Named::End, None)]);
    ui.draw(&Theme::dark());
    let amount = ui.find("$2,500.00").unwrap();
    assert!((amount.visible_bounds().unwrap().width - amount.bounds().width).abs() < 0.01);
    ui.simulate([simulator::press_key(Named::Home, None)]);
    ui.draw(&Theme::dark());
    assert!(ui.find("INV001").unwrap().visible_bounds().unwrap().width > 20.);
}
