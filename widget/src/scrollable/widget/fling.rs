//! Android's spline fling, ported from Chromium's `ui/events/mobile_scroller.cc`.
//!
//! Source revision: 37d10a51f02e32921290fd80cc0bdaa615ac25fa.
//! <https://chromium.googlesource.com/chromium/src/+/37d10a51f02e32921290fd80cc0bdaa615ac25fa/ui/events/mobile_scroller.cc>
//!
//! Uses the default Android configuration (no flywheel or Chromecast tuning).
//! Coordinates are relative to the release position; the caller clips against
//! scroll bounds. The unused scroll-mode viscosity and spline-time table are
//! omitted. Float/double boundaries and microsecond duration truncation follow
//! the source, including its 0.1-pixel endpoint termination.

// Copyright 2014 The Chromium Authors
// Copyright 2015 The Chromium Authors (license)
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::sync::OnceLock;

use crate::core::Vector;

const FRICTION: f32 = 0.015;
const DECELERATION_RATE: f32 = 2.358_201_8;
const INFLEXION: f32 = 0.35;
const EPSILON: f32 = 1e-5;
const END_THRESHOLD: f32 = 0.1;
const SAMPLES: usize = 100;

/// An unconstrained trajectory from a release velocity in logical pixels/second.
#[derive(Debug, Clone, Copy)]
pub(super) struct Fling {
    duration: f64,
    reciprocal: f64,
    distance: f32,
    displacement: Vector,
    direction: Vector,
}

/// Position and velocity at a timestamp relative to the release.
#[derive(Debug, Clone, Copy)]
pub(super) struct Sample {
    pub(super) displacement: Vector,
    pub(super) velocity: Vector,
    pub(super) finished: bool,
}

impl Fling {
    /// Converts a finite, nonzero release velocity into Android's trajectory.
    pub(super) fn new(velocity: Vector) -> Option<Self> {
        let speed = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
        if !speed.is_finite() || speed == 0.0 {
            return None;
        }

        let tuning = 9.806_65_f32 * 39.37 * 160.0 * 0.84;
        let deceleration = f64::from((INFLEXION * speed / (FRICTION * tuning)).ln());
        let exponent = f64::from(DECELERATION_RATE) - 1.0;
        let duration = ((deceleration / exponent).exp() * 1_000_000.0).trunc() / 1_000_000.0;
        if !duration.is_finite() || duration <= 0.0 {
            return None;
        }

        let distance = f64::from(FRICTION * tuning)
            * (f64::from(DECELERATION_RATE) / exponent * deceleration).exp();
        let displacement = Vector::new(
            (distance * f64::from(velocity.x / speed)) as f32,
            (distance * f64::from(velocity.y / speed)) as f32,
        );
        let magnitude = (displacement.x * displacement.x + displacement.y * displacement.y).sqrt();
        let direction = if magnitude > EPSILON {
            displacement / magnitude
        } else {
            Vector::new(1.0, 1.0)
        };

        Some(Self {
            duration,
            reciprocal: 1.0 / duration,
            distance: distance as f32,
            displacement,
            direction,
        })
    }

    /// Returns the unconstrained final displacement from the release position.
    pub(super) fn displacement(&self) -> Vector {
        self.displacement
    }

    /// Returns Android's duration in seconds, truncated to microseconds.
    pub(super) fn duration(&self) -> f64 {
        self.duration
    }

    /// Samples absolute elapsed time; no frame-by-frame integration is needed.
    pub(super) fn sample(&self, elapsed_seconds: f64) -> Sample {
        let elapsed = (elapsed_seconds * 1_000_000.0).trunc() / 1_000_000.0;
        if elapsed <= 0.0 {
            return Sample {
                displacement: Vector::ZERO,
                velocity: Vector::ZERO,
                finished: false,
            };
        }

        if elapsed >= self.duration {
            return self.finished();
        }

        let t = (elapsed * self.reciprocal) as f32;
        let (distance, velocity) = coefficients(t);
        let displacement = self.displacement * distance;
        let remaining = self.displacement - displacement;
        if remaining.x.abs() < END_THRESHOLD && remaining.y.abs() < END_THRESHOLD {
            return self.finished();
        }

        let speed = (f64::from(velocity * self.distance) * self.reciprocal) as f32;
        Sample {
            displacement,
            velocity: self.direction * speed,
            finished: false,
        }
    }

