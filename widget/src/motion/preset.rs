use std::fmt;
use std::time::Duration;

use iced_core::{Point, Rectangle, Transformation};

use crate::animation::{cubic_bezier, ease_out};

/// Entrance effects for newly mounted content.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Fade,
    #[default]
    SlideUp,
    SlideSide,
    Pop,
    SpringBounce,
    ScaleFade,
}

impl fmt::Display for Preset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Fade => "Fade",
            Self::SlideUp => "Slide Up",
            Self::SlideSide => "Slide Side",
            Self::Pop => "Pop",
            Self::SpringBounce => "Spring Bounce",
            Self::ScaleFade => "Scale Fade",
        })
    }
}

impl Preset {
    /// All supported presets, in picker order.
    pub const ALL: [Self; 6] = [
        Self::Fade,
        Self::SlideUp,
        Self::SlideSide,
        Self::Pop,
        Self::SpringBounce,
        Self::ScaleFade,
    ];

    pub(super) fn duration(self) -> Duration {
        Duration::from_millis(match self {
            Self::Fade => 220,
            Self::SlideUp => 260,
            Self::SlideSide => 280,
            Self::ScaleFade => 240,
            // At this time both springs are under 0.001px from rest.
            Self::Pop | Self::SpringBounce => 450,
        })
    }

    /// Fraction of initial displacement. Springs may cross the equilibrium.
    pub(super) fn remaining(self, progress: f32) -> f32 {
        if progress >= 1.0 {
            return 0.0;
        }
        match self {
            Self::Fade | Self::ScaleFade => 1.0 - ease_out(progress),
            Self::SlideUp | Self::SlideSide => 1.0 - cubic_bezier(0.16, 1.0, 0.3, 1.0, progress),
            Self::Pop | Self::SpringBounce => {
                let (stiffness, damping) = if self == Self::Pop {
                    (500.0, 34.0)
                } else {
                    (520.0, 30.0)
                };
                spring(
                    stiffness,
                    damping,
                    0.7,
                    progress * self.duration().as_secs_f32(),
                )
            }
        }
    }

    pub(super) fn transform(self, remaining: f32, bounds: Rectangle) -> Transformation {
        if remaining == 0.0 {
            return Transformation::IDENTITY;
        }
        let (x, y, scale) = match self {
            Self::SlideUp => (0.0, 10.0 * remaining, 1.0),
            Self::SlideSide => (18.0 * remaining, 0.0, 1.0),
            Self::Pop => (0.0, 6.0 * remaining, 1.0 - 0.06 * remaining),
            Self::SpringBounce => (0.0, 12.0 * remaining, 1.0 - 0.04 * remaining),
            Self::ScaleFade => (0.0, 0.0, 1.0 - 0.02 * remaining),
            Self::Fade => return Transformation::IDENTITY,
        };
        let origin = if self == Self::Pop {
            Point::new(bounds.x + bounds.width, bounds.y + bounds.height)
        } else {
            bounds.center()
        };
        Transformation::translate(origin.x + x, origin.y + y)
            * Transformation::scale(scale)
            * Transformation::translate(-origin.x, -origin.y)
    }
}

/// Closed-form displacement of an underdamped spring released from rest.
fn spring(stiffness: f32, damping: f32, mass: f32, seconds: f32) -> f32 {
    let decay = damping / (2.0 * mass);
    let frequency = (stiffness / mass - decay * decay).sqrt();
    let (sin, cos) = (frequency * seconds).sin_cos();
    (-decay * seconds).exp() * (cos + decay / frequency * sin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::Size;

    #[test]
    fn fade_stays_still_and_springs_keep_their_transform_origins() {
        let bounds = Rectangle::new(Point::new(10., 20.), Size::new(100., 40.));
        assert_eq!(
            Preset::Fade.transform(0.5, bounds),
            Transformation::IDENTITY
        );
        let edge = Point::new(110., 60.);
        let pop = edge * Preset::Pop.transform(1., bounds);
        assert!((pop.x - edge.x).abs() < 0.001);
        assert!((pop.y - edge.y - 6.).abs() < 0.001);
        let center = bounds.center();
        let spring = center * Preset::SpringBounce.transform(1., bounds);
        assert!((spring.x - center.x).abs() < 0.001);
        assert!((spring.y - center.y - 12.).abs() < 0.001);
        assert!(Preset::SpringBounce.remaining(0.5) < 0.);
    }
}
