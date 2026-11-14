//! Card recipe regressions: spacing rebuilds, constrained frames, and lazy headers.
use browser::component::card::CardPage;
use browser::{Message, docs, page};
use iced::{Element, Font, Size};
use iced_test::Simulator;
use iced_test::core::Settings;
use icedcn::widget::{button, text};
use icedcn::{Compose, Inherited, Slot, Theme, card};

fn settings() -> Settings {
    Settings {
        font: Font::new("Geist"),
        text_size: 14.into(),
        ..Settings::default()
    }
}

fn preview<'a>(page: &'a dyn page::Page, id: &str, width: f32) -> Simulator<'a, Message, Theme> {
    Simulator::with_size(settings(), Size::new(width, 700.), page.preview(id))
}

#[test]
fn spacing_rebuilds_keep_input_values_and_ignore_empty_selection() {
    let page = CardPage::new(0);
    let mut ui = preview(&*page, "spacing", 384.);
    ui.click("Email").expect("email label focuses its input");
    ui.typewrite("qa@example.com");
    ui.click("Password")
        .expect("password label focuses its input");
    ui.typewrite("secret");
    for (choice, inset) in [
        ("20px", 20.),
        ("24px", 24.),
        ("32px", 32.),
        ("32px", 32.),
        ("16px", 16.),
    ] {
        ui.click(choice).expect("spacing control");
        ui.find("qa@example.com")
            .expect("email survives composition");
        ui.find("••••••")
            .expect("password stays masked and retained");
        assert!(ui.find("secret").is_err());
        let header = ui.find("Login to your account").expect("header").bounds();
        let email = ui.find("Email").expect("email label").bounds();
        assert!((header.x - inset).abs() <= 1., "{choice}: {header:?}");
        assert!((email.x - header.x).abs() <= 1., "shared inset");
    }
}

#[test]
fn constrained_cards_keep_content_inside_the_declared_frames() {
    let page = CardPage::new(0);
    for width in [558., 260.] {
        for (id, height, labels) in [
            (
                "demo",
                480.,
                &[
                    "Login to your account",
                    "Sign Up",
                    "Email",
                    "Password",
                    "Forgot your password?",
                    "Login",
                    "Login with Google",
                ][..],
            ),
            ("small", 384., &["Small Card", "Action"][..]),
            (
                "spacing",
                544.,
                &["16px", "20px", "24px", "32px", "Login with Google"][..],
            ),
            (
                "edge-to-edge",
                384.,
                &["Terms of Service", "Decline", "Accept"][..],
            ),
            (
                "image",
                512.,
                &["Design systems meetup", "Featured", "View Event"][..],
            ),
        ] {
            let theme = docs::theme(false);
            let mut ui = Simulator::with_size(
                settings(),
                Size::new(width, height - 80.),
                page.docs_preview(id, &theme),
            );
            ui.draw(&theme);
            for label in labels {
                let bounds = ui.find(*label).expect("complete label").bounds();
                assert!(
                    bounds.x >= 0. && bounds.x + bounds.width <= width + 0.1,
                    "{width} {id} {label}: {bounds:?}"
                );
                assert!(
                    bounds.y >= 0. && bounds.y + bounds.height <= height - 79.9,
                    "{width} {id} {label}: {bounds:?}"
                );
            }
        }
    }
}

#[test]
fn header_receives_disabled_inheritance_before_composing_its_action() {
    let header = card::header(Slot::from(text::small("Title")))
        .description("Description")
        .action(button("Action").on_press(()));
    let mut slot: Slot<'_, ()> = header.into();
    slot.inherit(&Inherited::default().with_disabled(true));
    let mut ui = Simulator::with_size(settings(), Size::new(384., 100.), Element::from(slot));
    ui.click("Action").expect("visible action");
    assert_eq!(
        ui.drain().count(),
        0,
        "a disabled ancestor reaches the action"
    );
}