    fn finished(&self) -> Sample {
        Sample {
            displacement: self.displacement,
            velocity: Vector::ZERO,
            finished: true,
        }
    }
}

fn coefficients(t: f32) -> (f32, f32) {
    static POSITION: OnceLock<[f32; SAMPLES + 1]> = OnceLock::new();
    let position = POSITION.get_or_init(spline);
    let index = (SAMPLES as f32 * t).floor() as usize;
    if index >= SAMPLES {
        return (1.0, 0.0);
    }

    let lower = index as f32 / SAMPLES as f32;
    let upper = (index + 1) as f32 / SAMPLES as f32;
    let velocity = (position[index + 1] - position[index]) / (upper - lower);
    (position[index] + (t - lower) * velocity, velocity)
}

fn spline() -> [f32; SAMPLES + 1] {
    let start_tension = 0.5;
    let end_tension = 1.0;
    let p1 = start_tension * INFLEXION;
    let p2 = 1.0 - end_tension * (1.0 - INFLEXION);
    let mut position = [1.0; SAMPLES + 1];
    let mut x_min = 0.0;

    for (i, entry) in position.iter_mut().take(SAMPLES).enumerate() {
        let alpha = i as f32 / SAMPLES as f32;
        let mut x_max = 1.0;
        loop {
            let x = x_min + (x_max - x_min) / 2.0;
            let coefficient = 3.0 * x * (1.0 - x);
            let tx = coefficient * ((1.0 - x) * p1 + x * p2) + x * x * x;
            if (tx - alpha).abs() < EPSILON {
                *entry = coefficient * ((1.0 - x) * start_tension + x) + x * x * x;
                break;
            }
            if tx > alpha {
                x_max = x;
            } else {
                x_min = x;
            }
        }
    }
    position
}

#[cfg(test)]
mod tests {
    use super::Fling;
    use crate::core::Vector;

    // Golden numbers come from compiling the pinned, unmodified C++
    // mobile_scroller.cc and .h with clang++ -std=c++20 -ffp-contract=off.
    // mobile_scroller.cc SHA-256:
    // 4700c81b2c30fb647d250093901465ab843c5d93c1939109ef3b25cade4f641b
    // The standalone harness supplies only base/gfx interface stubs (integer
    // microsecond TimeDelta, vector storage, checks and ClampFloor). It invokes
    // Fling(0, 0, vx, vy, -FLT_MAX, FLT_MAX, -FLT_MAX, FLT_MAX, start) and
    // ComputeScrollOffset(start + Microseconds(frame * 1_000_000 / hz)).
    // Rows are frames 1, 8 and 30 at 60 Hz, then the same frames at 120 Hz.
    // These values are independent of this Rust implementation.
    #[test]
    fn matches_chromium_fast_slow_diagonal_and_reverse_at_60_and_120_hz() {
        assert_reference(
            Vector::new(0.0, 8000.0),
            2.566558,
            [0.0, 7186.36426],
            &[
                (16666, [0.0, 133.337051], [0.0, 7990.67383], false),
                (133333, [0.0, 1053.92578], [0.0, 7692.38135], false),
                (500000, [0.0, 3490.5769], [0.0, 5416.82812], false),
                (8333, [0.0, 66.7507629], [0.0, 7990.67383], false),
                (66666, [0.0, 531.669189], [0.0, 7934.42139], false),
                (250000, [0.0, 1921.4397], [0.0, 7157.94434], false),
            ],
        );
        assert_reference(
            Vector::new(0.0, 120.0),
            0.116537,
            [0.0, 4.89458132],
            &[
                (16666, [0.0, 1.84524822], [0.0, 94.5001373], false),
                (133333, [0.0, 4.89458132], [0.0, 0.0], true),
                (500000, [0.0, 4.89458132], [0.0, 0.0], true),
                (8333, [0.0, 0.977998674], [0.0, 111.66021], false),
                (66666, [0.0, 4.41810417], [0.0, 22.3279285], false),
                (250000, [0.0, 4.89458132], [0.0, 0.0], true),
            ],
        );
        assert_reference(
            Vector::new(1800.0, 2400.0),
            1.246591,
            [785.352661, 1047.13684],
            &[
                (
                    16666,
                    [29.9693298, 39.9591064],
                    [1794.93091, 2393.24121],
                    false,
                ),
                (
                    133333,
                    [228.830948, 305.10791],
                    [1572.80823, 2097.07764],
                    false,
                ),
                (
                    500000,
                    [609.654236, 812.872253],
                    [608.05542, 810.74054],
                    false,
                ),
                (
                    8333,
                    [14.9998913, 19.999855],
                    [1797.90161, 2397.20215],
                    false,
                ),
                (
                    66666,
                    [118.474861, 157.966476],
                    [1730.78589, 2307.7146],
                    false,
                ),
                (
                    250000,
                    [390.150574, 520.200745],
                    [1183.97656, 1578.6355],
                    false,
                ),
            ],
        );
        assert_reference(
            Vector::new(-1800.0, -2400.0),
            1.246591,
            [-785.352661, -1047.13684],
            &[
                (
                    16666,
                    [-29.9693298, -39.9591064],
                    [-1794.93091, -2393.24121],
                    false,
                ),
                (
                    133333,
                    [-228.830948, -305.10791],
                    [-1572.80823, -2097.07764],
                    false,
                ),
                (
                    500000,
                    [-609.654236, -812.872253],
                    [-608.05542, -810.74054],
                    false,
                ),
                (
                    8333,
                    [-14.9998913, -19.999855],
                    [-1797.90161, -2397.20215],
                    false,
                ),
                (
                    66666,
                    [-118.474861, -157.966476],
                    [-1730.78589, -2307.7146],
                    false,
                ),
                (
                    250000,
                    [-390.150574, -520.200745],
                    [-1183.97656, -1578.6355],
                    false,
                ),
            ],
        );
    }

