//! The platform whose text-editing conventions the text widgets follow.
use std::cell::Cell;

/// The family of platform whose keyboard conventions a text field follows.
///
/// A native build knows its platform when it compiles. A web build asks the
/// browser which system it runs on, since one binary serves them all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    /// macOS, and iPadOS with a hardware keyboard.
    Mac,
    /// Windows.
    Windows,
    /// Linux and the other desktops, with GTK's conventions.
    Linux,
}

thread_local! {
    static OVERRIDE: Cell<Option<Platform>> = const { Cell::new(None) };
}

impl Platform {
    /// The platform the text widgets on this thread follow.
    pub fn current() -> Self {
        OVERRIDE.with(Cell::get).unwrap_or_else(Self::host)
    }

    /// Follow `platform` instead of the host's on this thread, or the host's
    /// again with `None`. For tests of each platform's conventions.
    #[doc(hidden)]
    pub fn set(platform: Option<Self>) {
        OVERRIDE.with(|current| current.set(platform));
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn host() -> Self {
        if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
            Self::Mac
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else {
            Self::Linux
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn host() -> Self {
        use std::sync::OnceLock;

        static HOST: OnceLock<Platform> = OnceLock::new();

        *HOST.get_or_init(|| {
            let navigator = web_sys::window().map(|window| window.navigator());
            let platform = navigator
                .as_ref()
                .and_then(|navigator| navigator.platform().ok())
                .unwrap_or_default();
            let agent = navigator
                .as_ref()
                .and_then(|navigator| navigator.user_agent().ok())
                .unwrap_or_default();

            Platform::from_navigator(&platform, &agent)
        })
    }

    /// The platform a browser reports through `navigator.platform` and
    /// `navigator.userAgent`.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn from_navigator(platform: &str, agent: &str) -> Self {
        let platform = platform.to_ascii_lowercase();
        let agent = agent.to_ascii_lowercase();
        let reports = |name: &str| platform.contains(name) || agent.contains(name);

        if reports("mac") || reports("iphone") || reports("ipad") {
            Self::Mac
        } else if reports("win") {
            Self::Windows
        } else {
            Self::Linux
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Platform;

    #[test]
    fn a_browser_reports_its_platform() {
        let cases = [
            ("MacIntel", "", Platform::Mac),
            ("iPhone", "", Platform::Mac),
            ("Win32", "", Platform::Windows),
            ("Linux x86_64", "", Platform::Linux),
            (
                "",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
                Platform::Windows,
            ),
            (
                "",
                "Mozilla/5.0 (X11; CrOS x86_64 14541.0.0)",
                Platform::Linux,
            ),
            ("", "", Platform::Linux),
        ];
        for (platform, agent, expected) in cases {
            assert_eq!(
                Platform::from_navigator(platform, agent),
                expected,
                "{platform:?} {agent:?}"
            );
        }
    }

    #[test]
    fn a_thread_can_follow_another_platform() {
        Platform::set(Some(Platform::Windows));
        assert_eq!(Platform::current(), Platform::Windows);
        Platform::set(None);
    }
}