#[test]
fn footer_keeps_its_child_in_the_disabled_cascade() {
    let mut footer = card::footer(button("Save").on_press(()));
    footer.inherit(&Inherited::default().with_disabled(true));
    let mut ui = Simulator::with_size(settings(), Size::new(384., 100.), Element::from(footer));
    ui.click("Save").expect("visible footer action");
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn root_and_content_preserve_disabled_state_through_mapped_slots() {
    use icedcn::widget::{column, container};
    let body: Slot<'_, u8> = card::content(container(button("Body").on_press(1))).into();
    let mut root: Slot<'_, u16> = card(column![
        card::header("Title").action(button("Action").on_press(2u16)),
        body.map(u16::from),
        card::footer_row([button("Save").on_press(3u16).into()]),
    ])
    .width(300)
    .into();
    root.inherit(&Inherited::default().with_disabled(true));
    let mut ui = Simulator::with_size(settings(), Size::new(300., 300.), root.compose());
    for label in ["Action", "Body", "Save"] {
        ui.click(label).expect("visible disabled descendant");
    }
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn root_spacing_sizes_all_parts_without_changing_inner_column_gaps() {
    use iced::widget::{Id, Space};
    use icedcn::Size;
    use icedcn::widget::{column, container};
    for (size, spacing, title_height) in [
        (Size::Default, None, 22.),
        (Size::Sm, None, 19.25),
        (Size::Default, Some(32.), 22.),
        (Size::Sm, Some(24.), 19.25),
    ] {
        let inset = spacing.unwrap_or(if size == Size::Sm { 12. } else { 16. });
        let first = Id::unique();
        let second = Id::unique();
        let foot = Id::unique();
        let root_id = Id::unique();
        let footer: Slot<'_, u8> =
            card::footer(container(text::small("Footer")).id(foot.clone())).into();
        let mut root = card(
            column![
                card::header("Title"),
                card::content(
                    container(
                        column![
                            container(Space::new())
                                .id(first.clone())
                                .width(iced::Fill)
                                .height(10),
                            container(Space::new())
                                .id(second.clone())
                                .width(iced::Fill)
                                .height(10),
                        ]
                        .width(iced::Fill)
                    )
                    .width(iced::Fill)
                ),
                container(footer.map(u16::from)),
            ]
            .width(iced::Fill),
        )
        .size(size)
        .width(300);
        if let Some(spacing) = spacing {
            root = root.spacing(spacing);
        }
        let mut ui = Simulator::with_size(
            settings(),
            iced::Size::new(300., 400.),
            Element::from(container(root).id(root_id.clone())),
        );
        let title = ui.find("Title").unwrap().bounds();
        let body = ui.find(first).unwrap().bounds();
        let next = ui.find(second).unwrap().bounds();
        let footer = ui.find(foot).unwrap().bounds();
        let root = ui.find(root_id).unwrap().bounds();
        assert_eq!(title.x, inset);
        assert_eq!(title.y, inset);
        assert!((title.height - title_height).abs() < 0.1, "{title:?}");
        assert_eq!(body.x, inset);
        assert_eq!(body.width, 300. - 2. * inset);
        assert!((body.y - title.y - title.height - inset).abs() < 0.1);
        assert_eq!(next.y, body.y + 10., "root gap must stop at outer column");
        assert_eq!(footer.x, inset);
        assert!((footer.y - (next.y + next.height + inset + 1. + inset)).abs() < 0.1);
        assert!(
            (root.height - footer.y - footer.height - inset).abs() < 0.1,
            "footer removes root bottom padding: {root:?} {footer:?}"
        );
    }
}

#[test]
fn nested_cards_reset_layout_context_and_keep_the_outer_bottom_padding() {
    use iced::widget::Id;
    use icedcn::Size;
    use icedcn::widget::{column, container};
    let inner_id = Id::unique();
    let outer_id = Id::unique();
    let inner = card(
        column![
            card::header("Inner"),
            card::footer(text::small("Inner footer")),
        ]
        .width(iced::Fill),
    )
    .size(Size::Sm)
    .width(iced::Fill);
    let outer: Slot<'_, ()> = container(
        card(
            column![
                card::header("Outer"),
                card::content(container(inner).id(inner_id.clone())),
            ]
            .width(iced::Fill),
        )
        .spacing(32)
        .width(300),
    )
    .id(outer_id.clone())
    .into();
    let mut ui = Simulator::with_size(settings(), iced::Size::new(300., 400.), outer.compose());
    let inner = ui.find(inner_id).unwrap().bounds();
    let outer = ui.find(outer_id).unwrap().bounds();
    let title = ui.find("Inner").unwrap().bounds();
    assert_eq!(inner.x, 32.);
    assert_eq!(title.x, 44.);
    assert_eq!(title.y, inner.y + 12.);
    assert!((title.height - 19.25).abs() < 0.1);
    assert_eq!(outer.height, inner.y + inner.height + 32.);
}

#[test]
fn explicit_part_and_column_overrides_win_over_root_spacing() {
    use iced::widget::Id;
    use icedcn::Size;
    use icedcn::widget::{column, container};
    let body_id = Id::unique();
    let foot_id = Id::unique();
    let root: Slot<'_, ()> = card(
        column![
            card::header(card::title("Custom").size(18)).inset(7.),
            card::content(container(text::small("Body")).id(body_id.clone())).padding([0, 5]),
            card::footer_with_spacing(container(text::small("Foot")).id(foot_id.clone()), 9.),
        ]
        .spacing(3)
        .width(iced::Fill),
    )
    .size(Size::Sm)
    .spacing(32)
    .width(300)
    .into();
    let mut ui = Simulator::with_size(settings(), iced::Size::new(300., 400.), root.compose());
    let title = ui.find("Custom").unwrap().bounds();
    let body = ui.find(body_id).unwrap().bounds();
    let foot = ui.find(foot_id).unwrap().bounds();
    assert_eq!(title.x, 7.);
    assert!((title.height - 24.75).abs() < 0.1);
    assert_eq!(body.x, 5.);
    assert!((body.y - title.y - title.height - 3.).abs() < 0.1);
    assert_eq!(foot.x, 9.);
}
