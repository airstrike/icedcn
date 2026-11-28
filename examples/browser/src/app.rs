mod fade;
mod header;
mod home;
mod menu;
mod not_found;
mod route;
#[cfg(test)]
mod tests;
#[cfg(not(target_arch = "wasm32"))]
pub mod web;

use browser_routing::{Route as _, Router};
use iced::Element;
use iced::Widget;
use iced::font::set_defaults;
use iced::mouse::Interaction;
use iced::theme::Mode;
use iced::widget::{column, operation, responsive, row, space, stack};
use iced::{clipboard, system, window};
use icedcn::widget::scrollable;
use route::Route;
use std::time::Duration;

use iced::{Fill, Font, Pixels, Shrink, Subscription, Task};
use icedcn::theme::tweak::Bg;
use icedcn::theme::tweak::token::Token;
use icedcn::widget::container;
use icedcn::widget::markdown::highlighter::Map;
use icedcn::widget::sidebar::MenuButtonSize;

use icedcn::widget::{alert, button, context_menu, rule, sidebar, text, toast, toaster};
use icedcn::{Size, Theme, Weight, theme};

use crate::component;
use crate::docs;
use crate::examples;
use crate::guide;
use crate::icon;
use crate::page::{self, SnippetViewer};
use crate::sleep;
use crate::{font, fonts};

const PINNED_FONTS: &[&str] = &["Geist", "Inter"];

