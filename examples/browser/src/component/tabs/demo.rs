use iced::Fill;
use icedcn::widget::{column, tabs, text};
use icedcn::{View, card};

const PANELS: [(&str, &str, &str); 4] = [
    (
        "Overview",
        "View your key metrics and recent project activity. Track progress across all your active projects.",
        "You have 12 active projects and 3 pending tasks.",
    ),
    (
        "Analytics",
        "Track performance and user engagement metrics. Monitor trends and identify growth opportunities.",
        "Page views are up 25% compared to last month.",
    ),
    (
        "Reports",
        "Generate and download your detailed reports. Export data in multiple formats for analysis.",
        "You have 5 reports ready and available to export.",
    ),
    (
        "Settings",
        "Manage your account preferences and options. Customize your experience to fit your needs.",
        "Configure notifications, security, and themes.",
    ),
];

pub fn view<'a, M: Clone + 'static>() -> View<'a, M> {
    tabs(|t| {
        let list = t.list(PANELS.map(|(title, _, _)| t.trigger(title, title)));
        let panels = PANELS.map(|(title, description, detail)| {
            t.content(
                title,
                card(column![
                    card::header(title).description(description),
                    card::content(text::muted(detail)),
                ])
                .width(Fill),
            )
        });
        column![list, column(panels).width(Fill)]
            .spacing(8)
            .width(Fill)
            .into()
    })
    .default_value("Overview")
    .width(400)
    .into()
}
