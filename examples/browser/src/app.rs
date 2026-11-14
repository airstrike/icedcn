mod header;

use iced::Element;
use iced::Widget;
use iced::font::set_defaults;
use iced::mouse::Interaction;
use iced::widget::{column, combo_box, operation, row, space, stack};
use iced::{clipboard, window};
use icedcn::widget::scrollable;
use icedcn::widget::spinner;
use std::time::Duration;

use iced::{Center, Fill, Font, Pixels, Subscription, Task};
use icedcn::theme::tweak::Bg;
use icedcn::theme::tweak::token::Token;
use icedcn::widget::container;
use icedcn::widget::markdown::highlighter::Map;
use icedcn::widget::sidebar::MenuButtonSize;

use icedcn::widget::{button, rule, sidebar, text, toast, toaster};
use icedcn::{Size, Theme, Weight, theme};

use crate::component;
use crate::docs;
use crate::icon;
use crate::page::{self, SnippetViewer};
use crate::sleep;
use crate::{font, fonts};

const PINNED_FONTS: &[&str] = &["Geist", "Inter"];

pub struct App {
    dark_mode: bool,
    width: f32,
    search: bool,
    fonts_open: bool,
    stars: Option<u64>,
    current_page: usize,
    pages: Vec<Box<dyn page::Page>>,
    gh_light: Map,
    gh_dark: Map,
    snippet_viewer: SnippetViewer,
    fount: fonts::Fount,
    font_list: combo_box::State<String>,
    current_font: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme,
    Width(f32),
    SelectPage(usize),
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
            tasks.push(page.init());
            pages.push(page);
        }

        tasks.push(
            window::oldest()
                .and_then(window::size)
                .map(|size| Message::Width(size.width)),
        );
        tasks.push(Task::future(header::stars()).map(Message::Stars));

        let default_font = font::BODY.to_owned();

        let dark_mode = theme::is_dark_mode();
        let active_theme = docs::theme(dark_mode);
        let snippet_viewer = SnippetViewer::new(
            if dark_mode {
                gh_dark.clone()
            } else {
                gh_light.clone()
            },
            &active_theme,
        );

        (
            Self {
                dark_mode,
                width,
                search: false,
                fonts_open: false,
                stars: None,
                current_page: 0,
                pages,
                gh_light,
                gh_dark,
                snippet_viewer,
                fount: fonts::Fount::new(),
                font_list: combo_box::State::new(vec![default_font.clone()]),
                current_font: default_font,
            },
            Task::batch(tasks),
        )
    }

    pub fn is_dark(&self) -> bool {
        self.dark_mode
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let i = self.current_page;
        let page = match self.pages.get(i) {
            Some(page) => page
                .subscription()
                .with(i)
                .map(|(i, m)| Message::Page(i, m)),
            None => Subscription::none(),
        };
        Subscription::batch([
            page,
            header::shortcuts(),
            window::resize_events().map(|(_, size)| Message::Width(size.width)),
        ])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Width(width) => {
                self.width = width;
                for page in &mut self.pages {
                    page.window_width(width);
                }
                Task::none()
            }
            Message::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                let map = if self.dark_mode {
                    self.gh_dark.clone()
                } else {
                    self.gh_light.clone()
                };
                let active_theme = docs::theme(self.dark_mode);
                self.snippet_viewer.set_theme(map, active_theme);
                Task::none()
            }
            Message::OpenUrl(url) => {
                self.search = false;
                page::open_link(url)
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
                self.update(Message::Navigate(index, section))
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
                if page != self.current_page
                    && let Some(prev) = self.pages.get_mut(self.current_page)
                {
                    prev.reset_volatile();
                }
                self.current_page = page;
                self.pages
                    .get_mut(page)
                    .map(|p| p.on_visible())
                    .unwrap_or_else(Task::none)
            }
            Message::FontSelected(name) => {
                self.fonts_open = false;
                self.current_font = name.clone();

                if self.fount.system_families().contains(&name) {
                    self.apply_font(name)
                } else {
                    #[cfg(not(target_arch = "wasm32"))]
                    let task = fonts::load(name, self.fount.google_catalog());
                    #[cfg(target_arch = "wasm32")]
                    let task = fonts::load(name);
                    task.map(Message::Font)
                }
            }
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
                if self.search || self.fonts_open {
                    Task::none()
                } else {
                    select_all(self.pages[self.current_page].scrollable_id())
                }
            }
            Message::FocusNext => operation::focus_next(),
            Message::FocusPrevious => operation::focus_previous(),
            Message::SearchShortcut => self.update(Message::Search(!self.search)),
            Message::Fonts(open) => {
                self.fonts_open = open;
                self.search = false;
                Task::none()
            }
            Message::Stars(stars) => {
                self.stars = stars;
                Task::none()
            }
            Message::Navigate(page, section) => {
                self.search = false;
                let activate = self.update(Message::SelectPage(page));
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
                #[cfg(not(target_arch = "wasm32"))]
                fonts::Message::CatalogLoaded(Ok(catalog)) => {
                    self.fount.set_google_catalog(catalog);
                    self.rebuild_font_list();
                    fonts::load(self.current_font.clone(), self.fount.google_catalog())
                        .map(Message::Font)
                }
                #[cfg(not(target_arch = "wasm32"))]
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
        let body = Font::new(Box::leak(name.into_boxed_str()));
        self.snippet_viewer.set_font(body);
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

        self.font_list = combo_box::State::new(names);
    }

    pub fn view(&self) -> iced::Element<'_, Message, Theme> {
        let width = self.width;
        let header = self.header(width);
        let sidebar_header = container(
            text::xs("Components")
                .selectable(false)
                .weight(Weight::Medium)
                .style(theme::text::Muted),
        )
        .height(32)
        .padding([0, 8])
        .align_y(Center);

        let sidebar_items = column(self.pages.iter().enumerate().map(|(i, page)| {
            Element::from(
                sidebar::menu_button(page.meta().title)
                    .size(MenuButtonSize::Docs)
                    .inset(1)
                    .active(i == self.current_page)
                    .on_press(Message::SelectPage(i)),
            )
        }))
        .width(Fill);

        let sections = column(header::GUIDES.iter().map(|&(label, url)| {
            Element::from(
                sidebar::menu_button(label)
                    .size(MenuButtonSize::Docs)
                    .inset(1)
                    .on_press(Message::OpenUrl(url)),
            )
        }));
        let sections_label = container(
            text::xs("Sections")
                .selectable(false)
                .weight(Weight::Medium)
                .style(theme::text::Muted),
        )
        .height(32)
        .padding([0, 8])
        .align_y(Center);
        let sidebar_scrollable = scrollable::scrollable(
            column![
                Element::from(sections_label),
                sections,
                space::vertical().height(16),
                Element::from(sidebar_header),
                sidebar_items
            ]
            .padding(iced::Padding::ZERO.top(48).right(8).bottom(8).left(24))
            .width(240),
        )
        .height(Fill);

        // Top fade-to-bg overlay (`apps/v4/components/docs-sidebar.tsx`):
        // `h-8 from-bg via-bg/80 to-bg/50`. Pinned to top via column+space.
        // Layered over the scrollable so wheel events pass through — the
        // container has no on_press, so its `mouse_interaction` returns
        // None and `stack!` levitates the cursor through.
        let top_fade = container(space::vertical())
            .width(Fill)
            .height(32)
            .style(|t: &Theme| {
                use iced::gradient::Linear;
                use std::f32::consts::PI;
                use theme::container::Style;
                let bg: iced::Color = t.neutral.base.into();
                Style {
                    background: Some(
                        Linear::new(PI)
                            .add_stop(0.0, bg)
                            .add_stop(0.5, iced::Color { a: 0.8, ..bg })
                            .add_stop(1.0, iced::Color { a: 0.5, ..bg })
                            .into(),
                    ),
                    ..Style::default()
                }
            });
        let fade_overlay = column![Element::from(top_fade), space::vertical()]
            .width(Fill)
            .height(Fill);

        let sidebar = sidebar::fixed(
            stack![Element::from(sidebar_scrollable), fade_overlay]
                .width(240)
                .height(Fill),
        )
        .height(Fill)
        .class(theme::sidebar::Default + Bg(Token::Background));

        let current = &*self.pages[self.current_page];
        let meta = current.meta();
        let title = text::h1(meta.title);
        let description = text::p(meta.description).style(theme::text::Muted);

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

        let (page_body, page_nav) = page::view(current, &self.snippet_viewer);

        let content = container(
            column![page_header, Element::from(container(page_body).width(Fill))]
                .spacing(24)
                .padding(32)
                .width(700),
        )
        .center_x(Fill);

        let main = scrollable::scrollable(content)
            .width(Fill)
            .height(Fill)
            .id(current.scrollable_id());

        let sidebar_divider = rule::vertical(1).style(Box::new(|theme: &Theme| {
            use iced::gradient::Linear;
            use std::f32::consts::PI;
            let border = theme.neutral.border;
            rule::Style {
                background: Linear::new(PI)
                    .add_stop(0.0, iced::Color::TRANSPARENT)
                    .add_stop(0.5, border.into())
                    .add_stop(1.0, iced::Color::TRANSPARENT)
                    .into(),
                ..rule::Style::default()
            }
        }) as rule::StyleFn<'_, Theme>);

        let mut body = row![].width(Fill).height(Fill);
        if width >= 1024.0 {
            body = body
                .push(Element::from(sidebar))
                .push(sidebar_divider.into());
        }
        body = body.push(Element::from(main));
        if width >= 1280.0
            && let Some(nav) = page_nav
        {
            body = body.push(Element::from(nav));
        }

        let layout = column![header, body].width(Fill).height(Fill);

        toaster(layout.boxed())
            .icons(|kind| {
                let icon = match kind {
                    toast::Kind::Default => return None,
                    toast::Kind::Loading => return Some(spinner().size(16.).into()),
                    toast::Kind::Success => icon::circle_check(),
                    toast::Kind::Info => icon::info(),
                    toast::Kind::Warning => icon::triangle_alert(),
                    toast::Kind::Error => icon::octagon_x(),
                };

                Some(icon.size(16).into())
            })
            .into()
    }
}
