//! Decorative desktop chrome for the runnable application gallery.
use iced::alignment::Horizontal;
use iced::{Element, Fill, Vector};
use icedcn::core::{Border, Radius, Shadow};
use icedcn::theme::container::Style;
use icedcn::widget::{column, container, row, space, text};
use icedcn::{Color, Slot, Theme};

use crate::icon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Platform {
    Mac,
    Windows,
    Other,
    Mobile,
}

impl Default for Platform {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window().map_or(Self::Other, |window| {
                let navigator = window.navigator();
                Self::detect(
                    &navigator.user_agent().unwrap_or_default(),
                    &navigator.platform().unwrap_or_default(),
                    navigator.max_touch_points(),
                )
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if cfg!(target_os = "macos") {
                Self::Mac
            } else if cfg!(target_os = "windows") {
                Self::Windows
            } else if cfg!(any(target_os = "ios", target_os = "android")) {
                Self::Mobile
            } else {
                Self::Other
            }
        }
    }
}

impl Platform {
    #[cfg(any(target_arch = "wasm32", test))]
    fn detect(agent: &str, platform: &str, touch_points: i32) -> Self {
        let agent = agent.to_ascii_lowercase();
        let platform = platform.to_ascii_lowercase();
        let mac = platform.starts_with("mac") || agent.contains("macintosh");
        if ["android", "iphone", "ipad", "ipod", "mobile"]
            .iter()
            .any(|token| agent.contains(token))
            || (mac && touch_points > 1)
        {
            Self::Mobile
        } else if platform.starts_with("win") || agent.contains("windows") {
            Self::Windows
        } else if mac {
            Self::Mac
        } else {
            Self::Other
        }
    }

    pub(super) fn view<'a, Message: 'a>(
        self,
        width: f32,
        title: String,
        content: impl Into<Slot<'a, Message>>,
    ) -> Element<'a, Message, Theme> {
        let framed = width >= 900. && self != Self::Mobile;
        // Keep the content's tree position stable when resizing through the breakpoint.
        let titlebar = container(self.titlebar(title))
            .id("example-window-titlebar")
            .width(Fill)
            .height(if framed { 40 } else { 0 })
            .clip(true);
        let frame = container(column![titlebar, content.into()].width(Fill).height(Fill))
            .id("example-window")
            .width(Fill)
            .max_width(if framed { 1080. } else { f32::INFINITY })
            .height(Fill)
            .max_height(if framed { 780. } else { f32::INFINITY })
            .padding(if framed { 1 } else { 0 })
            .style(move |theme| {
                if !framed {
                    return Style::default();
                }
                Style {
                    background: Some(theme.neutral.base.into()),
                    border: Border {
                        color: theme.neutral.border,
                        width: 1.,
                        radius: 12.0.into(),
                    },
                    shadow: Shadow {
                        color: Color::BLACK.alpha(if theme.neutral.is_dark { 0.28 } else { 0.10 }),
                        offset: Vector::new(0., 8.),
                        blur_radius: 28.,
                    },
                    ..Style::default()
                }
            });
        container(frame)
            .padding(if framed { [20, 32] } else { [0, 0] })
            .center(Fill)
            .into()
    }

    fn titlebar<'a, Message: 'a>(self, title: String) -> Element<'a, Message, Theme> {
        let title = text::small(title)
            .size(12)
            .medium()
            .ellipsis(text::Ellipsis::End)
            .width(Fill);
        let contents: Element<'a, Message, Theme> = match self {
            Self::Mac => row![
                row![
                    dot(iced::color!(0xff6058)),
                    dot(iced::color!(0xfebc2e)),
                    dot(iced::color!(0x28c840)),
                ]
                .spacing(8)
                .width(96),
                title.align_x(Horizontal::Center),
                space::horizontal().width(96),
            ]
            .width(Fill)
            .into(),
            Self::Windows => row![
                title,
                row![
                    icon::minus().size(13),
                    icon::square().size(12),
                    icon::x().size(14),
                ]
                .spacing(24),
            ]
            .spacing(24)
            .width(Fill)
            .into(),
            Self::Other | Self::Mobile => row![
                space::horizontal().width(24),
                title.align_x(Horizontal::Center),
                container(icon::x().size(14))
                    .width(24)
                    .align_x(Horizontal::Right),
            ]
            .width(Fill)
            .into(),
        };
        container(contents)
            .padding([0, 16])
            .width(Fill)
            .height(40)
            .center_y(Fill)
            .style(|theme| Style {
                background: Some(theme.muted.base.into()),
                text_color: Some(theme.muted.foreground),
                border: Border {
                    radius: Radius {
                        top_left: 11.,
                        top_right: 11.,
                        ..Radius::default()
                    },
                    ..Border::default()
                },
                ..Style::default()
            })
            .into()
    }
}

