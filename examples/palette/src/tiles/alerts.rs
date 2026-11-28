//! Alerts tile — four alert variants stacked, each exercising a tint's
//! thematic context (info / success / warning / destructive).
//!
//! Exercises: alert (default + destructive), text variants (small),
//! container, column, custom-styled containers for the info / success /
//! warning surfaces (the alert widget itself only carries the default and
//! destructive variants; the rest are thematic containers).

use iced::Fill;
use icedcn::View;
use icedcn::card;
use icedcn::core::{Border, Color, Palette};
use icedcn::theme;
use icedcn::widget::{Column, alert, column, container, text};
use icedcn::{Theme, theme::container::Style};

use crate::Message;

pub fn view() -> iced::Element<'static, Message, Theme> {
    let info = tinted_alert(
        |t: &Theme| t.info,
        "Heads up",
        "This dashboard uses your current notification settings.",
    );
    let success = tinted_alert(
        |t: &Theme| t.success,
        "Saved",
        "Your dashboard layout is autosaved to local state every keystroke.",
    );
    let warning = tinted_alert(
        |t: &Theme| t.warning,
        "Storage is 80% full",
        "Archive older sessions to reclaim space before exports start failing.",
    );

    let destructive = alert::alert()
        .title("Payment failed")
        .description(
            "We couldn't charge the card on file. Update billing to keep this account active.",
        )
        .style(alert::Destructive);

    let body: Column<'_, Message> = column([info, success, warning, View::from(destructive)])
        .spacing(12)
        .width(Fill);

    card::card(
        column([
            card::header("Alerts")
                .description("Each row sits inside its own tint surface.")
                .into(),
            card::content(body).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .gap(0)
    .padding(0)
    .width(Fill)
    .into()
}

/// Build an informational, success, or warning alert with the selected palette.
fn tinted_alert(
    getter: fn(&Theme) -> Palette,
    title: &'static str,
    description: &'static str,
) -> View<'static, Message> {
    let body: Column<'static, Message> = column([
        text::small(title)
            .medium()
            .style(theme::text::Foreground)
            .into(),
        text::small(description).style(theme::text::Muted).into(),
    ])
    .spacing(4)
    .width(Fill);

    container(body)
        .padding([12, 16])
        .width(Fill)
        .style(move |t: &Theme| {
            let p = getter(t);
            Style {
                background: Some(tint_fill(p).into()),
                text_color: Some(p.foreground),
                border: Border {
                    radius: icedcn::radius::LG.into(),
                    width: 1.0,
                    color: tint_border(p),
                },
                ..Style::default()
            }
        })
        .into()
}

/// Use a 10% fill in light themes and a 20% fill in dark themes.
fn tint_fill(p: Palette) -> Color {
    if p.is_dark {
        p.base.alpha(0.20)
    } else {
        p.base.alpha(0.10)
    }
}

/// Tint surface border: 20% of `base` in light, 35% in dark.
fn tint_border(p: Palette) -> Color {
    if p.is_dark {
        p.base.alpha(0.35)
    } else {
        p.base.alpha(0.20)
    }
}
