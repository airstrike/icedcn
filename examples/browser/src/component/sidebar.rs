//! Sidebar menus, groups and workspace selection.

use iced::widget::component::{Component, component};
use iced::{Element, Task, Widget};

use icedcn::widget::{column, row, sidebar, text};
use icedcn::{Theme, View};

use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Sidebar",
    description: "A composable, themeable and customizable sidebar component.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/sidebar-demo.md",
    },
    page::Block::Md("examples/browser/snippets/sidebar-usage.md"),
    page::Block::Md("examples/browser/snippets/sidebar-composition.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-structure.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-provider.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-root.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-handle.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-header.md"),
    page::Block::Example {
        id: "header",
        title: "Header example",
        description: Some(
            "Use sidebar::header to add a workspace switcher or branding at the top.",
        ),
        snippet: "examples/browser/snippets/sidebar-header.md",
    },
    page::Block::Md("examples/browser/snippets/sidebar-guide-footer.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-content.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-group.md"),
    page::Block::Example {
        id: "group",
        title: "Group example",
        description: Some(
            "Use sidebar::group and sidebar::group_label to create labeled sections.",
        ),
        snippet: "examples/browser/snippets/sidebar-group.md",
    },
    page::Block::Md("examples/browser/snippets/sidebar-guide-menu.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-menu-button.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-menu-action.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-menu-sub.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-menu-badge.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-menu-skeleton.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-trigger.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-rail.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-controlled.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-theming.md"),
    page::Block::Md("examples/browser/snippets/sidebar-guide-styling.md"),
    page::Block::Md("examples/browser/snippets/sidebar-api-reference.md"),
];

pub struct SidebarPage {
    state: page::State,
    preview_height: f32,
}

impl SidebarPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            preview_height: 398.,
        })
    }
}

use crate::component::avatar::alex_handle;
use crate::icon;
use iced::widget::{Space, Text, hover, stack};
use iced::{Alignment, Fill, Right};
use icedcn::theme::container::Style;
use icedcn::widget::{avatar, collapsible, container, dropdown_menu, menu, scrollable};

