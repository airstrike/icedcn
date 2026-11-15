//! Mounted Sidebar recipes through the same public page used by the component browser.
use browser::Message;
use browser::component::sidebar::SidebarPage;
use iced::keyboard::{self, Modifiers, key::Named};
use iced::time::{Duration, Instant};
use iced::{Event, Size, window};
use iced_test::{Simulator, simulator};
use icedcn::Theme;

type Ui<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

fn frame(ui: &mut Ui<'_>) {
    let start = Instant::now();
    for ms in [0, 300] {
        ui.draw(&Theme::light());
        ui.simulate([Event::Window(window::Event::RedrawRequested(
            start + Duration::from_millis(ms),
        ))]);
    }
}

#[test]
fn installed_sidebar_fills_its_frame_and_changes_workspace() {
    let page = SidebarPage::new(0);
    let theme = Theme::light();
    let build = || page.docs_preview("demo", &theme);
    let mut ui = Ui::with_size(Default::default(), Size::new(960., 800.), build());
    frame(&mut ui);
    let header = ui.find("Acme Inc").unwrap().bounds();
    let footer = ui.find("Alex Chen").unwrap().bounds();
    assert!(header.y < 40.);
    assert!(footer.y > 336. && footer.y < 398., "{footer:?}");
    ui.click("Acme Inc").unwrap();
    frame(&mut ui);
    ui.click("Acme Corp.").unwrap();
    ui = ui.rebuild(build());
    frame(&mut ui);
    ui.find("Startup").unwrap();
    ui.click("Playground").unwrap();
    ui = ui.rebuild(build());
    frame(&mut ui);
    assert!(ui.find("History").is_err());
    ui.click("Models").unwrap();
    frame(&mut ui);
    ui.find("Genesis").unwrap();

    let mut shortcut = simulator::press_key(keyboard::Key::Character("b".into()), None);
    if let Event::Keyboard(keyboard::Event::KeyPressed { modifiers, .. }) = &mut shortcut {
        *modifiers = Modifiers::COMMAND;
    }
    ui.simulate([shortcut]);
    ui = ui.rebuild(build());
    frame(&mut ui);
    assert!(ui.find("Startup").is_err());
    ui.point_at([52., 30.]);
    ui.simulate(simulator::click());
    ui = ui.rebuild(build());
    frame(&mut ui);
    ui.find("Startup").unwrap();
    ui.find("Genesis").unwrap();
    assert!(ui.find("History").is_err());
}

#[test]
fn mobile_workspace_menu_opens_in_the_sheet_and_escape_returns_to_the_trigger() {
    let mut page = SidebarPage::new(0);
    page.window_width(390.);
    let theme = Theme::light();
    let build = || page.docs_preview("demo", &theme);
    let mut ui = Ui::with_size(Default::default(), Size::new(390., 844.), build());
    frame(&mut ui);
    ui = ui.rebuild(build());
    frame(&mut ui);
    assert!(ui.find("Acme Inc").is_err());
    ui.point_at([26., 32.]);
    ui.simulate(simulator::click());
    frame(&mut ui);
    ui = ui.rebuild(build());
    frame(&mut ui);
    let trigger = ui.find("Acme Inc").unwrap().bounds();
    ui.click("Acme Inc").unwrap();
    frame(&mut ui);
    let teams = ui.find("Teams").unwrap().bounds();
    assert!(teams.y > trigger.y + trigger.height);
    ui.click("Evil Corp.").unwrap();
    ui = ui.rebuild(build());
    frame(&mut ui);
    ui.find("Free").unwrap();
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(build());
    frame(&mut ui);
    assert!(ui.find("Free").is_err());
}

#[test]
fn guide_header_opens_a_menu_and_help_group_folds() {
    let page = SidebarPage::new(0);
    let mut header = Ui::with_size(
        Default::default(),
        Size::new(920., 600.),
        page.preview("header"),
    );
    frame(&mut header);
    header.click("Select Workspace").unwrap();
    frame(&mut header);
    header.find("Acme Inc").unwrap();
    let mut group = Ui::with_size(
        Default::default(),
        Size::new(920., 600.),
        page.preview("group"),
    );
    frame(&mut group);
    group.find("Feedback").unwrap();
    group.click("Help").unwrap();
    group = group.rebuild(page.preview("group"));
    frame(&mut group);
    assert!(group.find("Feedback").is_err());
}

#[test]
fn desktop_docs_sidebar_preserves_the_article_column() {
    use browser::app::App;
    use browser::component::radio_group::META;
    use icedcn::widget::markdown::highlighter::Map;

    let theme = Theme::light();
    let mut app = App::new(Map::from(&theme), Map::from(&theme)).0;
    let _ = app.update(Message::OpenComponent("radio-group".into()));
    let mut ui = Ui::with_size(Default::default(), Size::new(1440., 932.), app.view());
    let article = ui.find(META.description).unwrap().bounds();
    assert!((article.x - 412.5).abs() < 1., "{article:?}");
}