fn dot<'a, Message: 'a>(color: impl Into<Color>) -> Element<'a, Message, Theme> {
    let color = color.into();
    container(space::horizontal().width(10).height(10))
        .width(10)
        .height(10)
        .style(move |_| Style {
            background: Some(color.into()),
            border: Border::default().rounded(5),
            ..Style::default()
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::widget::Id;
    use iced_test::core::Settings;
    use iced_test::selector::{self, Target};
    use iced_test::simulator::Simulator;
    use icedcn::widget::{button, input};

    #[test]
    fn browser_platform_recognizes_mobile_before_desktop_tokens() {
        for (agent, platform, touch, expected) in [
            (
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
                "MacIntel",
                0,
                Platform::Mac,
            ),
            (
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
                "Win32",
                10,
                Platform::Windows,
            ),
            (
                "Mozilla/5.0 (X11; Linux x86_64)",
                "Linux x86_64",
                0,
                Platform::Other,
            ),
            (
                "Mozilla/5.0 (Linux; Android 14)",
                "Linux armv8l",
                5,
                Platform::Mobile,
            ),
            (
                "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)",
                "iPhone",
                5,
                Platform::Mobile,
            ),
            (
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
                "MacIntel",
                5,
                Platform::Mobile,
            ),
            ("", "", 0, Platform::Other),
        ] {
            assert_eq!(Platform::detect(agent, platform, touch), expected);
        }
    }

    #[test]
    fn frame_respects_available_bounds_and_mobile_has_no_chrome_space() {
        for (width, height, platform, framed) in [
            (1440., 1000., Platform::Mac, true),
            (1000., 600., Platform::Windows, true),
            (900., 420., Platform::Other, true),
            (899., 700., Platform::Mac, false),
            (375., 600., Platform::Windows, false),
            (1200., 800., Platform::Mobile, false),
        ] {
            let view = platform.view(width, "Receipts".into(), button("Action").on_press(()));
            let mut ui =
                Simulator::with_size(Settings::default(), iced::Size::new(width, height), view);
            let bounds = ui.find(Id::from("example-window")).unwrap().bounds();
            let bar = ui
                .find(Id::from("example-window-titlebar"))
                .unwrap()
                .bounds();
            assert_eq!(
                bounds.width,
                if framed {
                    (width - 64.).min(1080.)
                } else {
                    width
                }
            );
            assert_eq!(
                bounds.height,
                if framed {
                    (height - 40.).min(780.)
                } else {
                    height
                }
            );
            assert_eq!(bounds.x, (width - bounds.width) / 2.);
            assert_eq!(bounds.y, (height - bounds.height) / 2.);
            assert_eq!(bar.height, if framed { 40. } else { 0. });
            ui.click("Action").unwrap();
            assert_eq!(ui.into_messages().count(), 1);
        }
    }

    #[test]
    fn resizing_preserves_input_focus_and_chrome_adds_no_tab_stops() {
        let view = |width| Platform::Mac.view(width, "Receipts".into(), input("Name").id("name"));
        let mut ui: Simulator<'_, (), Theme, iced::Renderer> = Simulator::with_size(
            Settings::default(),
            iced::Size::new(1200., 800.),
            view(1200.),
        );
        ui.click(Id::from("name")).unwrap();
        ui = ui.resize((700., 600.)).rebuild(view(700.));
        assert!(
            matches!(ui.find(selector::is_focused()), Ok(Target::Focusable { id: Some(id), .. }) if id == Id::from("name"))
        );
        ui = ui.resize((1200., 800.)).rebuild(view(1200.));
        assert!(
            matches!(ui.find(selector::is_focused()), Ok(Target::Focusable { id: Some(id), .. }) if id == Id::from("name"))
        );
        let mut focusables = 0;
        let _ = ui.find(|candidate: selector::Candidate<'_>| {
            if let selector::Candidate::Focusable { .. } = candidate {
                focusables += 1;
            }
            None::<()>
        });
        assert_eq!(focusables, 1);
    }
}