type Glyph = fn() -> Text<'static, Theme>;
const TEAMS: [(&str, &str, Glyph); 3] = [
    ("Acme Inc", "Enterprise", icon::gallery_vertical_end),
    ("Acme Corp.", "Startup", icon::audio_waveform),
    ("Evil Corp.", "Free", icon::command),
];
const GROUPS: [(&str, Glyph, &[&str]); 4] = [
    (
        "Playground",
        icon::square_terminal,
        &["History", "Starred", "Settings"],
    ),
    ("Models", icon::bot, &["Genesis", "Explorer", "Quantum"]),
    (
        "Documentation",
        icon::book_open,
        &["Introduction", "Get Started", "Tutorials", "Changelog"],
    ),
    (
        "Settings",
        icon::settings_2,
        &["General", "Team", "Billing", "Limits"],
    ),
];
const PROJECTS: [(&str, Glyph); 3] = [
    ("Design Engineering", icon::frame),
    ("Sales & Marketing", icon::chart_pie),
    ("Travel", icon::map),
];
struct Projects;
#[derive(Default, Copy, Clone)]
struct Workspace {
    team: usize,
    selected: Option<&'static str>,
}
#[derive(Debug, Clone)]
enum Choice {
    Team(usize),
    Nav(&'static str),
}
impl<'a> Component<'a, crate::Message, Theme> for Projects {
    type State = Workspace;
    type Event = Choice;
    fn update(
        &self,
        state: &mut Workspace,
        event: Choice,
        _: &iced::Renderer,
    ) -> Option<crate::Message> {
        match event {
            Choice::Team(i) => state.team = i,
            Choice::Nav(label) => state.selected = Some(label),
        }
        None
    }
    fn view(&self, state: &Workspace) -> impl Widget<Choice, Theme> + 'a {
        let state = *state;
        Element::from(
            sidebar::provider(move |s| {
                let mobile = s.is_mobile();
                let compact = !s.is_open() && !mobile;
                let nav = sidebar::content([
                    platform(compact, state.selected),
                    if compact {
                        Space::new().into()
                    } else {
                        projects(state.selected, mobile)
                    },
                ])
                .height(iced::Shrink);
                let side = sidebar::fixed(
                    column![
                        sidebar::header([teams(state.team, compact, mobile)]),
                        scrollable(nav).width(Fill).height(Fill),
                        sidebar::footer([account(compact, mobile)])
                    ]
                    .width(Fill)
                    .height(Fill),
                );
                row![
                    s.sidebar(side),
                    if mobile {
                        Space::new().into()
                    } else {
                        sidebar::rail_trigger()
                    },
                    sidebar::main(
                        container(sidebar::trigger())
                            .center_y(if compact { 48. } else { 64. })
                            .padding([0, if mobile { 12 } else { 0 }]),
                    )
                ]
                .width(Fill)
                .height(Fill)
                .into()
            })
            .collapsible(sidebar::Collapsible::Icon),
        )
    }
}
fn line(label: &'static str, glyph: Glyph) -> View<'static, Choice> {
    row![glyph().size(16), text(label).size(14)]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
}
fn menu_width(compact: bool, mobile: bool) -> f32 {
    if mobile {
        sidebar::WIDTH_MOBILE - 16.
    } else if compact {
        224.
    } else {
        sidebar::WIDTH - 16.
    }
}

fn team_option(label: View<'static, Choice>, glyph: Glyph) -> View<'static, Choice> {
    let tile = container(glyph().size(14)).center(24).style(|t: &Theme| {
        let mut style = Style::default();
        style.border.width = 1.;
        style.border.radius = 6.into();
        style.border.color = t.neutral.border;
        style
    });
    container(row![tile, label].spacing(8).align_y(Alignment::Center))
        .center_y(32)
        .padding([0, 2])
        .into()
}

fn teams(i: usize, compact: bool, mobile: bool) -> View<'static, Choice> {
    let (name, plan, glyph) = TEAMS[i];
    dropdown_menu(
        move |m| {
            let logo = container(glyph().size(16)).center(32).style(|t: &Theme| {
                let mut s = Style::default();
                s.background = Some(t.sidebar.primary.base.into());
                s.text_color = Some(t.sidebar.primary.foreground.into());
                s.border.radius = 8.into();
                s
            });
            let body: View<'static, Choice> = if compact {
                logo.into()
            } else {
                row![
                    logo,
                    column![text(name).medium().size(14), text(plan).size(12)].width(Fill),
                    icon::chevrons_up_down().size(16)
                ]
                .spacing(8)
                .width(Fill)
                .align_y(Alignment::Center)
                .into()
            };
            m.trigger(
                sidebar::menu_button(body)
                    .size(sidebar::MenuButtonSize::Lg)
                    .active(m.is_open())
                    .collapsed(compact),
            )
        },
        |_| {
            let mut entries = vec![menu::label("Teams")];
            entries.extend(TEAMS.iter().enumerate().map(|(i, &(name, _, glyph))| {
                menu::item(team_option(text(name).into(), glyph))
                    .shortcut(["⌘1", "⌘2", "⌘3"][i])
                    .on_select(Choice::Team(i))
                    .into()
            }));
            entries.extend([
                menu::separator(),
                menu::item(team_option(
                    text::muted("Add team").medium().into(),
                    icon::plus,
                ))
                .into(),
            ]);
            menu::group(entries)
        },
    )
    .side(if mobile {
        dropdown_menu::Side::Bottom
    } else {
        dropdown_menu::Side::Right
    })
    .align(dropdown_menu::Align::Start)
    .width(menu_width(compact, mobile))
    .into()
}
fn platform(compact: bool, selected: Option<&'static str>) -> View<'static, Choice> {
    let items = GROUPS.iter().map(move |&(name, glyph, links)| {
        collapsible(move |c| {
            let label: View<'static, Choice> = if compact {
                glyph().size(16).into()
            } else {
                row![
                    glyph().size(16),
                    text(name).width(Fill),
                    if c.is_open() {
                        icon::chevron_down().size(14)
                    } else {
                        icon::chevron_right().size(14)
                    }
                ]
                .width(Fill)
                .align_y(Alignment::Center)
                .spacing(8)
                .into()
            };
            let trigger = c.trigger(sidebar::menu_button(label).collapsed(compact).tooltip(name));
            if compact {
                return trigger;
            }
            let children = links.iter().map(move |&link| {
                sidebar::submenu_item([sidebar::menu_button(text(link).size(14))
                    .size(sidebar::MenuButtonSize::Sm)
                    .active(selected == Some(link))
                    .on_press(Choice::Nav(link))
                    .into()])
                .into()
            });
            column![trigger, c.content(sidebar::submenu(children))]
                .width(Fill)
                .into()
        })
        .width(Fill)
        .default_open(name == "Playground")
        .into()
    });
    let label = if compact {
        Space::new().into()
    } else {
        heading("Platform")
    };
    sidebar::group([label, sidebar::menu(items).into()]).into()
}
fn heading<'a, M: 'a>(label: &'a str) -> View<'a, M> {
    container(sidebar::group_label(label))
        .center_y(32)
        .padding([0, 8])
        .into()
}

fn projects(selected: Option<&'static str>, mobile: bool) -> View<'static, Choice> {
    let mut items: Vec<_> = PROJECTS
        .iter()
        .map(|&(name, glyph)| {
            let more = dropdown_menu(
                |d| d.trigger(sidebar::menu_action(icon::ellipsis().size(16))),
                move |_| {
                    menu::group([
                        menu::item(line("View Project", icon::folder))
                            .on_select(Choice::Nav(name))
                            .into(),
                        menu::item(line("Share Project", icon::forward)).into(),
                        menu::separator(),
                        menu::item(line("Delete Project", icon::trash_2)).into(),
                    ])
                },
            )
            .side(if mobile {
                dropdown_menu::Side::Bottom
            } else {
                dropdown_menu::Side::Right
            })
            .align(if mobile {
                dropdown_menu::Align::End
            } else {
                dropdown_menu::Align::Start
            })
            .width(192);
            let primary = Element::from(
                sidebar::menu_button(line(name, glyph))
                    .active(selected == Some(name))
                    .on_press(Choice::Nav(name)),
            );
            let action = Element::from(
                container(more)
                    .align_x(Right)
                    .width(Fill)
                    .center_y(32)
                    .padding([0, 4]),
            );
            let row = if mobile {
                stack([primary, action]).boxed()
            } else {
                hover(primary, action).boxed()
            };
            row.into()
        })
        .collect();
    items.push(sidebar::menu_button(line("More", icon::ellipsis)).into());
    sidebar::group([heading("Projects"), sidebar::menu(items).into()]).into()
}
fn identity(compact: bool) -> View<'static, Choice> {
    let face = avatar(Some(alex_handle())).fallback("AC");
    if compact {
        return face.into();
    }
    row![
        face,
        column![
            text("Alex Chen").size(14).medium(),
            text("m@example.com").size(12)
        ]
        .width(Fill),
        icon::chevrons_up_down().size(16)
    ]
    .spacing(8)
    .width(Fill)
    .align_y(Alignment::Center)
    .into()
}
fn account(compact: bool, mobile: bool) -> View<'static, Choice> {
    dropdown_menu(
        move |d| {
            d.trigger(
                sidebar::menu_button(identity(compact))
                    .size(sidebar::MenuButtonSize::Lg)
                    .active(d.is_open())
                    .collapsed(compact),
            )
        },
        |_| {
            menu::group([
                identity(false),
                menu::separator(),
                menu::item(line("Upgrade to Pro", icon::sparkles)).into(),
                menu::separator(),
                menu::item(line("Account", icon::badge_check)).into(),
                menu::item(line("Billing", icon::credit_card)).into(),
                menu::item(line("Notifications", icon::bell)).into(),
                menu::separator(),
                menu::item(line("Log out", icon::log_out)).into(),
            ])
        },
    )
    .width(menu_width(compact, mobile))
    .side(if mobile {
        dropdown_menu::Side::Bottom
    } else {
        dropdown_menu::Side::Right
    })
    .align(dropdown_menu::Align::End)
    .into()
}

