//! Frequently asked questions grouped into working tabs.

use iced::Fill;
use icedcn::widget::{accordion, column, tabs};
use icedcn::{Theme, card};

use crate::{Message, State};

pub fn view(_: &State) -> iced::Element<'static, Message, Theme> {
    card(column![
        card::header("FAQ").description("A little help with your workspace."),
        card::content(tabs(|t| {
            column![
                t.list([t.trigger("general", "General"), t.trigger("billing", "Billing")]),
                t.content("general", accordion![
                    ("Can I invite my team?", "Yes. Invite teammates from the workspace settings and choose the access each person needs."),
                    ("Where can I change my profile?", "Open account settings to update your name, avatar, and notification preferences."),
                ].default_value(0)),
                t.content("billing", accordion![
                    ("When are invoices available?", "An invoice is available in Billing after each successful payment."),
                    ("Can I change my plan?", "Choose another plan in Billing. Review the effective date and total before confirming."),
                ].default_value(0)),
            ].spacing(12).width(Fill).into()
        }).default_value("general").width(Fill).list_width(Fill)),
    ].spacing(16)).width(Fill).into()
}