    fn assert_reference(
        velocity: Vector,
        duration: f64,
        destination: [f64; 2],
        samples: &[(u64, [f64; 2], [f64; 2], bool)],
    ) {
        let fling = Fling::new(velocity).expect("nonzero reference velocity");
        assert_eq!(fling.duration(), duration);
        near(fling.displacement().x, destination[0]);
        near(fling.displacement().y, destination[1]);
        for &(micros, position, velocity, finished) in samples {
            let sample = fling.sample(micros as f64 / 1_000_000.0);
            near(sample.displacement.x, position[0]);
            near(sample.displacement.y, position[1]);
            near(sample.velocity.x, velocity[0]);
            near(sample.velocity.y, velocity[1]);
            assert_eq!(sample.finished, finished);
        }
        let end = fling.sample(duration);
        assert!(end.finished);
        assert_eq!(end.displacement, fling.displacement());
        assert_eq!(end.velocity, Vector::ZERO);
    }

    fn near(actual: f32, expected: f64) {
        assert!(
            (f64::from(actual) - expected).abs() <= 0.002,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn rejects_invalid_releases_without_clamping_valid_speeds() {
        assert!(Fling::new(Vector::ZERO).is_none());
        assert!(Fling::new(Vector::new(f32::NAN, 0.0)).is_none());
        assert!(Fling::new(Vector::new(f32::INFINITY, 0.0)).is_none());
        let slow = Fling::new(Vector::new(0.0, 1.0)).expect("slow fling");
        let fast = Fling::new(Vector::new(0.0, 100_000.0)).expect("fast fling");
        assert!(slow.duration() < 0.01);
        assert!(fast.duration() > 10.0);
        assert!(fast.displacement().y > 100_000.0);
    }

    #[test]
    fn starts_at_release_and_stops_at_chromium_endpoint_threshold() {
        let fling = Fling::new(Vector::new(0.0, 120.0)).expect("slow fling");
        let start = fling.sample(0.0);
        assert_eq!(start.displacement, Vector::ZERO);
        assert!(!start.finished);
        let before_duration = fling.sample(fling.duration() - 0.001);
        assert!(before_duration.finished);
        assert_eq!(before_duration.displacement, fling.displacement());
        assert_eq!(before_duration.velocity, Vector::ZERO);
    }
}