pub struct App {
    dark_mode: bool,
    manual_mode: bool,
    destination: Route,
    create: palette::State,
    routes: Router<route::Definition>,
    examples: examples::State,
    width: f32,
    search: bool,
    fonts_open: bool,
    stars: Option<u64>,
    current_page: usize,
    pages: Vec<Box<dyn page::Page>>,
    component_count: usize,
    gh_light: Map,
    gh_dark: Map,
    snippet_viewer: SnippetViewer,
    fount: fonts::Fount,
    font_list: Vec<String>,
    current_font: String,
    /// Show the CPU-rendering notice; only ever set on the web.
    cpu_renderer: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme,
    SystemMode(Mode),
    Example(examples::Event),
    Create(palette::Message),
    Escape,
    Width(f32),
    SelectPage(usize),
    /// Restore a browser history entry without creating another entry.
    History(String),
    /// Navigate to a registered site path.
    GoTo(&'static str),
    OpenUrl(&'static str),
    OpenComponent(String),
    CopyText(String),
    CreateEvent,
    SaveAttachment,
    AttachmentSaved(Result<(), String>),
    FontSelected(String),
    Page(usize, page::Message),
    Font(fonts::Message),
    Search(bool),
    SearchShortcut,
    FocusNext,
    FocusPrevious,
    SelectArticle,
    Fonts(bool),
    Navigate(usize, Option<usize>),
    Stars(Option<u64>),
    /// `App` ignores; `Browser` intercepts the `value == 100` case to dismiss
    /// the splash so it stays up until the bar visibly reaches the right edge.
    SplashIdle(f32),
    DismissCpuRenderer,
}

impl App {
    pub fn new(gh_light: Map, gh_dark: Map) -> (Self, Task<Message>) {
        let mut pages: Vec<Box<dyn page::Page>> = Vec::new();
        let mut tasks: Vec<Task<Message>> = Vec::new();

        use component::{
            accordion, alert, alert_dialog, aspect_ratio, attachment, avatar, badge, breadcrumb,
            bubble, button, button_group, calendar, card, carousel, chart, checkbox, code_block,
            collapsible, combobox, command, context_menu, data_table, date_picker, dialog, drawer,
            dropdown_menu, empty, field, hover_card, input, input_group, input_otp, item, kbd,
            label, marker, menubar, message, native_select, navigation_menu, pagination, popover,
            progress, questionnaire, radio_group, resizable, scroll_area, scroller, select,
            separator, sheet, sidebar, skeleton, slider, spinner, switch, table, tabs, text,
            textarea, toast, toggle, toggle_group, tooltip, trace,
        };

        let constructors: Vec<fn(usize) -> Box<dyn page::Page>> = vec![
            accordion::AccordionPage::new,
            alert::AlertPage::new,
            alert_dialog::AlertDialogPage::new,
            aspect_ratio::AspectRatioPage::new,
            attachment::AttachmentPage::new,
            avatar::AvatarPage::new,
            badge::BadgePage::new,
            breadcrumb::BreadcrumbPage::new,
            bubble::BubblePage::new,
            button::ButtonPage::new,
            button_group::ButtonGroupPage::new,
            calendar::CalendarPage::new,
            card::CardPage::new,
            carousel::CarouselPage::new,
            chart::ChartPage::new,
            checkbox::CheckboxPage::new,
            code_block::CodeBlockPage::new,
            collapsible::CollapsiblePage::new,
            combobox::ComboboxPage::new,
            command::CommandPage::new,
            context_menu::ContextMenuPage::new,
            data_table::DataTablePage::new,
            date_picker::DatePickerPage::new,
            dialog::DialogPage::new,
            drawer::DrawerPage::new,
            dropdown_menu::DropdownMenuPage::new,
            empty::EmptyPage::new,
            field::FieldPage::new,
            hover_card::HoverCardPage::new,
            input::InputPage::new,
            input_group::InputGroupPage::new,
            input_otp::InputOtpPage::new,
            item::ItemPage::new,
            kbd::KbdPage::new,
            label::LabelPage::new,
            marker::MarkerPage::new,
            menubar::MenubarPage::new,
            message::MessagePage::new,
            scroller::Page::new,
            native_select::NativeSelectPage::new,
            navigation_menu::NavigationMenuPage::new,
            pagination::PaginationPage::new,
            popover::PopoverPage::new,
            progress::ProgressPage::new,
            questionnaire::QuestionnairePage::new,
            radio_group::RadioGroupPage::new,
            resizable::ResizablePage::new,
            scroll_area::ScrollAreaPage::new,
            select::SelectPage::new,
            separator::SeparatorPage::new,
            sheet::SheetPage::new,
            sidebar::SidebarPage::new,
            skeleton::SkeletonPage::new,
            slider::SliderPage::new,
            spinner::SpinnerPage::new,
            switch::SwitchPage::new,
            table::TablePage::new,
            tabs::TabsPage::new,
            text::TextPage::new,
            textarea::TextareaPage::new,
            toast::Page::new,
            toggle::TogglePage::new,
            toggle_group::ToggleGroupPage::new,
            tooltip::TooltipPage::new,
            trace::TracePage::new,
        ];

        let width = 1440.;
        for (i, ctor) in constructors.into_iter().enumerate() {
            let mut page = ctor(i);
            page.window_width(width);
            pages.push(page);
        }

        let component_count = pages.len();
        pages.extend(guide::pages(component_count));
        let routes = Router::new(
            pages
                .iter()
                .enumerate()
                .map(|(index, page)| {
                    let namespace = if index < component_count {
                        "components"
                    } else {
                        "docs"
                    };
                    route::Definition {
                        destination: Route::Page(index),
                        path: format!("/{namespace}/{}", page::slug(page.meta().title)),
                        name: page.meta().title,
                        title: Some(page.meta().title),
                    }
                })
                .chain([
                    route::Definition {
                        destination: Route::Home,
                        path: "/".into(),
                        name: "Home",
                        title: Some("Home"),
                    },
                    route::Definition {
                        destination: Route::Create,
                        path: "/create".into(),
                        name: "Create",
                        title: Some("Create"),
                    },
                    route::Definition {
                        destination: Route::Examples,
                        path: "/examples".into(),
                        name: "Examples",
                        title: Some("Examples"),
                    },
                    route::Definition {
                        destination: Route::Receipts,
                        path: "/examples/receipts".into(),
                        name: "Receipts",
                        title: Some("Receipts"),
                    },
                    route::Definition {
                        destination: Route::Playground,
                        path: "/examples/playground".into(),
                        name: "Playground",
                        title: Some("Playground"),
                    },
                ]),
        );
        tasks.push(
            window::oldest()
                .and_then(window::size)
                .map(|size| Message::Width(size.width)),
        );
        tasks.push(Task::future(header::stars()).map(Message::Stars));

        let default_font = font::BODY.to_owned();

        let dark_mode = false;
        tasks.push(system::theme().map(Message::SystemMode));
        let active_theme = docs::theme(dark_mode);
        let snippet_viewer = SnippetViewer::new(
            if dark_mode {
                gh_dark.clone()
            } else {
                gh_light.clone()
            },
            &active_theme,
        );

        let (mut create, create_init) = palette::State::embedded(if dark_mode {
            palette::Mode::Dark
        } else {
            palette::Mode::Light
        });
        create.set_font(font::BODY.into());
        tasks.push(create_init.map(Message::Create));

        let mut app = Self {
            dark_mode,
            manual_mode: false,
            destination: Route::Home,
            create,
            routes,
            examples: examples::State::default(),
            width,
            search: false,
            fonts_open: false,
            stars: None,
            current_page: 0,
            pages,
            component_count,
            gh_light,
            gh_dark,
            snippet_viewer,
            fount: fonts::Fount::new(),
            font_list: vec![default_font.clone()],
            current_font: default_font,
            cpu_renderer: crate::cpu_renderer(),
        };
        let initial = browser_routing::location()
            .map(|path| Route::parse(&path, &app.routes))
            .unwrap_or(Route::Home);
        tasks.push(app.restore(initial));
        (app, Task::batch(tasks))
    }

    pub fn is_dark(&self) -> bool {
        self.dark_mode
    }

    pub(crate) fn theme(&self) -> Theme {
        if self.destination == Route::Create {
            self.create.theme()
        } else {
            self.snippet_viewer.theme().clone()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let i = self.current_page;
        let page = match self
            .pages
            .get(i)
            .filter(|_| matches!(self.destination, Route::Page(_)))
        {
            Some(page) => page
                .subscription()
                .with(i)
                .map(|(i, m)| Message::Page(i, m)),
            _ => Subscription::none(),
        };
        Subscription::batch([
            page,
            header::shortcuts(),
            system::theme_changes().map(Message::SystemMode),
            browser_routing::subscription().map(Message::History),
            window::resize_events().map(|(_, size)| Message::Width(size.width)),
        ])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        if let Message::History(path) = message {
            let destination = Route::parse(&path, &self.routes);
            return self.restore(destination);
        }
        if let Message::GoTo(path) = message {
            let destination = Route::parse(path, &self.routes);
            let changed = destination != self.route();
            let task = self.restore(destination);
            if changed {
                browser_routing::push(path);
            }
            return task;
        }
        let previous = self.route();
        let task = self.update_inner(message);
        let destination = self.route();
        if destination != previous {
            if let Some(path) = destination.path(&self.routes) {
                browser_routing::push(path);
            }
            browser_routing::set_title(&self.title());
        }
        task
    }

    fn route(&self) -> Route {
        self.destination.clone()
    }

    fn showing_examples(&self) -> bool {
        matches!(
            self.destination,
            Route::Examples | Route::Receipts | Route::Playground
        )
    }

    fn restore(&mut self, destination: Route) -> Task<Message> {
        self.search = false;
        self.fonts_open = false;
        let message = match destination {
            Route::Home | Route::Create | Route::NotFound(_) => {
                self.destination = destination;
                browser_routing::set_title(&self.title());
                return Task::none();
            }
            Route::Page(index) => Message::SelectPage(index),
            Route::Examples => Message::Example(examples::Event::Gallery),
            Route::Receipts => Message::Example(examples::Event::OpenReceipts),
            Route::Playground => Message::Example(examples::Event::OpenPlayground),
        };
        let task = self.update_inner(message);
        browser_routing::set_title(&self.title());
        task
    }

    /// Title of the active site destination.
    pub fn title(&self) -> String {
        let destination = self.route();
        let title = destination
            .path(&self.routes)
            .and_then(|path| self.routes.find(path))
            .map(|definition| definition.title().unwrap_or_else(|| definition.name()))
            .unwrap_or("Page not found");
        format!("{title} — icedcn")
    }

    fn set_mode(&mut self, mode: Mode) -> Task<Message> {
        self.dark_mode = mode == Mode::Dark;
        let map = if self.dark_mode {
            self.gh_dark.clone()
        } else {
            self.gh_light.clone()
        };
        self.snippet_viewer
            .set_theme(map, docs::theme(self.dark_mode));
        self.create
            .update(palette::Message::SetMode(mode.into()))
            .map(Message::Create)
    }

    fn update_inner(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::History(_) | Message::GoTo(_) => Task::none(),
            Message::Example(event) => {
                self.search = false;
                let task = self.examples.update(event);
                self.destination = if self.examples.is_playground() {
                    Route::Playground
                } else if self.examples.is_open() {
                    Route::Receipts
                } else {
                    Route::Examples
                };
                task
            }
            Message::Escape if self.showing_examples() && !self.search && !self.fonts_open => {
                let task = self.examples.escape();
                if !self.examples.is_open() {
                    self.destination = Route::Examples;
                }
                task
            }
            Message::Escape => Task::none(),
            Message::Width(width) => {
                self.width = width;
                for page in &mut self.pages {
                    page.window_width(width);
                }
                Task::none()
            }
            Message::Create(palette::Message::ChooseFont) => {
                self.update_inner(Message::Fonts(true))
            }
            Message::Create(palette::Message::SetMode(mode)) => {
                self.manual_mode = true;
                self.set_mode(mode.into())
            }
            Message::Create(message) => self.create.update(message).map(Message::Create),
            Message::ToggleTheme => {
                self.manual_mode = true;
                self.set_mode(if self.dark_mode {
                    Mode::Light
                } else {
                    Mode::Dark
                })
            }
            Message::SystemMode(mode) => {
                if self.manual_mode {
                    Task::none()
                } else {
                    self.set_mode(mode)
                }
            }
            Message::OpenUrl(url) => {
                self.search = false;
                match url {
                    "icedcn://examples" => {
                        self.update_inner(Message::Example(examples::Event::Gallery))
                    }
                    "icedcn://examples/receipts" => {
                        self.update_inner(Message::Example(examples::Event::OpenReceipts))
                    }
                    "icedcn://examples/playground" => {
                        self.update_inner(Message::Example(examples::Event::OpenPlayground))
                    }
                    _ => page::open_link(url),
                }
            }
            Message::OpenComponent(target) => {
                let (name, fragment) = target
                    .split_once('#')
                    .map_or((target.as_str(), None), |(name, fragment)| {
                        (name, Some(fragment))
                    });
                let Some(index) = self
                    .pages
                    .iter()
                    .position(|p| page::slug(p.meta().title) == name)
                else {
                    return Task::none();
                };
                let section = fragment.and_then(|id| self.pages[index].state().section(id));
                self.update_inner(Message::Navigate(index, section))
            }
            Message::CopyText(value) => clipboard::write(value).discard(),
            Message::CreateEvent => toast::promise::<_, Message, _, ()>(
                "Loading...",
                async {
                    sleep(Duration::from_secs(2)).await;
                    Ok("Event")
                },
                |name| format!("{name} has been created"),
                |()| "Error".to_owned(),
            ),
            Message::SaveAttachment => Task::perform(
                async {
                    let file = rfd::AsyncFileDialog::new()
                        .set_file_name("sales-dashboard.pdf")
                        .save_file()
                        .await;
                    if let Some(file) = file {
                        file.write(include_bytes!("../assets/attachments/sales-dashboard.pdf"))
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                    Ok(())
                },
                Message::AttachmentSaved,
            ),
            Message::AttachmentSaved(Ok(())) => Task::none(),
            Message::AttachmentSaved(Err(error)) => Task::perform(
                async move {
                    rfd::AsyncMessageDialog::new()
                        .set_title("Could not save attachment")
                        .set_description(error)
                        .show()
                        .await;
                },
                |_| Message::AttachmentSaved(Ok(())),
            ),
            Message::SelectPage(page) => {
                if page >= self.pages.len() {
                    return Task::none();
                }
                if page != self.current_page
                    && let Some(prev) = self.pages.get_mut(self.current_page)
                {
                    prev.reset_volatile();
                }
                self.destination = Route::Page(page);
                self.current_page = page;
                let page = &mut self.pages[page];
                let visible = page.on_visible();
                if page.state().unvisited() {
                    Task::batch([page.init(), visible])
                } else {
                    visible
                }
            }
            Message::FontSelected(name) => {
                self.fonts_open = false;
                self.current_font = name.clone();

                if self.fount.system_families().contains(&name) {
                    self.apply_font(name)
                } else {
                    let task = fonts::load(name, self.fount.google_catalog());
                    task.map(Message::Font)
                }
            }
            Message::Page(i, page::Message::ScrollTo(_)) if i != self.current_page => Task::none(),
            Message::Page(i, message) => {
                if let Some(page) = self.pages.get_mut(i) {
                    page.update(message)
                } else {
                    Task::none()
                }
            }
            Message::Search(open) => {
                self.search = open;
                self.fonts_open = false;
                Task::none()
            }
            Message::SelectArticle => {
                use iced::widget::operation::text::select_all;
                if self.search || self.fonts_open || !matches!(self.destination, Route::Page(_)) {
                    Task::none()
                } else {
                    select_all(self.pages[self.current_page].scrollable_id())
                }
            }
            Message::FocusNext if self.showing_examples() && !self.search && !self.fonts_open => {
                self.examples.focus(false)
            }
            Message::FocusPrevious
                if self.showing_examples() && !self.search && !self.fonts_open =>
            {
                self.examples.focus(true)
            }
            Message::FocusNext => operation::focus_next(),
            Message::FocusPrevious => operation::focus_previous(),
            Message::SearchShortcut => self.update_inner(Message::Search(!self.search)),
            Message::Fonts(open) => {
                self.fonts_open = open;
                self.search = false;
                Task::none()
            }
            Message::Stars(stars) => {
                self.stars = stars;
                Task::none()
            }
            Message::DismissCpuRenderer => {
                self.cpu_renderer = false;
                Task::none()
            }
            Message::Navigate(page, section) => {
                self.search = false;
                let activate = self.update_inner(Message::SelectPage(page));
                match section {
                    Some(index) => Task::batch([
                        activate,
                        Task::done(Message::Page(page, page::Message::ScrollTo(index))),
                    ]),
                    None => activate,
                }
            }
            Message::SplashIdle(_) => Task::none(),
            Message::Font(msg) => match msg {
                fonts::Message::SystemFontsLoaded(families) => {
                    let names: Vec<String> = families
                        .into_iter()
                        .map(|f| f.to_string())
                        .filter(|name| !name.starts_with('.'))
                        .collect();
                    self.fount.set_system_families(names);
                    self.rebuild_font_list();
                    Task::none()
                }
                fonts::Message::CatalogLoaded(Ok(catalog)) => {
                    self.fount.set_google_catalog(catalog);
                    self.rebuild_font_list();
                    fonts::load(self.current_font.clone(), self.fount.google_catalog())
                        .map(Message::Font)
                }
                fonts::Message::CatalogLoaded(Err(_)) => Task::none(),
                fonts::Message::Loaded(name, Err(e)) => {
                    eprintln!("Font loading failed ({name}): {e}");
                    Task::none()
                }
                fonts::Message::Loaded(name, Ok(())) if name != self.current_font => Task::none(),
                fonts::Message::Loaded(name, Ok(())) => self.apply_font(name),
            },
        }
    }

    fn apply_font(&mut self, name: String) -> Task<Message> {
        let name: &'static str = Box::leak(name.into_boxed_str());
        let body = Font::from(name);
        self.snippet_viewer.set_font(body);
        self.create.set_font(body);
        set_defaults(body, Pixels(14.0))
    }

    fn rebuild_font_list(&mut self) {
        let mut names = self.fount.families();

        // Reverse iteration so the first pinned font ends up first.
        for &pinned in PINNED_FONTS.iter().rev() {
            if let Some(pos) = names.iter().position(|n| n == pinned) {
                let name = names.remove(pos);
                names.insert(0, name);
            }
        }

        self.font_list = names;
    }

    /// The docs sidebar: `--sidebar-width` (288px) inside the window's `px-2`,
    /// `top-[calc(var(--header-height)+0.6rem)]` and `h-[calc(100svh-10rem)]`.
    fn sidebar(&self) -> iced::Element<'_, Message, Theme> {
        // `SidebarGroupLabel` is `h-8 px-2`; the menu button's 1px inset below
        // it supplies the label's last pixel.
        let label = |title: &'static str| {
            container(
                text::xs(title)
                    .selectable(false)
                    .weight(Weight::Medium)
                    .style(theme::text::Muted),
            )
            .height(31)
            .padding(iced::Padding::ZERO.top(8).right(8).left(8))
        };

        let current_slug = page::slug(self.pages[self.current_page].meta().title);
        // `gap-1` between the sections' 30px items, `gap-0.5` between the components'.
        let sections = column(header::GUIDES.iter().map(|&(label, url)| {
            Element::from(
                sidebar::menu_button(label)
                    .active(url.rsplit('/').next() == Some(current_slug.as_str()))
                    .size(MenuButtonSize::Docs)
                    .inset(1)
                    .on_press(Message::OpenUrl(url)),
            )
        }))
        .spacing(2);
        let components = column(
            self.pages
                .iter()
                .take(self.component_count)
                .enumerate()
                .map(|(i, page)| {
                    Element::from(
                        sidebar::menu_button(page.meta().title)
                            .size(MenuButtonSize::Docs)
                            .inset(1)
                            .active(i == self.current_page)
                            .on_press(Message::SelectPage(i)),
                    )
                }),
        )
        .width(Fill);

        // `SidebarContent` is `--sidebar-menu-width` (224px) wide with `pl-2.5`.
        // Its `p-2` groups sit `gap-2` apart, and the first opens with `pt-12`.
        let menu = column![
            Element::from(label("Sections")),
            sections,
            space::vertical().height(23),
            Element::from(label("Components")),
            components,
        ]
        .padding(iced::Padding::ZERO.top(48).right(8).bottom(7).left(18))
        .width(224);
        // The scroll region reaches the divider, so its scrollbar runs along it. It shows
        // while the menu scrolls and fades out once idle, like an overlay scrollbar.
        let menu = scrollable::scrollable(menu)
            .width(Fill)
            .height(Fill)
            .visibility(scrollable::Visibility::Scroll)
            .fade(scrollable::FADE);

        sidebar::fixed(row![
            fade::fade(menu, |theme| theme.neutral.base.into()),
            responsive(divider).width(1).boxed(),
        ])
        .padding(iced::Padding::ZERO.top(9.6).right(8).bottom(86.4).left(8))
        .width(296)
        .height(Fill)
        .style(theme::sidebar::Default + Bg(Token::Background))
        .into()
    }

    pub fn view(&self) -> iced::Element<'_, Message, Theme> {
        context_menu(|_| self.chrome().into(), |_| menu::view(self))
            .width(248)
            .into()
    }

