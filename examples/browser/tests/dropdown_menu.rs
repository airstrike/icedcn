//! Navigation of the installed Dropdown Menu recipes.
use browser::Message;
use browser::component::dropdown_menu::DropdownMenuPage;
use iced::Rectangle;
use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::{Event, Size, mouse, window};
use iced_test::Simulator;
use iced_widget::core::widget::{Id, Operation, operation::Focusable};
use icedcn::Theme;
use std::time::Duration;

type Ui<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

fn frame(ui: &mut Ui<'_>) {
    ui.draw(&Theme::light());
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    ui.draw(&Theme::light());
}

fn hover(ui: &mut Ui<'_>, label: &'static str) {
    let position = ui.find(label).unwrap().bounds().center();
    ui.point_at(position);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position })]);
    frame(ui);
}

#[derive(Default)]
struct Focus(Vec<Rectangle>);
impl Operation for Focus {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, control: &mut dyn Focusable) {
        if control.is_focused() {
            self.0.push(bounds);
        }
    }
}
fn focused(ui: &mut Ui<'_>, label: &'static str) -> bool {
    let center = ui.find(label).unwrap().bounds().center();
    let mut focus = Focus::default();
    ui.operate(&mut focus);
    focus.0.iter().any(|bounds| bounds.contains(center))
}

#[test]
fn pointer_open_traversal_typeahead_and_shortcuts() {
    let page = DropdownMenuPage::new(0);
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(390., 844.),
        page.preview("demo"),
    );
    frame(&mut ui);
    ui.click("Open").unwrap();
    frame(&mut ui);
    assert!(!focused(&mut ui, "Profile"));
    ui.tap_key(Named::ArrowDown);
    assert!(focused(&mut ui, "Profile"));
    ui.tap_key(Named::ArrowUp);
    assert!(focused(&mut ui, "Profile"));
    ui.tap_key(Named::End);
    assert!(focused(&mut ui, "Log out"));
    ui.tap_key(Named::ArrowUp);
    assert!(focused(&mut ui, "Support"));
    ui.tap_key(Named::Home);
    ui.typewrite("bi");
    assert!(focused(&mut ui, "Billing"));
    let hint = ui.find("⌘B").unwrap().bounds();
    assert!(hint.x > 120., "{hint:?}");
    assert_eq!(
        {
            let mut f = Focus::default();
            ui.operate(&mut f);
            f.0.last().unwrap().height
        },
        28.
    );
}

#[test]
fn pointer_can_travel_from_parent_to_grandchild_and_select() {
    let page = DropdownMenuPage::new(0);
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(520., 480.),
        page.preview("submenu"),
    );
    frame(&mut ui);
    ui.click("Open").unwrap();
    frame(&mut ui);
    hover(&mut ui, "Invite users");
    hover(&mut ui, "More options");
    hover(&mut ui, "Slack");
    // Let the parent's pointer-leave interval elapse while the grandchild owns the cursor.
    std::thread::sleep(Duration::from_millis(170));
    frame(&mut ui);
    ui.click("Slack").unwrap();
    frame(&mut ui);
    ui = ui.rebuild(page.preview("submenu"));
    frame(&mut ui);
    assert!(ui.find("Team").is_err());
}

#[test]
fn keyboard_can_enter_and_leave_two_levels() {
    let page = DropdownMenuPage::new(0);
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(520., 480.),
        page.preview("submenu"),
    );
    frame(&mut ui);
    ui.click("Open").unwrap();
    frame(&mut ui);
    ui.typewrite("i");
    ui.tap_key(Named::ArrowRight);
    frame(&mut ui);
    ui.typewrite("more");
    ui.tap_key(Named::ArrowRight);
    frame(&mut ui);
    ui.find("Slack").unwrap();
    ui.tap_key(Named::ArrowLeft);
    frame(&mut ui);
    assert!(ui.find("Slack").is_err());
    ui.find("More options").unwrap();
    ui.tap_key(Named::ArrowLeft);
    frame(&mut ui);
    assert!(ui.find("More options").is_err());
    ui.tap_key(Named::Escape);
    frame(&mut ui);
    ui = ui.rebuild(page.preview("submenu"));
    frame(&mut ui);
    assert!(ui.find("Team").is_err());
}

#[test]
fn short_windows_scroll_the_complex_menu_to_its_last_action() {
    use iced::Vector;
    use iced_widget::core::widget::operation::Scrollable;
    struct Offset(f32);
    impl Operation for Offset {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn scrollable(
            &mut self,
            _: Option<&Id>,
            _: Rectangle,
            _: Size,
            offset: Vector,
            _: &mut dyn Scrollable,
        ) {
            self.0 = offset.y;
        }
    }
    let page = DropdownMenuPage::new(0);
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(390., 240.),
        page.preview("complex"),
    );
    frame(&mut ui);
    ui.click("Complex Menu").unwrap();
    frame(&mut ui);
    let mut offset = Offset(0.);
    ui.operate(&mut offset);
    assert_eq!(offset.0, 0.);
    ui.tap_key(Named::End);
    frame(&mut ui);
    ui.operate(&mut offset);
    assert!(offset.0 > 200., "{}", offset.0);
    assert!(focused(&mut ui, "Sign Out"));
    ui.tap_key(Named::Home);
    frame(&mut ui);
    ui.operate(&mut offset);
    assert!(offset.0 < 25., "{}", offset.0);
}
