// RTL is omitted under the browser's component scope.

use iced::widget::text::LineHeight;
use iced::{Fill, Task, Widget};

use icedxdy::Theme;
use icedxdy::card;
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::{accordion, column, container};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Accordion",
    description: "A vertically stacked set of interactive headings that each reveal a section of content.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/accordion-demo.md",
    },
    page::Block::Md("examples/browser/snippets/accordion-installation.md"),
    page::Block::Md("examples/browser/snippets/accordion-usage.md"),
    page::Block::Md("examples/browser/snippets/accordion-composition.md"),
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some(
            "A basic accordion that shows one item at a time. The first item is open by default.",
        ),
        snippet: "examples/browser/snippets/accordion-basic.md",
    },
    page::Block::Example {
        id: "multiple",
        title: "Multiple",
        description: Some(
            "Use `.mode(accordion::Mode::Multiple)` to keep several items open at once.",
        ),
        snippet: "examples/browser/snippets/accordion-multiple.md",
    },
    page::Block::Example {
        id: "disabled",
        title: "Disabled",
        description: Some("Set `.disabled(true)` on an item to disable its trigger."),
        snippet: "examples/browser/snippets/accordion-disabled.md",
    },
    page::Block::Example {
        id: "borders",
        title: "Borders",
        description: Some(
            "Add a border to the accordion and inset each item to keep its divider edge to edge.",
        ),
        snippet: "examples/browser/snippets/accordion-borders.md",
    },
    page::Block::Example {
        id: "card",
        title: "Card",
        description: Some("Place the accordion in `card::content(...)` below the card header."),
        snippet: "examples/browser/snippets/accordion-card.md",
    },
    page::Block::Md("examples/browser/snippets/accordion-api-reference.md"),
];

/// The accordion page. Each example keeps its own open set in its widget
/// tree, seeded from `.default_value(...)`.
pub struct AccordionPage {
    state: page::State,
    wide: bool,
}

impl AccordionPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: true,
        })
    }
}

fn build_accordion<'a>(
    mode: accordion::Mode,
    open: &[&'a str],
    items: impl IntoIterator<Item = (&'a str, &'a str, &'a str, bool)>,
) -> accordion::Accordion<'a, Message> {
    accordion(items.into_iter().map(|(id, trigger, body, disabled)| {
        accordion::item(id, trigger)
            .content(body)
            .disabled(disabled)
    }))
    .mode(mode)
    .default_value(open.iter().copied())
}

