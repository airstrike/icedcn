use super::{App, Message};
mod search;
use crate::{icon, page};
use iced::Widget;
use iced::keyboard::key::Named;
use iced::keyboard::{self, Key};
use iced::widget::button::{Status, StyleFn};
use iced::widget::{Space, combo_box, stack, svg};
use iced::{Center, Element, Event, Fill, Subscription, event};
use icedcn::theme::button::ghost;
use icedcn::widget::Row;
use icedcn::widget::{
    button, column, command, container, dialog, dropdown_menu, menu, row, rule, space, text,
    tooltip,
};
use icedcn::{Slot, Theme, radius};
use serde::Deserialize;
use std::time::Duration;

const GITHUB: &str = "https://github.com/airstrike/icedcn";

const NAVIGATION: &[(&str, &str)] = &[
    ("Home", GITHUB),
    ("Docs", "icedcn://component/button#installation"),
    ("Components", "icedcn://component/accordion"),
    ("Examples", "https://github.com/airstrike/icedcn/tree/HEAD/examples"),
    ("Charts", "icedcn://component/chart"),
    ("Typography", "icedcn://component/typography"),
];

pub(super) const GUIDES: &[(&str, &str)] = &[
    ("Introduction", GITHUB),
    ("Components", "icedcn://component/accordion"),
    ("Conversations", "icedcn://component/message-scroller"),
    ("Installation", "icedcn://component/button#installation"),
    ("Theming", "https://github.com/airstrike/icedcn/tree/HEAD/core/src/theme"),
    ("Cargo", "https://doc.rust-lang.org/cargo/"),
    ("Typography", "icedcn://component/typography"),
    ("Changelog", "https://github.com/airstrike/icedcn/commits/HEAD"),
];

pub(super) async fn stars() -> Option<u64> {
    #[derive(Deserialize)]
    struct Repository {
        stargazers_count: u64,
    }
    let response = reqwest::Client::new()
        .get("https://api.github.com/repos/airstrike/icedcn")
        .header("User-Agent", "icedcn-component-browser")
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .bytes()
        .await
        .ok()?;
    serde_json::from_slice::<Repository>(&response)
        .ok()
        .map(|repo| repo.stargazers_count)
}

pub(super) fn shortcuts() -> Subscription<Message> {
    event::listen_with(|event, status, _| {
        let Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            modifiers,
            repeat: false,
            ..
        }) = event
        else {
            return None;
        };
        if status == event::Status::Captured {
            return None;
        }
        match key.as_ref() {
            Key::Named(Named::Tab) if modifiers.shift() => Some(Message::FocusPrevious),
            Key::Named(Named::Tab) => Some(Message::FocusNext),
            Key::Character("a") if modifiers == keyboard::Modifiers::COMMAND => {
                Some(Message::SelectArticle)
            }
            Key::Character("k") if modifiers.command() || modifiers.control() => {
                Some(Message::SearchShortcut)
            }
            Key::Character("/") if modifiers.is_empty() => Some(Message::SearchShortcut),
            _ => None,
        }
    })
}

fn symbol(bytes: &'static [u8], size: f32) -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(size)
        .height(size)
        .style(|theme: &Theme, _| svg::Style {
            color: Some(theme.neutral.foreground.into()),
        })
        .into()
}

fn divider() -> Slot<'static, Message> {
    container(Element::from(rule::vertical(1)))
        .height(16)
        .into()
}