fn group_example<'a, M: Clone + 'static>() -> View<'a, M> {
    sidebar::floating(
        collapsible(|c| {
            let links = ["Support", "Feedback"].map(|name| sidebar::menu_button(name).into());
            sidebar::group([
                c.trigger(sidebar::menu_button(
                    row![
                        text("Help").width(Fill),
                        if c.is_open() {
                            icon::chevron_down().size(16)
                        } else {
                            icon::chevron_right().size(16)
                        }
                    ]
                    .width(Fill)
                    .align_y(Alignment::Center),
                )),
                c.content(sidebar::menu(links)),
            ])
            .into()
        })
        .width(Fill)
        .default_open(true),
    )
    .width(sidebar::WIDTH)
    .height(Fill)
    .into()
}

fn header_example<'a, M: Clone + 'static>() -> View<'a, M> {
    let switcher = dropdown_menu(
        |m| {
            m.trigger(sidebar::menu_button(
                row![
                    text("Select Workspace").width(Fill),
                    icon::chevron_down().size(16)
                ]
                .width(Fill)
                .align_y(Alignment::Center),
            ))
        },
        |_| menu::group([menu::item("Acme Inc").into()]),
    )
    .align(dropdown_menu::Align::Start)
    .width(240);
    sidebar::floating(sidebar::header([switcher.into()]))
        .width(sidebar::WIDTH)
        .height(Fill)
        .into()
}

impl page::Page for SidebarPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<crate::Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<crate::Message> {
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 0.into(),
            ..Default::default()
        }
    }
    fn window_width(&mut self, width: f32) {
        self.preview_height = if width < 768. { 214. } else { 398. };
    }
    fn docs_preview(&self, id: &str, theme: &Theme) -> Element<'_, crate::Message, Theme> {
        container(self.themed_preview(id, theme))
            .width(Fill)
            .height(if id == "demo" {
                self.preview_height
            } else {
                288.
            })
            .into()
    }
    fn preview(&self, id: &str) -> iced::Element<'_, crate::Message, Theme> {
        match id {
            "demo" => component(Projects).boxed(),
            "group" => group_example().into(),
            "header" => header_example().into(),
            _ => unreachable!("unknown sidebar preview id: {id}"),
        }
    }
}