impl page::Page for AccordionPage {
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

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn window_width(&mut self, width: f32) {
        self.wide = width >= 768.;
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: iced::Padding::ZERO,
            ..page::Backdrop::default()
        }
    }

    fn docs_preview(&self, id: &str, outer: &Theme) -> iced::Element<'_, Message, Theme> {
        // The embedded preview has a smaller width cap than the standalone examples.
        let content = container(self.themed_preview(id, outer)).width(Fill.max(384));
        let height = match (id, self.wide) {
            ("multiple", true) => 480,
            ("multiple", false) => 576,
            ("borders", true) => 320,
            ("borders", false) => 384,
            ("card", true) => 448,
            ("card", false) => 512,
            _ => 300,
        };
        // The shell contributes 32 px of exterior padding on narrow windows.
        let gutter = if self.wide { 40 } else { 32 };
        container(content)
            .center_x(Fill)
            .height(height)
            .padding(iced::Padding::default().top(40).left(gutter).right(gutter))
            .clip(true)
            .into()
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let single = accordion::Mode::Single { collapsible: true };
        let multiple = accordion::Mode::Multiple;

        match id {
            "demo" => container(build_accordion(
                single,
                &["shipping"],
                [
                    (
                        "shipping",
                        "What are your shipping options?",
                        "We offer standard (5-7 days), express (2-3 days), and overnight shipping. Free shipping on international orders.",
                        false,
                    ),
                    (
                        "returns",
                        "What is your return policy?",
                        "Returns accepted within 30 days. Items must be unused and in original packaging. Refunds processed within 5-7 business days.",
                        false,
                    ),
                    (
                        "support",
                        "How can I contact customer support?",
                        "Reach us via email, live chat, or phone. We respond within 24 hours during business days.",
                        false,
                    ),
                ],
            ))
            .width(Fill.max(512))
            .into(),

            "basic" => container(build_accordion(
                single,
                &["item-1"],
                [
                    (
                        "item-1",
                        "How do I reset my password?",
                        "Click on 'Forgot Password' on the login page, enter your email address, and we'll send you a link to reset your password. The link will expire in 24 hours.",
                        false,
                    ),
                    (
                        "item-2",
                        "Can I change my subscription plan?",
                        "Yes, you can upgrade or downgrade your plan at any time from your account settings. Changes will be reflected in your next billing cycle.",
                        false,
                    ),
                    (
                        "item-3",
                        "What payment methods do you accept?",
                        "We accept all major credit cards, PayPal, and bank transfers. All payments are processed securely through our payment partners.",
                        false,
                    ),
                ],
            ))
            .width(Fill.max(512))
            .into(),

            "multiple" => container(build_accordion(
                multiple,
                &["notifications"],
                [
                    (
                        "notifications",
                        "Notification Settings",
                        "Manage how you receive notifications. You can enable email alerts for updates or push notifications for mobile devices.",
                        false,
                    ),
                    (
                        "privacy",
                        "Privacy & Security",
                        "Control your privacy settings and security preferences. Enable two-factor authentication, manage connected devices, review active sessions, and configure data sharing preferences. You can also download your data or delete your account.",
                        false,
                    ),
                    (
                        "billing",
                        "Billing & Subscription",
                        "View your current plan, payment history, and upcoming invoices. Update your payment method, change your subscription tier, or cancel your subscription.",
                        false,
                    ),
                ],
            ))
            .width(Fill.max(512))
            .into(),

            "disabled" => container(build_accordion(
                single,
                &[],
                [
                    (
                        "item-1",
                        "Can I access my account history?",
                        "Yes, you can view your complete account history including all transactions, plan changes, and support tickets in the Account History section of your dashboard.",
                        false,
                    ),
                    (
                        "item-2",
                        "Premium feature information",
                        "This section contains information about premium features. Upgrade your plan to access this content.",
                        true,
                    ),
                    (
                        "item-3",
                        "How do I update my email address?",
                        "You can update your email address in your account settings. You'll receive a verification email at your new address to confirm the change.",
                        false,
                    ),
                ],
            ))
            .width(Fill)
            .into(),

            "borders" => {
                let items = [
                    (
                        "billing",
                        "How does billing work?",
                        "We offer monthly and annual subscription plans. Billing is charged at the beginning of each cycle, and you can cancel anytime. All plans include automatic backups, 24/7 support, and unlimited team members.",
                    ),
                    (
                        "security",
                        "Is my data secure?",
                        "Yes. We use end-to-end encryption, SOC 2 Type II compliance, and regular third-party security audits. All data is encrypted at rest and in transit using industry-standard protocols.",
                    ),
                    (
                        "integration",
                        "What integrations do you support?",
                        "We integrate with 500+ popular tools including Slack, Zapier, Salesforce, HubSpot, and more. You can also build custom integrations using our REST API and webhooks.",
                    ),
                ];
                let inner = accordion(items.into_iter().map(|(id, trigger, body)| {
                    accordion::item(id, trigger).content(body).inset(16.0)
                }))
                .mode(single)
                .default_value(["billing"]);

                container(inner)
                    .width(Fill.max(512))
                    .padding(1)
                    .style(|theme: &Theme| Style {
                            background: None,
                            border: Border {
                                width: 1.0,
                                color: theme.neutral.border,
                                radius: icedxdy::radius::LG.into(),
                            },
                            ..Style::default()
                    })
                    .into()
            }

            "card" => {
                let inner = build_accordion(
                    single,
                    &["plans"],
                    [
                        (
                            "plans",
                            "What subscription plans do you offer?",
                            "We offer three subscription tiers: Starter ($9/month), Professional ($29/month), and Enterprise ($99/month). Each plan includes increasing storage limits, API access, priority support, and team collaboration features.",
                            false,
                        ),
                        (
                            "billing",
                            "How does billing work?",
                            "Billing occurs automatically at the start of each billing cycle. We accept all major credit cards, PayPal, and ACH transfers for enterprise customers. You'll receive an invoice via email after each payment.",
                            false,
                        ),
                        (
                            "cancel",
                            "How do I cancel my subscription?",
                            "You can cancel your subscription anytime from your account settings. There are no cancellation fees or penalties. Your access will continue until the end of your current billing period.",
                            false,
                        ),
                    ],
                );

                let description = card::description("Common questions about your account, plans, payments and cancellations.")
                    .line_height(LineHeight::Absolute(20.into()));
                let header = card::header("Subscription & Billing").description(description);
                let body = card::content(inner);
                let stack = column![header, body].spacing(16).padding([16, 0]);

                card(stack).width(Fill.max(384)).boxed()
            }

            _ => unreachable!("unknown accordion preview id: {id}"),
        }
    }
}
