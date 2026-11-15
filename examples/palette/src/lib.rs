//! Interactive palette preview for icedcn. Use the side panel to customize the theme and see
//! how its colors apply to dashboard cards, alerts, and actions.
//!
//! The root scrollable opts into both axes; tiles that genuinely need
//! horizontal overflow (transactions, etc.) wrap their own.

use iced::{Element, clipboard};

use std::collections::HashSet;
use std::f32::consts::PI;

use iced::gradient::Linear;
use iced::widget::Id;
use iced::widget::stack;
use iced::{Bottom, Fill, Right, Task};
use iced_widget::button::Status;
use icedcn::Theme;
use icedcn::core::{Background, Border};
use icedcn::theme;
use icedcn::theme::container::Style;
use icedcn::widget::Size;
use icedcn::widget::chart::Data;
use icedcn::widget::qr_code;
use icedcn::widget::scrollable::{Direction, scrollbar};
use icedcn::widget::{button, column, container, dialog, row, scrollable, text};

use crate::tiles::{
    card_overview, chart, contribution_history, dividend_income, power_usage, stock_performance,
};

pub mod chrome;
pub mod customize;
pub mod dashboard;
pub mod icon;
pub mod sidebar;
pub mod tiles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewPage {
    One,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceivingMethod {
    BankTransfer,
    Paypal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadePosition {
    Open,
    Half,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KitchenMode {
    Hue,
    Color,
    Ambient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KitchenScene {
    Cooking,
    Dining,
    Nightlight,
    Focus,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetMode(Mode),
    SetPreviewPage(PreviewPage),
    ToggleConnection(bool),
    SetReceivingMethod(ReceivingMethod),
    SetShadePosition(ShadePosition),
    SetKitchenMode(KitchenMode),
    SetKitchenScene(KitchenScene),
    SetPayoutThreshold(f32),
    TransferAmountChanged(String),
    ToggleNotificationSetting(String),
    SetAllNotificationSettings(bool),
    SetBase(String),
    SetAccent(String),
    SetChart(String),
    SetMenu(String),
    Reset,
    NextPreset,
    ToggleControls,
    CodeOpen(bool),
    CopyCode,
    CodeCopied(Result<(), String>),
    SaveForm,
    CancelForm,
    NotificationsOpened(Vec<String>),
    ArmDelete,
    Delete,
    Deploy,
    NotificationCleared,
    Noop,
}

pub struct State {
    pub mode: Mode,
    pub traffic: Data,
    pub contributions: Data,
    pub power: Data,
    pub dividends: Data,
    pub stocks: Data,
    pub spending: Data,

    pub preview_page: PreviewPage,
    pub connected: bool,
    pub receiving_method: ReceivingMethod,
    pub shade_position: ShadePosition,
    pub kitchen_mode: KitchenMode,
    pub kitchen_scene: KitchenScene,
    pub payout_threshold: f32,
    pub transfer_amount: String,
    pub notification_settings: HashSet<String>,
    pub open_notifications: Vec<String>,
    pub armed: bool,
    pub deploying: bool,
    pub qr_connect: Option<qr_code::Data>,
    pub settings: customize::Settings,
    pub controls_open: bool,
    pub code_open: bool,
    pub copied: bool,
    copy_error: Option<String>,
    code: String,
    preset: usize,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let mode = if theme::is_dark_mode() {
            Mode::Dark
        } else {
            Mode::Light
        };

        Self::embedded(mode)
    }

    /// Initialize an embedded designer without network requests or global font changes.
    pub fn embedded(mode: Mode) -> (Self, Task<Message>) {
        (Self::with_mode(mode), Task::none())
    }

    pub fn dark() -> Self {
        Self::with_mode(Mode::Dark)
    }

    fn with_mode(mode: Mode) -> Self {
        Self {
            mode,
            traffic: chart::data("Blue", mode),
            contributions: contribution_history::data("Blue", mode),
            power: power_usage::data("Blue", mode),
            dividends: dividend_income::data("Blue", mode),
            stocks: stock_performance::data("Blue", mode),
            spending: card_overview::data("Blue", mode),

            preview_page: PreviewPage::One,
            connected: true,
            receiving_method: ReceivingMethod::BankTransfer,
            shade_position: ShadePosition::Half,
            kitchen_mode: KitchenMode::Hue,
            kitchen_scene: KitchenScene::Dining,
            payout_threshold: 2500.0,
            transfer_amount: "$ 1,200.00".into(),
            notification_settings: ["transactions".to_owned(), "security".to_owned()]
                .into_iter()
                .collect(),
            open_notifications: vec!["welcome".to_owned()],
            armed: false,
            deploying: false,
            qr_connect: qr_code::Data::new("https://ledger.app/connect/jd-4829").ok(),
            settings: customize::Settings::default(),
            controls_open: false,
            code_open: false,
            copied: false,
            copy_error: None,
            code: customize::Settings::default().source(mode),
            preset: 0,
        }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        let design_changed = matches!(
            &msg,
            Message::SetMode(_)
                | Message::SetBase(_)
                | Message::SetAccent(_)
                | Message::SetChart(_)
                | Message::SetMenu(_)
                | Message::Reset
                | Message::NextPreset
        );
        match msg {
            Message::SetMode(m) => self.mode = m,
            Message::SetBase(value) => {
                if let Some(value) = customize::BASES.iter().find(|name| **name == value) {
                    self.settings.base = value;
                }
            }
            Message::SetAccent(value) => {
                if let Some(value) = customize::ACCENTS.iter().find(|name| **name == value) {
                    self.settings.accent = value;
                }
            }
            Message::SetChart(value) => {
                if let Some(value) = customize::ACCENTS.iter().find(|name| **name == value) {
                    self.settings.chart = value;
                }
            }
            Message::SetMenu(value) => {
                if let Some(value) = customize::MENUS.iter().find(|name| **name == value) {
                    self.settings.menu = value;
                }
            }
            Message::Reset => {
                self.settings = customize::Settings {
                    font: self.settings.font,
                    ..Default::default()
                };
                self.preset = 0;
            }
            Message::NextPreset => {
                self.preset = (self.preset + 1) % 5;
                self.settings.base = customize::BASES[self.preset % customize::BASES.len()];
                self.settings.accent = customize::ACCENTS[self.preset + 1];
                self.settings.chart = self.settings.accent;
                self.settings.menu = customize::MENUS[self.preset % 2];
            }
            Message::ToggleControls => self.controls_open = !self.controls_open,
            Message::CodeOpen(open) => self.code_open = open,
            Message::CopyCode => {
                return clipboard::write(self.code.clone()).map(|result| {
                    Message::CodeCopied(
                        result
                            .map_err(|_| "Clipboard unavailable. Select and copy the code.".into()),
                    )
                });
            }
            Message::CodeCopied(result) => {
                self.copied = result.is_ok();
                self.copy_error = result.err();
            }
            Message::SetPreviewPage(page) => self.preview_page = page,
            Message::ToggleConnection(v) => self.connected = v,
            Message::SetReceivingMethod(v) => self.receiving_method = v,
            Message::SetShadePosition(v) => self.shade_position = v,
            Message::SetKitchenMode(v) => self.kitchen_mode = v,
            Message::SetKitchenScene(v) => self.kitchen_scene = v,
            Message::SetPayoutThreshold(v) => self.payout_threshold = v,
            Message::TransferAmountChanged(v) => self.transfer_amount = v,
            Message::ToggleNotificationSetting(id) => {
                if !self.notification_settings.remove(&id) {
                    self.notification_settings.insert(id);
                }
            }
            Message::SetAllNotificationSettings(enabled) => {
                self.notification_settings.clear();
                if enabled {
                    self.notification_settings.extend(
                        ["transactions", "security", "goals", "market"]
                            .into_iter()
                            .map(str::to_owned),
                    );
                }
            }
            Message::SaveForm | Message::CancelForm => {}
            Message::NotificationsOpened(open) => self.open_notifications = open,
            Message::ArmDelete => self.armed = !self.armed,
            Message::Delete => {
                self.armed = false;
            }
            Message::Deploy => self.deploying = !self.deploying,
            Message::NotificationCleared => {}
            Message::Noop => {}
        }
        if design_changed {
            self.refresh();
        }
        Task::none()
    }

    fn refresh(&mut self) {
        self.traffic = chart::data(self.settings.chart, self.mode);
        self.contributions = contribution_history::data(self.settings.chart, self.mode);
        self.power = power_usage::data(self.settings.chart, self.mode);
        self.dividends = dividend_income::data(self.settings.chart, self.mode);
        self.stocks = stock_performance::data(self.settings.chart, self.mode);
        self.spending = card_overview::data(self.settings.chart, self.mode);

        self.code = self.settings.source(self.mode);
        self.copied = false;
        self.copy_error = None;
    }

    /// Keep exported typography and chart labels in sync with the host's selected font.
    pub fn set_font(&mut self, font: iced::Font) {
        if self.settings.font != font {
            self.settings.font = font;
            self.refresh();
        }
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    /// The current design for the host to apply while Create is visible.
    pub fn theme(&self) -> Theme {
        self.settings.theme(self.mode)
    }

    pub fn view(&self) -> iced::Element<'_, Message, Theme> {
        column![chrome::view(self), self.view_embedded(1280.0)]
            .height(Fill)
            .into()
    }

    /// Customizer and dashboard, without an outer navigation bar.
    fn designer_body(&self, width: f32) -> iced::Element<'_, Message, Theme> {
        let preview = container(
            scrollable(dashboard::view(self))
                .id(Id::new("create-preview-scroll-v2"))
                .direction(Direction::Both {
                    vertical: scrollbar(),
                    horizontal: scrollbar(),
                })
                .width(Fill)
                .height(Fill),
        )
        .width(Fill)
        .height(Fill)
        .clip(true)
        .style(preview_frame);

        let preview = stack![
            Element::from(preview),
            Element::from(
                container(page_switcher(self.preview_page))
                    .width(Fill)
                    .height(Fill)
                    .padding(12)
                    .align_x(Right)
                    .align_y(Bottom)
            )
        ]
        .width(Fill)
        .height(Fill);

        let compact = width < 760.0;
        let body: Element<'_, Message, Theme> = if compact {
            let controls = column![
                button(if self.controls_open {
                    "Hide customization"
                } else {
                    "Customize theme"
                })
                .style(button::Outline)
                .width(Fill)
                .on_press(Message::ToggleControls),
            ]
            .width(Fill);
            let controls = if self.controls_open {
                controls.push(
                    container(sidebar::view(self, true))
                        .width(Fill)
                        .height(300)
                        .into(),
                )
            } else {
                controls
            };
            column![controls.spacing(8), preview]
                .spacing(12)
                .width(Fill)
                .height(Fill)
                .into()
        } else {
            row![sidebar::view(self, false), preview]
                .spacing(24)
                .height(Fill)
                .width(Fill)
                .into()
        };
        let designer = container(body)
            .padding(if compact { 12 } else { 24 })
            .width(Fill)
            .height(Fill)
            .style(section_soft);

        designer.into()
    }

    /// Customizer with its export dialog.
    pub fn view_embedded(&self, width: f32) -> Element<'_, Message, Theme> {
        // The dialog trigger owns the normal page so its overlay stays inside the host tree.
        dialog(
            move |_| {
                container(self.designer_body(width))
                    .width(Fill)
                    .height(Fill)
                    .into()
            },
            |_| {
                column![
                    dialog::header(
                        "Your theme",
                        "Copy this Rust configuration into your icedcn app."
                    ),
                    scrollable(text::small(self.code.as_str()).font(iced::Font::MONOSPACE))
                        .height(360)
                        .width(Fill)
                        .direction(Direction::Both {
                            vertical: scrollbar(),
                            horizontal: scrollbar()
                        }),
                    button(if self.copied { "Copied" } else { "Copy code" })
                        .on_press(Message::CopyCode),
                    text::xs(self.copy_error.as_deref().unwrap_or("")),
                ]
                .spacing(16)
                .width(Fill)
                .into()
            },
        )
        .max_width(760.0)
        .open(self.code_open)
        .on_open_change(Message::CodeOpen)
        .into()
    }
}

