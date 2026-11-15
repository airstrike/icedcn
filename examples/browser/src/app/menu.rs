//! Browser actions available from any unclaimed right click.

use icedcn::Slot;
use icedcn::widget::{column, menu, row};

use crate::{icon, page};

use super::{App, Message};

pub(super) fn view(app: &App) -> Slot<'_, Message> {
    let source = source(app);
    let title = if app.showing_examples {
        "Examples"
    } else {
        app.pages[app.current_page].meta().title
    };
    let (theme_icon, theme_label) = if app.dark_mode {
        (icon::sun(), "Use light theme")
    } else {
        (icon::moon(), "Use dark theme")
    };

    column![
        menu::label(title),
        menu::item(row![icon::search().size(16), "Search documentation"].spacing(8))
            .shortcut("/")
            .on_select(Message::Search(true)),
        menu::separator(),
        menu::item(row![icon::book_open().size(16), "Open documentation"].spacing(8))
            .on_select(Message::OpenUrl("icedcn://guide/introduction")),
        menu::item(row![icon::panels_top_left().size(16), "Browse examples"].spacing(8))
            .on_select(Message::OpenUrl("icedcn://examples")),
        menu::separator(),
        menu::item(row![theme_icon.size(16), theme_label].spacing(8))
            .on_select(Message::ToggleTheme),
        menu::separator(),
        menu::item(row![icon::file_code().size(16), "View source on GitHub"].spacing(8)).on_select(
            Message::Page(
                app.current_page,
                page::Message::LinkClicked(source.clone().into()),
            )
        ),
        menu::item(row![icon::copy().size(16), "Copy source link"].spacing(8))
            .on_select(Message::CopyText(source)),
    ]
    .into()
}

fn source(app: &App) -> String {
    if app.showing_examples {
        "https://github.com/airstrike/icedcn/tree/HEAD/examples/browser/src/examples".into()
    } else if let Some(path) = first_source(app.pages[app.current_page].blocks()) {
        format!("https://github.com/airstrike/icedcn/blob/HEAD/{path}")
    } else {
        "https://github.com/airstrike/icedcn/tree/HEAD/examples/browser/src/component".into()
    }
}

fn first_source(blocks: &[page::Block]) -> Option<&'static str> {
    blocks.iter().find_map(|block| match block {
        page::Block::Md(path) | page::Block::Example { snippet: path, .. } => {
            (!path.is_empty()).then_some(*path)
        }
        page::Block::Group { children, .. } => first_source(children),
        page::Block::Inline { .. } => None,
    })
}

#[cfg(test)]
mod tests {
    use super::{App, Message};
    use iced::{Event, Point, Size, mouse};
    use iced_test::core::Settings;
    use iced_test::simulator::Simulator;
    use icedcn::Theme;
    use icedcn::widget::markdown::highlighter::Map;

    fn app() -> App {
        let mut app = App::new(Map::from(&Theme::light()), Map::from(&Theme::dark())).0;
        app.width = 1440.0;
        app
    }

    fn ui(app: &App) -> Simulator<'_, Message, Theme, iced::Renderer> {
        Simulator::with_size(Settings::default(), Size::new(1440.0, 900.0), app.view())
    }

    fn right_click(ui: &mut Simulator<'_, Message, Theme, iced::Renderer>, point: Point) {
        ui.point_at(point);
        let _ = ui.simulate([
            Event::Mouse(mouse::Event::CursorMoved { position: point }),
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)),
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)),
        ]);
    }

    #[test]
    fn blank_space_menu_routes_existing_browser_actions() {
        let app = app();
        let mut ui = ui(&app);
        right_click(&mut ui, Point::new(1400.0, 700.0));
        ui.click("Browse examples").unwrap();
        assert!(
            ui.drain()
                .any(|message| matches!(message, Message::OpenUrl("icedcn://examples")))
        );
        let mut ui = ui.rebuild(app.view());
        assert!(ui.find("Copy source link").is_err());

        right_click(&mut ui, Point::new(1400.0, 700.0));
        ui.click("Search documentation").unwrap();
        assert!(
            ui.drain()
                .any(|message| matches!(message, Message::Search(true)))
        );
    }

    #[test]
    fn component_context_menu_wins_over_browser_menu() {
        let mut app = app();
        app.current_page = app
            .pages
            .iter()
            .position(|page| page.meta().title == "Context Menu")
            .unwrap();
        let mut ui = ui(&app);
        let bounds = ui.find("Right click here").unwrap();
        right_click(&mut ui, bounds.bounds().center());
        assert!(ui.find("Show Bookmarks").is_ok());
        assert!(ui.find("Copy source link").is_err());
    }

    #[test]
    fn copy_source_link_targets_the_current_registered_recipe() {
        let mut app = app();
        app.current_page = app
            .pages
            .iter()
            .position(|page| page.meta().title == "Chart")
            .unwrap();
        let mut ui = ui(&app);
        right_click(&mut ui, Point::new(1400.0, 700.0));
        ui.click("Copy source link").unwrap();
        assert!(ui.drain().any(|message| matches!(
            message,
            Message::CopyText(url) if url.ends_with("/examples/browser/snippets/chart-demo.md")
        )));
    }
}
