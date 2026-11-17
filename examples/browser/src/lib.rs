//! Components-browser entry point.
//!
//! Mirrors the loader-state-machine pattern in `examples/highlight`: a
//! [`Browser`] state machine handles the splash screen while we parse the
//! bundled code themes and kick off the font catalog fetch, then
//! hands off to [`app::App`] once the first font message arrives.
//!
//! `app::App` owns the actual UI. Everything else lives in `lib.rs` only
//! to keep startup state out of the runtime path.

// Page constructors return `Box<dyn page::Page>` for the page registry.
#![allow(clippy::new_ret_no_self)]

pub mod app;
pub mod component;
pub mod docs;
pub mod examples;
pub mod font;
pub mod fonts;
pub mod guide;
pub mod icon;
pub mod page;
#[cfg(target_arch = "wasm32")]
mod parser_heap;

use iced::widget::column;
use iced::{Element, Subscription, Task};
use icedcn::Theme;
use icedcn::widget::markdown::highlighter::Map;
use icedcn::widget::{center, progress, text};

use app::App;

#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;
#[cfg(target_arch = "wasm32")]
use wasmtimer::tokio::sleep;

/// Bundled GitHub code themes, parsed once during startup.
const GH_LIGHT: &str = include_str!("../themes/github-light-default.json");
const GH_DARK: &str = include_str!("../themes/github-dark.json");

fn parse_code_map(json: &str) -> Map {
    Map::from_vscode_theme(json).expect("vendored github theme parses")
}

/// Re-export so pages and other callers continue to use `crate::Message`.
pub use app::Message;

/// Loader state. The [`App`] is constructed at startup so it can absorb
/// font/catalog events from the moment they arrive; the splash overlay
/// stays visible until the default font binary actually finishes loading.
pub struct Browser {
    app: App,
    font_loaded: bool,
    /// Visual hint for splash progression. Bumped by phase as font-loading
    /// events arrive (system list -> Google catalog -> font bytes). Clamped
    /// to never decrease, so out-of-order events still progress visually.
    load_progress: f32,
}

impl Browser {
    pub fn new() -> (Self, Task<Message>) {
        let gh_light = parse_code_map(GH_LIGHT);
        let gh_dark = parse_code_map(GH_DARK);
        let (app, app_init) = App::new(gh_light, gh_dark);
        let fonts_init = fonts::init().map(Message::Font);
        (
            Browser {
                app,
                font_loaded: cfg!(target_arch = "wasm32"),
                load_progress: 0.0,
            },
            Task::batch([app_init, fonts_init]),
        )
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        // Splash dismissal is gated on the progress bar's `on_idle` arriving
        // with `value == 100`, so the bar visibly reaches the right edge
        // before the splash hands off to the app. Bumping `load_progress`
        // to 100 on terminal font events (success, per-font fail, or
        // catalog-load fail offline) drives the final ease; `SplashIdle(100)`
        // then flips `font_loaded`.
        if let Message::SplashIdle(value) = msg
            && value >= 100.0
        {
            self.font_loaded = true;
            return Task::none();
        }

        // Phase-based splash progress. Each terminal stage clamps the bar
        // upward; the `.max()` keeps reordered events monotone.
        if let Message::Font(font_msg) = &msg {
            let bump = match font_msg {
                fonts::Message::SystemFontsLoaded(_) => 25.0,
                fonts::Message::CatalogLoaded(Ok(_)) => 50.0,
                // CatalogLoaded(Err): offline; no `Loaded` will follow, so
                // jump to 100 to drive the splash through to dismissal.
                fonts::Message::CatalogLoaded(Err(_)) => 100.0,
                // Any `Loaded` (success or per-font failure) means the font
                // pipeline is done — drive to 100.
                fonts::Message::Loaded(_, _) => 100.0,
            };
            self.load_progress = self.load_progress.max(bump);
        }

        self.app.update(msg)
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        // iced asks for the first view only once its renderer exists.
        #[cfg(target_arch = "wasm32")]
        {
            use std::sync::Once;
            static READY: Once = Once::new();
            READY.call_once(|| shell("icedcnReady", ""));
        }
        if self.font_loaded {
            self.app.view()
        } else {
            splash("Loading\u{2026}", self.load_progress)
        }
    }

    /// Title of the active site destination.
    pub fn title(&self) -> String {
        self.app.title()
    }

    pub fn theme(&self) -> Theme {
        self.app.theme()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.font_loaded {
            self.app.subscription()
        } else {
            Subscription::none()
        }
    }
}

fn splash<'a>(message: &str, progress_value: f32) -> Element<'a, Message, Theme> {
    center(
        column![
            Element::from(text::h1("icedcn")),
            Element::from(text::muted(message.to_owned())),
            Element::from(
                progress(progress_value)
                    .width(140)
                    .on_idle(Message::SplashIdle)
            ),
        ]
        .spacing(12)
        .align_x(iced::Center),
    )
    .into()
}

/// Whether the page shell found neither WebGPU nor WebGL2, so iced draws
/// with tiny-skia. Set by `web/index.html` before the app starts; always
/// false on native.
pub(crate) fn cpu_renderer() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        let key = js_sys::JsString::from("icedcnRenderer");
        web_sys::window()
            .and_then(|window| js_sys::Reflect::get(&window, &key).ok())
            .and_then(|value| value.as_string())
            .is_some_and(|renderer| renderer == "cpu")
    }
    #[cfg(not(target_arch = "wasm32"))]
    false
}

/// Calls the page shell's `name` callback, if `web/index.html` defined one.
#[cfg(target_arch = "wasm32")]
fn shell(name: &str, argument: &str) {
    use js_sys::{Function, JsString, Reflect};

    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(value) = Reflect::get(&window, &JsString::from(name)) else {
        return;
    };
    if value.is_function() {
        let _ = Function::from(value).call1(&window, &JsString::from(argument));
    }
}

/// Run the component browser with its bundled fonts and theme.
pub fn run() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::console;
        // Startup runs after the shell's `start()` has resolved, so a panic
        // there is reported to the shell, which otherwise keeps its splash.
        std::panic::set_hook(Box::new(|panic| {
            let message = panic.to_string();
            console::error_1(&message.as_str().into());
            shell("icedcnFailed", &message);
        }));
        parser_heap::install();
    }
    font::use_geist_mono();
    let application = iced::application(Browser::new, Browser::update, Browser::view)
        .title(Browser::title)
        .window_size([1600.0, 1000.0])
        .antialiasing(true)
        .subscription(Browser::subscription)
        .theme(Browser::theme)
        .fonts([icon::FONT, palette::icon::FONT, font::GEIST_MONO]);
    // Skip the wgpu attempts the page already knows will fail; this also makes
    // `?renderer=cpu` draw on the CPU.
    #[cfg(target_arch = "wasm32")]
    let application = if cpu_renderer() {
        application.backend(iced::Backend::Software)
    } else {
        application
    };
    #[cfg(target_arch = "wasm32")]
    let application = application
        .fonts([include_bytes!("../fonts/Geist.ttf").as_slice(), font::EMOJI])
        .font(font::BODY.into());
    application.run()
}