fn page_switcher(page: PreviewPage) -> iced::Element<'static, Message, Theme> {
    let one = page_button("01", PreviewPage::One, page == PreviewPage::One);
    let two = page_button("02", PreviewPage::Two, page == PreviewPage::Two);

    container(row![one, two].spacing(4).align_y(iced::Center))
        .padding(4)
        .style(switcher_style)
        .into()
}

fn page_button(
    label: &'static str,
    page: PreviewPage,
    active: bool,
) -> iced::Element<'static, Message, Theme> {
    let button = button(label)
        .style(button::Ghost)
        .size(Size::Sm)
        .width(36.0)
        .style(page_button_style(active))
        .on_press(Message::SetPreviewPage(page));

    button.into()
}

fn preview_frame(theme: &Theme) -> Style {
    Style {
        background: Some(theme.neutral.base.into()),
        border: Border {
            radius: 18.0.into(),
            width: 1.0,
            color: theme.neutral.foreground.alpha(0.10),
        },
        ..Style::default()
    }
}

fn section_soft(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Gradient(
            Linear::new(PI)
                .add_stop(0.0, theme.neutral.base.into())
                .add_stop(1.0, theme.code.base.alpha(0.40).into())
                .into(),
        )),
        ..Style::default()
    }
}

fn switcher_style(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.neutral.foreground),
        background: Some(theme.card.base.scale_alpha(0.90).into()),
        border: Border {
            radius: 14.0.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

fn page_button_style(active: bool) -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(move |theme, status| {
        let mut style = iced_widget::button::Style {
            text_color: if active {
                theme.accent.foreground.into()
            } else {
                theme.muted.foreground.into()
            },
            border: Border {
                radius: 10.0.into(),
                ..Border::default()
            }
            .into(),
            ..Default::default()
        };

        if active || matches!(status, Status::Hovered) {
            style.background = Some(theme.accent.base.into());
        }

        style
    })
}
