//! Responsive alert-dialog layout retains its modal keyboard order.
use browser::Message;
use browser::component::alert_dialog::AlertDialogPage;
use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::{Event, Size, window};
use iced_test::{Simulator, selector};
use icedxdy::Theme;

#[test]
fn resizing_an_open_dialog_preserves_cancel_first_and_reverses_only_mobile_layout() {
    let page = AlertDialogPage::new(0);
    let theme = Theme::light();
    let mut ui: Simulator<'_, Message, Theme, iced::Renderer> = Simulator::with_size(
        Default::default(),
        Size::new(1440., 900.),
        page.preview("demo"),
    );
    ui.draw(&theme);
    ui.click("Show Dialog").expect("mounted trigger");
    ui = ui.rebuild(page.preview("demo"));
    for width in [1440., 639., 640., 390., 1440.] {
        ui = ui.resize(Size::new(width, 900.));
        ui.draw(&theme);
        ui.simulate([Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )]);
        ui = ui.rebuild(page.preview("demo"));
        ui.draw(&theme);
        let cancel = ui.find("Cancel").expect("mounted cancel").bounds();
        let action = ui.find("Continue").expect("mounted action").bounds();
        let focused = ui
            .find(selector::is_focused())
            .expect("modal focus")
            .bounds();
        assert!(focused.contains(cancel.center()));
        assert_eq!(cancel.y > action.y, width < 640.);
        ui.tap_key(Named::Tab);
        assert!(
            ui.find(selector::is_focused())
                .expect("action focus")
                .bounds()
                .contains(action.center())
        );
        ui.tap_key(Named::Tab);
    }
}
