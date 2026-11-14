//! Responsive alert-dialog layout retains its modal keyboard order.
use browser::Message;
use browser::component::alert_dialog::AlertDialogPage;
use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::{Event, Size, window};
use iced_test::{Simulator, selector};
use icedxdy::Theme;
use std::time::Duration;

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

#[test]
fn every_recipe_retains_keyboard_dismissal_and_trigger_reactivation() {
    let page = AlertDialogPage::new(0);
    let theme = Theme::light();
    for (id, trigger, cancel, action) in [
        ("demo", "Show Dialog", "Cancel", "Continue"),
        ("basic", "Show Dialog", "Cancel", "Continue"),
        ("small", "Show Dialog", "Don't allow", "Allow"),
        ("media", "Share Project", "Cancel", "Share"),
        ("small-media", "Show Dialog", "Don't allow", "Allow"),
        ("destructive", "Delete Chat", "Cancel", "Delete"),
    ] {
        let mut ui: Simulator<'_, Message, Theme, iced::Renderer> =
            Simulator::with_size(Default::default(), Size::new(1440., 900.), page.preview(id));
        ui.draw(&theme);
        ui.click(trigger).expect("mounted trigger");
        ui = ui.rebuild(page.preview(id));
        ui.draw(&theme);
        for reopen in [Named::Enter, Named::Space] {
            let cancel = ui.find(cancel).expect("cancel action").bounds();
            let focused = ui.find(selector::is_focused()).expect("initial focus");
            assert!(focused.bounds().contains(cancel.center()), "{id}");
            ui.tap_key(Named::Tab);
            let action = ui.find(action).expect("confirm action").bounds();
            let focused = ui.find(selector::is_focused()).expect("action focus");
            assert!(focused.bounds().contains(action.center()), "{id}");

            ui.tap_key(Named::Escape);
            ui = ui.rebuild(page.preview(id));
            // Let the real exit animation release the modal focus scope.
            std::thread::sleep(Duration::from_millis(180));
            ui.simulate([Event::Window(
                window::Event::RedrawRequested(Instant::now()),
            )]);
            ui = ui.rebuild(page.preview(id));
            ui.draw(&theme);
            let trigger = ui.find(trigger).expect("restored trigger").bounds();
            let focused = ui.find(selector::is_focused()).expect("restored focus");
            assert!(focused.bounds().contains(trigger.center()), "{id}");

            ui.tap_key(reopen);
            ui = ui.rebuild(page.preview(id));
            ui.draw(&theme);
        }
    }
}
