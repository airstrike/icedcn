//! Chromium's unweighted second-order least-squares touch velocity estimator.
//!
//! Ported from ui/events/velocity_tracker/velocity_tracker.{cc,h}, Chromium
//! 37d10a51f02e32921290fd80cc0bdaa615ac25fa (BSD-3-Clause). The single-contact
//! history, stop thresholds, fit order, QR solver and singularity threshold
//! follow that source. Pointer coordinates and seconds produce logical px/s.

// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in assets/Chromium-LICENSE.

use crate::core::time::{Duration, Instant};
use crate::core::{Point, Vector};

const HISTORY: usize = 20;
const HORIZON: Duration = Duration::from_millis(100);
const MOVE_STOPPED: Duration = Duration::from_millis(40);
const UP_STOPPED: Duration = Duration::from_millis(80);

/// The bounded motion history of one touch contact.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Tracker {
    samples: [Option<Sample>; HISTORY],
    next: usize,
    len: usize,
}

#[derive(Debug, Clone, Copy)]
struct Sample {
    position: Point,
    time: Instant,
}

impl Tracker {
    /// Records a down or move position at its event timestamp.
    pub(super) fn sample(&mut self, position: Point, time: Instant) {
        if self
            .latest()
            .is_some_and(|last| time.saturating_duration_since(last.time) >= MOVE_STOPPED)
        {
            *self = Self::default();
        }

        self.samples[self.next] = Some(Sample { position, time });
        self.next = (self.next + 1) % HISTORY;
        self.len = (self.len + 1).min(HISTORY);
    }

    /// Estimates pointer velocity at the last move, unless release was delayed.
    pub(super) fn velocity(&self, release: Instant) -> Vector {
        let Some(last) = self.latest() else {
            return Vector::ZERO;
        };
        if release.saturating_duration_since(last.time) >= UP_STOPPED {
            return Vector::ZERO;
        }

        let mut time = [0.; HISTORY];
        let mut x = [0.; HISTORY];
        let mut y = [0.; HISTORY];
        let mut count = 0;
        for offset in 0..self.len {
            let index = (self.next + HISTORY - 1 - offset) % HISTORY;
            let Some(sample) = self.samples[index] else {
                break;
            };
            let age = last.time.saturating_duration_since(sample.time);
            if age > HORIZON {
                break;
            }
            time[count] = -(age.as_secs_f64() as f32);
            x[count] = sample.position.x;
            y[count] = sample.position.y;
            count += 1;
        }

        if count < 2 {
            return Vector::ZERO;
        }
        let columns = count.min(3);
        match (
            solve(&time[..count], &x[..count], columns),
            solve(&time[..count], &y[..count], columns),
        ) {
            (Some(x), Some(y)) => Vector::new(x, y),
            _ => Vector::ZERO,
        }
    }

    fn latest(&self) -> Option<Sample> {
        self.samples[(self.next + HISTORY - 1) % HISTORY]
    }
}

/// Chromium's Gram-Schmidt QR solve, specialized to unit weights and degree <= 2.
fn solve(time: &[f32], position: &[f32], columns: usize) -> Option<f32> {
    let count = time.len();
    let mut a = [[0.; HISTORY]; 3];
    for (h, time) in time.iter().enumerate() {
        a[0][h] = 1.;
        for i in 1..columns {
            a[i][h] = a[i - 1][h] * time;
        }
    }

    let mut q = [[0.; HISTORY]; 3];
    let mut r = [[0.; 3]; 3];
    for j in 0..columns {
        q[j][..count].copy_from_slice(&a[j][..count]);
        for i in 0..j {
            let projection = dot(&q[i][..count], &q[j][..count]);
            for h in 0..count {
                q[j][h] -= projection * q[i][h];
            }
        }

        let norm = dot(&q[j][..count], &q[j][..count]).sqrt();
        if norm < 0.000001 {
            return None;
        }
        let inverse = 1. / norm;
        for h in 0..count {
            q[j][h] *= inverse;
        }
        for i in j..columns {
            r[j][i] = dot(&q[j][..count], &a[i][..count]);
        }
    }

    let mut coefficients = [0.; 3];
    for i in (0..columns).rev() {
        coefficients[i] = dot(&q[i][..count], position);
        for j in ((i + 1)..columns).rev() {
            coefficients[i] -= r[i][j] * coefficients[j];
        }
        coefficients[i] /= r[i][i];
    }
    Some(coefficients[1])
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).fold(0., |sum, (a, b)| sum + a * b)
}

#[cfg(test)]
mod tests {
    use super::{Duration, Instant, Point, Tracker, Vector};

    fn trace(samples: &[(u64, f32)], release_ms: u64) -> Vector {
        let start = Instant::now();
        let mut tracker = Tracker::default();
        for &(ms, x) in samples {
            tracker.sample(Point::new(x, -x * 2.), start + Duration::from_millis(ms));
        }
        tracker.velocity(start + Duration::from_millis(release_ms))
    }

    fn close(actual: Vector, expected: f32) {
        assert!(
            (actual.x - expected).abs() < 0.02,
            "{actual:?} != {expected}"
        );
        assert!((actual.y + expected * 2.).abs() < 0.04);
    }

    #[test]
    fn constant_motion_keeps_velocity_until_release_stop_threshold() {
        let samples = [(0, 0.), (10, 10.), (20, 20.), (30, 30.)];
        close(trace(&samples, 30), 1000.);
        close(trace(&samples, 109), 1000.);
        assert_eq!(trace(&samples, 110), Vector::ZERO);
    }

    #[test]
    fn acceleration_and_reversal_follow_endpoint_derivative() {
        close(trace(&[(0, 0.), (10, 1.), (20, 4.), (30, 9.)], 30), 600.);
        close(
            trace(&[(0, 0.), (10, 5.), (20, 8.), (30, 9.), (40, 8.)], 40),
            -200.,
        );
    }

    #[test]
    fn irregular_motion_matches_chromium_cpp_solver() {
        // Golden value from the pinned C++ SolveLeastSquares, run with unit
        // weights and the same newest-first times and positions.
        close(
            trace(&[(0, 0.), (9, 4.), (21, 12.), (31, 13.)], 31),
            177.209_18,
        );
    }

    #[test]
    fn movement_stop_discards_previous_history() {
        assert_eq!(trace(&[(0, 0.), (10, 10.), (50, 20.)], 50), Vector::ZERO);
        close(trace(&[(0, 0.), (10, 10.), (50, 20.), (60, 25.)], 60), 500.);
    }

    #[test]
    fn duplicate_timestamps_have_no_invented_elapsed_time() {
        assert_eq!(trace(&[(10, 0.), (10, 10.), (10, 20.)], 10), Vector::ZERO);
        // Three samples request LSQ2; two distinct times cannot determine it.
        assert_eq!(trace(&[(0, 0.), (10, 10.), (10, 20.)], 10), Vector::ZERO);
    }

    #[test]
    fn history_is_limited_by_both_time_and_capacity() {
        let mut samples = vec![(0, 1000.)];
        samples.extend((1..=12).map(|i| (i * 10, (i * 10) as f32)));
        close(trace(&samples, 120), 1000.);

        let mut samples = (0..5).map(|i| (i, 1000.)).collect::<Vec<_>>();
        samples.extend((5..25).map(|i| (i, i as f32)));
        close(trace(&samples, 24), 1000.);
    }
}