    fn chrome(&self) -> iced::Element<'_, Message, Theme> {
        if !self.cpu_renderer {
            return self.content();
        }
        let close = button(icon::x())
            .style(button::Ghost)
            .size(Size::Xs)
            .square()
            .on_press(Message::DismissCpuRenderer);
        let notice = alert()
            .icon(icon::triangle_alert().size(16).line_height(1.0))
            .title("Hardware acceleration is off")
            .description(
                "This page is drawing on the CPU, so it may feel slow. \
                 Turn on hardware acceleration in your browser for full speed.",
            )
            .action(close)
            .style(alert::Destructive)
            .max_width(640);
        column![
            Element::from(container(notice).padding([12, 16]).center_x(Fill)),
            self.content(),
        ]
        .width(Fill)
        .height(Fill)
        .boxed()
    }

    fn content(&self) -> iced::Element<'_, Message, Theme> {
        let width = self.width;
        let header = self.header(width);

        if let Route::NotFound(path) = &self.destination {
            return column![header, not_found::view(path)]
                .width(Fill)
                .height(Fill)
                .boxed();
        }

        if self.destination == Route::Home {
            return column![header, home::view(width)]
                .width(Fill)
                .height(Fill)
                .boxed();
        }

        if self.destination == Route::Create {
            return column![
                header,
                self.create.view_embedded(width).map(Message::Create)
            ]
            .width(Fill)
            .height(Fill)
            .boxed();
        }

        if self.showing_examples() {
            return toaster(
                column![header, self.examples.view(width)]
                    .width(Fill)
                    .height(Fill)
                    .boxed(),
            )
            .position(toast::Position::TopCenter)
            .into();
        }

        let current = &*self.pages[self.current_page];
        let meta = current.meta();
        let title = text::h1(meta.title);
        // `sm:text-balance md:max-w-[80%]` of the 640px column.
        let mut description = text::p(meta.description).style(theme::text::Muted);
        if width >= 640.0 {
            description = description.wrapping(text::Wrapping::Balanced);
        }
        let description =
            container(description).max_width(if width >= 768.0 { 512.0 } else { f32::INFINITY });

        let total = self.pages.len();
        let curr = self.current_page;
        let prev_btn = button(icon::arrow_left())
            .style(button::Secondary)
            .size(Size::Sm)
            .square()
            .translate(false)
            .interaction(Interaction::Pointer)
            .on_press(Message::SelectPage(curr.saturating_sub(1)));
        let next_btn = button(icon::arrow_right())
            .style(button::Secondary)
            .size(Size::Sm)
            .square()
            .translate(false)
            .interaction(Interaction::Pointer)
            .on_press(Message::SelectPage((curr + 1).min(total.saturating_sub(1))));

        let title_row = row![
            Element::from(title),
            space::horizontal(),
            row![Element::from(prev_btn), Element::from(next_btn)].spacing(8),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Start);
        let page_header = column![title_row, Element::from(description)].spacing(8);

        // The sidebar shows from `lg`, the table of contents from `xl`.
        let desktop = width >= 1024.0;
        let wide = width >= 1280.0;
        let page_body = page::body(current, &self.snippet_viewer, wide);

        // `max-w-160 px-4 py-6 md:px-0 lg:py-8` below `--top-spacing` (16px at `lg`).
        let gutter = if width >= 768.0 { 0.0 } else { 16.0 };
        let (top, bottom) = if desktop { (48.0, 32.0) } else { (24.0, 24.0) };
        let article = container(
            column![page_header, Element::from(container(page_body).width(Fill))].spacing(24),
        )
        .padding(iced::Padding {
            top,
            right: gutter,
            bottom,
            left: gutter,
        })
        .width(Fill)
        .max_width(640);
        // The column scrolls to the window's edge, like the page, and keeps the
        // `px-2` gutter and the table of contents' `--sidebar-width` clear.
        let content = container(article).center_x(Fill).padding(
            iced::Padding::ZERO
                .right(if wide { 296.0 } else { 8.0 })
                .left(if desktop { 0.0 } else { 8.0 }),
        );
        let main = scrollable::scrollable(content)
            .width(Fill)
            .height(Fill)
            .id(current.scrollable_id());

        let mut docs = stack![Element::from(main)].width(Fill).height(Fill);
        if wide && page::has_toc(current) {
            docs = docs.push(responsive(move |area| toc(current, area)).boxed());
        }
        let mut body = row![].width(Fill).height(Fill);
        if desktop {
            body = body.push(self.sidebar());
        }
        body = body.push(docs.boxed());

        let layout = column![header, body].width(Fill).height(Fill);

        toaster(layout.boxed())
            .position(toast::Position::TopCenter)
            .into()
    }
}