impl App {
    pub(super) fn header(&self, width: f32) -> Element<'_, Message, Theme> {
        let desktop = width >= 1024.;
        let full = width >= 1280.;
        let mobile = width < 768.;
        let nav: Slot<'_, Message> = if desktop {
            Row::with_children(NAVIGATION.iter().map(|&(label, url)| {
                button(label)
                    .style(button::Ghost)
                    .height(32)
                    .translate(false)
                    .on_press(Message::OpenUrl(url))
                    .into()
            }))
            .into()
        } else {
            dropdown_menu(
                |menu| {
                    menu.trigger(
                        button(row![icon::list().size(16), "Menu"].spacing(8)).style(button::Ghost),
                    )
                },
                |_| {
                    let search = menu::item("Search documentation...")
                        .on_select(Message::Search(true))
                        .into();
                    let links = NAVIGATION.iter().map(|&(label, url)| {
                        menu::item(label).on_select(Message::OpenUrl(url)).into()
                    });
                    column([search, menu::separator()].into_iter().chain(links)).into()
                },
            )
            .align(dropdown_menu::Align::Start)
            .width(176)
            .into()
        };
        let search = dialog(
            move |d| {
                if mobile {
                    return Space::new().into();
                }
                let label = if full {
                    "Search documentation..."
                } else {
                    "Search..."
                };
                let style: StyleFn<'_, Theme> = Box::new(|theme, status| {
                    let mut style = ghost(theme, status);
                    style.border.radius = radius::LG.into();
                    let surface = if theme.neutral.is_dark {
                        theme.card.base
                    } else {
                        theme.muted.base
                    };
                    style.background = Some(
                        surface
                            .scale_alpha(if status == Status::Hovered { 0.5 } else { 1.0 })
                            .into(),
                    );
                    style.into()
                });
                d.trigger(
                    button(text::small(label).width(Fill))
                        .width(if full {
                            256
                        } else if desktop {
                            160
                        } else {
                            192
                        })
                        .style(style)
                        .focus_ring(3, |theme| {
                            theme.neutral.ring.scale_alpha(0.5).over(theme.neutral.base)
                        })
                        .translate(false),
                )
            },
            |_| self.palette(),
        )
        .open(self.search)
        .on_open_change(Message::Search)
        .show_close_button(false)
        .padding(search::INSET + search::RIM)
        .max_width(512. + 2. * f32::from(search::RIM))
        .margin(16. - f32::from(search::RIM))
        .backdrop(false)
        .style(search::panel);
        let github = symbol(include_bytes!("../../../../assets/github.svg"), 16.);
        let mut repo = row![github].spacing(6);
        if let Some(stars) = self.stars {
            let count = if stars >= 1000 {
                format!("{}k", (stars + 500) / 1000)
            } else {
                stars.to_string()
            };
            repo = repo.push(text::muted(count).size(12).into());
        }
        let github = button(repo)
            .style(button::Ghost)
            .translate(false)
            .on_press(Message::OpenUrl(GITHUB));
        let theme = tooltip(
            button(symbol(
                include_bytes!("../../assets/navigation/theme.svg"),
                18.,
            ))
            .square()
            .style(button::Ghost)
            .on_press(Message::ToggleTheme),
            "Toggle theme",
        );
        let fonts = dialog(
            |_| Space::new().into(),
            |_| {
                column![
                    text::label("Font"),
                    combo_box(
                        &self.font_list,
                        "Search fonts...",
                        Some(&self.current_font),
                        Message::FontSelected
                    )
                    .width(Fill)
                ]
                .width(Fill)
                .spacing(8)
                .into()
            },
        )
        .open(self.fonts_open)
        .on_open_change(Message::Fonts)
        .max_width(384.);
        let mut tools = row![nav, space::horizontal()]
            .width(Fill)
            .height(if desktop { 64 } else { 56 })
            .align_y(Center)
            .spacing(8)
            .padding([0, 24]);
        let hidden_search = if mobile {
            Some(search)
        } else {
            tools = tools.push(search.into());
            if desktop {
                tools = tools.push(
                    container(rule::vertical(1).boxed())
                        .padding([0, 0, 0, 8])
                        .height(16)
                        .into(),
                );
            }
            None
        };
        let toolbar = tools.push(github.into()).push(divider()).push(theme.into());
        let mut layers = stack![Element::from(toolbar), Element::from(fonts)];
        if let Some(search) = hidden_search {
            layers = layers.push(Element::from(search));
        }
        layers.boxed()
    }

    fn palette(&self) -> Slot<'_, Message> {
        let settings = command::item("Change font…")
            .group("Settings")
            .icon(icon::settings)
            .on_select(Message::Fonts(true));
        let pages = self.pages.iter().enumerate().map(|(i, page)| {
            command::item(page.meta().title)
                .group("Components")
                .icon(icon::book_open)
                .on_select(Message::Navigate(i, None))
        });
        let sections = self.pages.iter().enumerate().flat_map(|(i, page)| {
            let component = page.meta().title;
            page::sections(&**page).map(move |(block, title)| {
                command::item(format!("{component} › {title}"))
                    .group("Sections")
                    .on_select(Message::Navigate(i, Some(block)))
            })
        });
        let links = NAVIGATION.iter().map(|&(label, url)| {
            command::item(label)
                .group("Navigation")
                .on_select(Message::OpenUrl(url))
        });
        let guides = GUIDES
            .iter()
            .filter(|(_, url)| !NAVIGATION.iter().any(|(_, nav)| *nav == *url))
            .map(|&(label, url)| {
                command::item(label)
                    .group("Documentation")
                    .on_select(Message::OpenUrl(url))
            });
        command(
            pages
                .chain([settings])
                .chain(links)
                .chain(guides)
                .chain(sections),
        )
        .limit(8)
        .placeholder("Search documentation...")
        .empty("No results found.")
        .embedded(search::field)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::keyboard::key::Named;
    use iced::time::Instant;
    use iced::window;
    use iced_test::core::Settings;
    use iced_test::simulator::Simulator;
    use icedcn::widget::markdown::highlighter::Map;

    fn app() -> App {
        App::new(Map::from(&Theme::light()), Map::from(&Theme::dark())).0
    }

    fn header(app: &App, width: f32) -> Simulator<'_, Message, Theme, iced::Renderer> {
        Simulator::with_size(
            Settings::default(),
            iced::Size::new(width, 640.),
            app.header(width),
        )
    }

    #[test]
    fn desktop_links_and_mobile_menu_publish_the_same_destinations() {
        let app = app();
        let mut desktop = header(&app, 1440.);
        desktop.click("Charts").unwrap();
        assert!(
            desktop
                .drain()
                .any(|m| matches!(m, Message::OpenUrl("icedcn://component/chart")))
        );
        assert!(desktop.find("New").is_err());
        desktop.click("Examples").unwrap();
        assert!(
            desktop
                .drain()
                .any(|m| matches!(m, Message::OpenUrl("https://github.com/airstrike/icedcn/tree/HEAD/examples")))
        );
        let mut mobile = header(&app, 375.);
        mobile.click("Menu").unwrap();
        mobile.click("Charts").unwrap();
        assert!(
            mobile
                .drain()
                .any(|m| matches!(m, Message::OpenUrl("icedcn://component/chart")))
        );
    }

    #[test]
    fn component_links_work_before_the_markdown_finishes_loading() {
        let mut app = app();
        for &(_, url) in NAVIGATION.iter().chain(GUIDES) {
            if let Some(target) = url.strip_prefix("icedcn://component/") {
                let _ = app.update(Message::OpenComponent(target.to_owned()));
                let current = &app.pages[app.current_page];
                let (name, fragment) = target.split_once('#').unwrap_or((target, ""));
                assert_eq!(page::slug(current.meta().title), name);
                assert!(current.state().section(fragment).is_some());
            }
        }
        let _ = app.update(Message::OpenComponent("attachment#trigger".into()));
        let current = &app.pages[app.current_page];
        assert_eq!(current.meta().title, "Attachment");
        assert!(current.state().section("trigger").is_some());
        let _ = app.update(Message::OpenComponent("missing".into()));
        assert_eq!(app.pages[app.current_page].meta().title, "Attachment");
    }

    #[test]
    fn keyboard_search_filters_sections_and_selects_the_correct_installed_page() {
        let mut app = app();
        let mut ui = header(&app, 1440.);
        ui.click("Search documentation...").unwrap();
        let open = ui
            .into_messages()
            .find(|m| matches!(m, Message::Search(true)))
            .unwrap();
        let _ = app.update(open);
        let mut ui = header(&app, 1440.);
        ui.simulate([Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )]);
        ui.typewrite("message opening saved");
        ui.find("Message Scroller › Opening Saved Threads").unwrap();
        ui.tap_key(Named::Enter);
        let target = ui
            .into_messages()
            .find_map(|m| match m {
                Message::Navigate(page, Some(section)) => Some((page, section)),
                _ => None,
            })
            .expect("Enter chooses the matching example");
        assert_eq!(app.pages[target.0].meta().title, "Message Scroller");
        assert!(
            page::sections(&*app.pages[target.0])
                .any(|(i, title)| i == target.1 && title == "Opening Saved Threads")
        );
        let _ = app.update(Message::Navigate(target.0, Some(target.1)));
        assert_eq!(app.current_page, target.0);
        assert!(!app.search);
    }
}
