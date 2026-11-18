//! When a floating scrollbar shows, after Radix ScrollArea's `type` and `scrollHideDelay`.
use crate::animation::cubic_bezier;
use crate::core::time::{Duration, Instant};

/// Radix's `scrollHideDelay` default.
pub const HIDE_DELAY: Duration = Duration::from_millis(600);

/// A suggested fade-out for idle scrollbars, after the overlay scrollbars of macOS and
/// Chromium, which fade over about 200ms. Radix and Nova hide at once, the default.
pub const FADE: Duration = Duration::from_millis(200);

/// Radix's scroll-end debounce: a scrollbar in [`Visibility::Scroll`] starts its hide delay this
/// long after the last scroll.
const SCROLL_END: Duration = Duration::from_millis(100);

/// When the scrollbars of a [`Scrollable`](super::Scrollable) are shown, like Radix
/// ScrollArea's `type`. An embedded scrollbar (one with spacing) is always shown.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Visibility {
    /// Shown whenever the content overflows.
    #[default]
    Always,
    /// Shown when the content overflows. A scrollbar only exists when its axis overflows, so
    /// this paints like [`Visibility::Always`].
    Auto,
    /// Shown while the pointer is over the scrollable or a scroller is dragged, then hidden
    /// once the hide delay has passed.
    Hover,
    /// Shown while its axis scrolls, while the pointer is on it and while it is dragged, then
    /// hidden once the hide delay has passed.
    Scroll,
}

impl Visibility {
    /// Whether this mode hides idle scrollbars.
    pub(super) fn hides(self) -> bool {
        matches!(self, Visibility::Hover | Visibility::Scroll)
    }
}

/// One scrollbar's opacity over time in [`Visibility::Hover`] and [`Visibility::Scroll`].
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct Fade {
    held: bool,
    hide_at: Option<Instant>,
    alpha: f32,
}

impl Fade {
    /// The opacity to paint the scrollbar with.
    pub(super) fn alpha(&self) -> f32 {
        self.alpha
    }

    /// Advance to `now`. A `held` scrollbar is shown; letting go of it starts the hide delay.
    /// A `scrolled` one is shown until the scroll ends and the delay passes. Then it fades out
    /// over `fade`, or hides at once when `fade` is zero. Returns when the next frame must be
    /// drawn, if the scrollbar is waiting to hide or fading out.
    pub(super) fn advance(
        &mut self,
        now: Instant,
        held: bool,
        scrolled: bool,
        delay: Duration,
        fade: Duration,
    ) -> Option<Instant> {
        if held {
            *self = Fade {
                held: true,
                hide_at: None,
                alpha: 1.0,
            };
            return None;
        }
        if std::mem::take(&mut self.held) {
            self.hide_at = Some(now + delay);
        }
        if scrolled {
            let at = now + SCROLL_END + delay;
            self.hide_at = Some(self.hide_at.map_or(at, |hide_at| hide_at.max(at)));
            self.alpha = 1.0;
        }
        let hide_at = self.hide_at?;
        if now < hide_at {
            self.alpha = 1.0;
            return Some(hide_at);
        }
        let elapsed = now.saturating_duration_since(hide_at);
        if elapsed >= fade {
            self.hide_at = None;
            self.alpha = 0.0;
            return None;
        }
        let t = elapsed.as_secs_f32() / fade.as_secs_f32();
        self.alpha = 1.0 - cubic_bezier(0.4, 0.0, 0.2, 1.0, t);
        Some(now)
    }
}

#[cfg(test)]
mod tests {
    use super::{FADE, Fade, HIDE_DELAY, SCROLL_END};
    use crate::core::time::{Duration, Instant};

    #[test]
    fn alpha_follows_hold_delay_and_fade() {
        let start = Instant::now();
        let mut fade = Fade::default();
        assert_eq!(fade.alpha(), 0.0, "starts hidden");
        assert_eq!(fade.advance(start, true, false, HIDE_DELAY, FADE), None);
        assert_eq!(fade.alpha(), 1.0, "held means shown");

        let hide = start + HIDE_DELAY;
        assert_eq!(
            fade.advance(start, false, false, HIDE_DELAY, FADE),
            Some(hide)
        );
        assert_eq!(fade.alpha(), 1.0, "shown through the delay");

        let middle = hide + FADE / 2;
        assert_eq!(
            fade.advance(middle, false, false, HIDE_DELAY, FADE),
            Some(middle)
        );
        assert!(
            fade.alpha() > 0.0 && fade.alpha() < 1.0,
            "half way through the fade: {}",
            fade.alpha()
        );

        assert_eq!(
            fade.advance(hide + FADE, false, false, HIDE_DELAY, FADE),
            None
        );
        assert_eq!(fade.alpha(), 0.0, "hidden after the fade");
    }

    #[test]
    fn without_a_fade_it_hides_at_once() {
        let start = Instant::now();
        let mut fade = Fade::default();
        let _ = fade.advance(start, true, false, HIDE_DELAY, Duration::ZERO);
        let hide = start + HIDE_DELAY;
        let next = fade.advance(start, false, false, HIDE_DELAY, Duration::ZERO);
        assert_eq!(next, Some(hide));
        assert_eq!(
            fade.advance(hide, false, false, HIDE_DELAY, Duration::ZERO),
            None
        );
        assert_eq!(fade.alpha(), 0.0, "gone at the end of the delay");
    }

    #[test]
    fn a_scroll_during_the_fade_shows_it_again() {
        let start = Instant::now();
        let mut fade = Fade::default();
        let _ = fade.advance(start, false, true, HIDE_DELAY, FADE);
        let fading = start + SCROLL_END + HIDE_DELAY + FADE / 2;
        let _ = fade.advance(fading, false, false, HIDE_DELAY, FADE);
        assert!(fade.alpha() < 1.0);
        let again = fade.advance(fading, false, true, HIDE_DELAY, FADE);
        assert_eq!(fade.alpha(), 1.0);
        assert_eq!(again, Some(fading + SCROLL_END + HIDE_DELAY));
    }
}