/// The sidebar's `w-px` rule, `absolute top-12 right-2 h-full`: it starts 48px
/// down, and its `transparent, border 10%, border 90%, transparent` gradient
/// spans the sidebar's full height, so the sidebar clips its last 48px.
fn divider<'a>(sidebar: iced::Size) -> Element<'a, Message, Theme> {
    use iced::gradient::Linear;
    use std::f32::consts::PI;
    const TOP: f32 = 48.0;
    let shown = (sidebar.height - TOP) / sidebar.height;
    if shown.is_nan() || shown <= 0.0 {
        return space().boxed();
    }
    let rule = rule::vertical(1).style(Box::new(move |theme: &Theme| {
        let border: iced::Color = theme.neutral.border.into();
        let background = [0.0, 0.1, 0.9]
            .into_iter()
            .filter(|&at| at < shown)
            .chain([shown])
            .fold(Linear::new(PI), |gradient, at: f32| {
                let alpha = (at / 0.1).min((1.0 - at) / 0.1).clamp(0.0, 1.0);
                gradient.add_stop(at / shown, border.scale_alpha(alpha))
            });
        rule::Style {
            background: background.into(),
            ..rule::Style::default()
        }
    }) as rule::StyleFn<'_, Theme>);
    container(Element::from(rule))
        .padding(iced::Padding::ZERO.top(TOP))
        .height(Fill)
        .into()
}

/// The "On This Page" column at the docs column's right edge: `sticky
/// top-[calc(var(--header-height)+1px)] h-[90svh] w-(--sidebar-width) gap-4
/// pb-8` after `--top-spacing`, with the links in a `px-8 scroll-fade
/// scrollbar-none` region. Its thin scrollbar shows only while it scrolls, the
/// sidebar's way. `area` is the window below the 64px header.
fn toc<'a>(page: &'a dyn page::Page, area: iced::Size) -> Element<'a, Message, Theme> {
    let links = scrollable::scrollable(container(page::toc(page)).width(Fill).padding([0, 32]))
        .width(Fill)
        .height(Shrink)
        .visibility(scrollable::Visibility::Scroll)
        .fade(scrollable::FADE);
    let column = container(fade::fade(links, |theme| theme.neutral.base.into()))
        .padding(iced::Padding::ZERO.top(33).bottom(32))
        .width(288)
        .height(1.0 + 0.9 * (area.height + 64.0));
    row![space::horizontal(), Element::from(column), space().width(8)].boxed()
}
