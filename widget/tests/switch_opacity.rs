//! A disabled switch composites its overlapping thumb and track only once.
#![cfg(feature = "wgpu")]

use iced_core::renderer::{Headless, Settings, Style};
use iced_core::{Color, Element, Size, mouse};
use iced_test::futures::futures::executor::block_on;
use iced_test::runtime::UserInterface;
use iced_widget::{Renderer, Space, Widget, container, row};
use icedcn_core::Theme;
use icedcn_core::theme::switch;
use icedcn_widget::switch::switch;

type Control<'a> = Element<'a, (), Theme, Renderer>;

fn swatch(color: Color) -> Control<'static> {
    container(Space::new().width(8).height(18))
        .style(move |_| container::Style::default().background(color))
        .boxed()
}

fn pixel(frame: &[u8], x: usize) -> &[u8] {
    let offset = (64 * 10 + x) * 4;
    &frame[offset..offset + 3]
}

#[cfg(feature = "vello-cpu")]
#[test]
fn software_fallback_keeps_the_disabled_style() {
    let mut renderer = block_on(<Renderer as Headless>::new(
        Settings::default(),
        Some("vello-cpu"),
    ))
    .unwrap();
    let style: switch::StyleFn<'_, Theme> = Box::new(|_, status| {
        let track = if matches!(status, switch::Status::Disabled { .. }) {
            Color::from_rgb8(0, 255, 0)
        } else {
            Color::from_rgb8(255, 0, 0)
        };
        switch::Style {
            background: icedcn_core::Color::from(track).into(),
            ..switch::Style::default()
        }
    });
    let switch: Control<'_> = switch()
        .checked(true)
        .disabled(true)
        .disabled_opacity(0.5)
        .style(style)
        .into();
    let mut ui = UserInterface::build(
        switch,
        Size::new(64., 24.),
        Default::default(),
        &mut renderer,
    );
    ui.draw(
        &mut renderer,
        &Theme::light(),
        &Style::default(),
        mouse::Cursor::Unavailable,
    );
    let frame = Headless::screenshot(&mut renderer, Size::new(64, 24), 1., Color::WHITE);
    assert_eq!(pixel(&frame, 8), [0, 255, 0]);
}

#[test]
fn group_opacity_matches_a_single_overlay_on_each_backdrop() {
    let mut renderer = block_on(<Renderer as Headless>::new(
        Settings::default(),
        Some("wgpu"),
    ))
    .unwrap();
    for (theme, thumb, surface, canvas, on, off) in [
        (
            Theme::light(),
            0xf8fafb,
            0xf8fafb,
            0xffffff,
            0x037f8c,
            0xdde2e5,
        ),
        (
            Theme::dark(),
            0xf5f6f7,
            0x1f2328,
            0x181c21,
            0x3197a5,
            0x35393f,
        ),
    ] {
        let rgb =
            |value: u32| Color::from_rgb8((value >> 16) as u8, (value >> 8) as u8, value as u8);
        for background in [surface, canvas] {
            for checked in [false, true] {
                let mut track = rgb(if checked { on } else { off });
                if !checked && canvas == 0x181c21 {
                    track.a = 0.8;
                }
                let thumb = rgb(thumb);
                let style: switch::StyleFn<'_, Theme> = Box::new(move |_, _| switch::Style {
                    background: icedcn_core::Color::from(track).into(),
                    foreground: icedcn_core::Color::from(thumb).into(),
                    ..switch::Style::default()
                });
                let control: Control<'_> = switch()
                    .checked(checked)
                    .disabled(true)
                    .disabled_opacity(0.5)
                    .style(style)
                    .into();
                // The two reference swatches do not overlap. Each blends once with the same surface.
                let row = row![
                    control,
                    swatch(track.scale_alpha(0.5)),
                    swatch(thumb.scale_alpha(0.5))
                ];
                let mut ui = UserInterface::build(
                    row,
                    Size::new(64., 24.),
                    Default::default(),
                    &mut renderer,
                );
                ui.draw(
                    &mut renderer,
                    &theme,
                    &Style::default(),
                    mouse::Cursor::Unavailable,
                );
                let frame =
                    Headless::screenshot(&mut renderer, Size::new(64, 24), 1., rgb(background));
                for (point, reference) in [
                    (if checked { 8 } else { 24 }, 36),
                    (if checked { 24 } else { 8 }, 44),
                ] {
                    let observed = pixel(&frame, point);
                    let expected = pixel(&frame, reference);
                    assert!(
                        observed
                            .iter()
                            .zip(expected)
                            .all(|(a, b)| a.abs_diff(*b) <= 2),
                        "background={background:x}, checked={checked}, x={point}: {observed:?} vs {expected:?}"
                    );
                }
            }
        }
    }
}
