//! Browser actions available from any unclaimed right click.

use iced::Fill;
use icedcn::View;
use icedcn::widget::{column, menu, row, text};

use crate::{icon, page};

use super::route::Route;
use super::{App, Message};

pub(super) fn view(app: &App) -> View<'_, Message> {
    let source = source(app);
    let title = match app.route() {
        Route::Home => "Home",
        Route::Create => "Create",
        Route::NotFound(_) => "Page not found",
        Route::Examples | Route::Receipts | Route::Playground => "Examples",
        Route::Page(index) => app.pages[index].meta().title,
    };
    let (theme_icon, theme_label) = if app.is_dark() {
        (icon::sun(), "Use light theme")
    } else {
        (icon::moon(), "Use dark theme")
    };

    column![
        menu::label(title),
        menu::item(
            row![
                icon::search().size(16),
                text("Search documentation").width(Fill)
            ]
            .spacing(8)
            .width(Fill)
        )
        .shortcut("/")
        .on_select(Message::Search(true)),
        menu::separator(),
        menu::item(
            row![
                icon::book_open().size(16),
                text("Open documentation").width(Fill)
            ]
            .spacing(8)
            .width(Fill)
        )
        .on_select(Message::OpenUrl("icedcn://guide/introduction")),
        menu::item(
            row![
                icon::panels_top_left().size(16),
                text("Browse examples").width(Fill)
            ]
            .spacing(8)
            .width(Fill)
        )
        .on_select(Message::OpenUrl("icedcn://examples")),
        menu::separator(),
        menu::item(
            row![theme_icon.size(16), text(theme_label).width(Fill)]
                .spacing(8)
                .width(Fill)
        )
        .on_select(Message::ToggleTheme),
        menu::separator(),
        menu::item(
            row![
                icon::file_code().size(16),
                text("View source on GitHub").width(Fill)
            ]
            .spacing(8)
            .width(Fill)
        )
        .on_select(Message::Page(
            app.current_page,
            page::Message::LinkClicked(source.clone().into()),
        )),
        menu::item(
            row![icon::copy().size(16), text("Copy source link").width(Fill)]
                .spacing(8)
                .width(Fill)
        )
        .on_select(Message::CopyText(source)),
    ]
    .width(Fill)
    .into()
}

fn source(app: &App) -> String {
    if app.destination == Route::Home {
        "https://github.com/airstrike/icedcn".into()
    } else if app.destination == Route::Create {
        "https://github.com/airstrike/icedcn/tree/HEAD/examples/palette".into()
    } else if app.showing_examples() {
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
        let _ = app.update(Message::SelectPage(0));
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
        let _ = app.update(Message::SelectPage(app.current_page));
        let mut ui = ui(&app);
        let bounds = ui.find("Right click here").unwrap();
        right_click(&mut ui, bounds.bounds().center());
        assert!(ui.find("Show Bookmarks").is_ok());
        assert!(ui.find("Copy source link").is_err());
    }

    #[test]
    fn menu_labels_receive_full_width_without_wrapping() {
        let app = app();
        let mut ui = ui(&app);
        right_click(&mut ui, Point::new(1400.0, 700.0));
        for label in [
            "Search documentation",
            "Open documentation",
            "Browse examples",
            if app.is_dark() {
                "Use light theme"
            } else {
                "Use dark theme"
            },
            "View source on GitHub",
            "Copy source link",
        ] {
            let bounds = ui.find(label).unwrap().bounds();
            assert!(bounds.height <= 21.0, "{label}: {bounds:?}");
            assert!(bounds.width >= 160.0, "{label}: {bounds:?}");
        }
    }

    #[test]
    fn copy_source_link_targets_the_current_registered_recipe() {
        let mut app = app();
        app.current_page = app
            .pages
            .iter()
            .position(|page| page.meta().title == "Chart")
            .unwrap();
        let _ = app.update(Message::SelectPage(app.current_page));
        let mut ui = ui(&app);
        right_click(&mut ui, Point::new(1400.0, 700.0));
        ui.click("Copy source link").unwrap();
        assert!(ui.drain().any(|message| matches!(
            message,
            Message::CopyText(url) if url.ends_with("/examples/browser/snippets/chart-demo.md")
        )));
    }
}
