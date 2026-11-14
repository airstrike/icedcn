//! GPU regression for Avatar's foreground layers, rings, and responsive recipe.
use browser::component::avatar::AvatarPage;
use iced::widget::image;
use iced::{Color, Element, Rectangle, Renderer, Size, mouse};
use iced_test::core::Renderer as _;
use iced_test::core::image::Renderer as _;
use iced_test::core::renderer::{Headless, Scale, Settings, Style};
use iced_test::core::widget::{Id, Operation};
use iced_test::futures::futures::executor::block_on;
use iced_test::runtime::UserInterface;
use icedxdy::Theme;
use icedxdy::theme::container;
use icedxdy::widget::avatar;

fn renderer() -> Renderer {
    let mut renderer =
        block_on(Renderer::new(Settings::default(), Some("wgpu"))).expect("headless WGPU renderer");
    renderer.hint(Scale {
        window: 1.0,
        application: 1.0,
    });
    renderer
}

fn solid(renderer: &Renderer, rgba: [u8; 4]) -> image::Handle {
    let handle = image::Handle::from_rgba(4, 4, rgba.repeat(16));
    renderer.load_image(&handle).expect("decode solid fixture");
    handle
}

fn pixels(element: Element<'_, (), Theme>, renderer: &mut Renderer, theme: &Theme) -> Vec<u8> {
    let mut ui = UserInterface::build(element, Size::new(64.0, 40.0), Default::default(), renderer);
    ui.draw(
        renderer,
        theme,
        &Style {
            text_color: theme.neutral.foreground.into(),
        },
        mouse::Cursor::Unavailable,
    );
    renderer.screenshot(Size::new(64, 40), 1.0, Color::from(theme.neutral.base))
}

fn pixel(pixels: &[u8], x: usize, y: usize) -> &[u8] {
    let index = (y * 64 + x) * 4;
    &pixels[index..index + 3]
}

#[test]
fn badge_and_group_rings_paint_above_loaded_portraits() {
    let mut renderer = renderer();
    let red = solid(&renderer, [255, 0, 0, 255]);
    let blue = solid(&renderer, [0, 0, 255, 255]);
    for theme in [Theme::light(), Theme::dark()] {
        let badge = avatar::badge(avatar(Some(red.clone())), avatar::dot()).badge_style(|_| {
            container::Style {
                background: Some(icedxdy::Color::from_srgb(Color::from_rgb8(0, 166, 62)).into()),
                ..Default::default()
            }
        });
        let dot = pixels(badge.into(), &mut renderer, &theme);
        let center = pixel(&dot, 27, 27);
        assert!(
            center[0] < 5 && center[1] > 160 && center[2] < 65,
            "badge is behind image: {center:?}"
        );
        let background = pixel(&dot, 60, 20);
        assert_eq!(
            pixel(&dot, 21, 27),
            background,
            "badge outer ring must cover the portrait"
        );
        let group = pixels(
            avatar::group([avatar(Some(red.clone())), avatar(Some(blue.clone()))]).into(),
            &mut renderer,
            &theme,
        );
        assert_eq!(
            pixel(&group, 23, 16),
            background,
            "later avatar's outer ring must cover the first image"
        );
        assert_eq!(pixel(&group, 16, 16), [255, 0, 0]);
        assert_eq!(pixel(&group, 40, 16), [0, 0, 255]);
    }
}

#[test]
fn fallback_edge_matches_border_token_and_dark_portrait_uses_white_overlay() {
    let mut renderer = renderer();
    let invalid = image::Handle::from_bytes(b"invalid avatar image".as_slice());
    assert!(renderer.load_image(&invalid).is_err());
    for theme in [Theme::light(), Theme::dark()] {
        let fallback = pixels(avatar(None).into(), &mut renderer, &theme);
        let failed = pixels(avatar(Some(invalid.clone())).into(), &mut renderer, &theme);
        assert_eq!(fallback, failed, "failed images must retain the same edge");
        let border = Color::from(theme.neutral.border);
        let background = Color::from(theme.muted.base);
        let expected =
            ((border.r * border.a + background.r * (1.0 - border.a)) * 255.0).round() as i16;
        let edge = pixel(&fallback, 0, 15)[0] as i16;
        assert!(
            (edge - expected).abs() <= 2,
            "fallback edge:{edge}, expected:{expected}"
        );
    }

    let black = solid(&renderer, [0, 0, 0, 255]);
    let portrait = pixels(
        avatar(Some(black)).size(33.0).into(),
        &mut renderer,
        &Theme::dark(),
    );
    // Source dark:border-border is white/10. Allow one level of WGPU's
    // byte quantization around the source value of25.5 on black.
    // An odd diameter puts this edge exactly on a full-coverage pixel.
    assert!(
        pixel(&portrait, 0, 16)
            .iter()
            .all(|channel| (25..=26).contains(channel))
    );
    assert_eq!(pixel(&portrait, 1, 16), [0, 0, 0]);
}

#[derive(Default)]
struct Bounds(Vec<Rectangle>);
impl Operation for Bounds {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn container(&mut self, _: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
        self.0.push(bounds);
    }
}

#[test]
fn registered_hero_tracks_window_breakpoint_and_wraps_in_a_tight_preview() {
    let mut renderer = renderer();
    let mut page = AvatarPage::new(0);
    for (window, available, width, height) in [
        (1440.0, 638.0, 264.0, 32.0),
        (390.0, 340.0, 216.0, 32.0),
        (390.0, 150.0, 104.0, 88.0),
    ] {
        page.window_width(window);
        let mut ui = UserInterface::build(
            page.preview("demo"),
            Size::new(available, 288.0),
            Default::default(),
            &mut renderer,
        );
        let mut bounds = Bounds::default();
        ui.operate(&renderer, &mut bounds);
        let hero = bounds.0.first().expect("hero row bounds");
        assert_eq!((hero.width, hero.height), (width, height));
        assert!(
            bounds
                .0
                .iter()
                .all(|b| b.x >= 0.0 && b.x + b.width <= available)
        );
    }
}
