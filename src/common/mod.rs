//! Commonly used items for decompression models and dive planning

use num_traits::cast::FromPrimitive;
use std::isize;
use time::Duration;

/// A default, placeholder descent rate (measured in m min^-1).
pub const DEFAULT_DESCENT_RATE: isize = 30;

/// A default, placeholder ascent rate (measured in m min^-1). This is the maximum rate recommended by major instruction agencies.
pub const DEFAULT_ASCENT_RATE: isize = -18;

/// Density of fresh water (measured in kg m^-3).
pub const DENSITY_FRESHWATER: f64 = 997.0;

/// Average density of salt water (measured in kg m^-3).
pub const DENSITY_SALTWATER: f64 = 1023.6;

pub mod dive_segment;
pub mod gas;
pub mod tank;

pub use dive_segment::DiveSegment;
pub use dive_segment::DiveSegmentError;
pub use dive_segment::SegmentType;

pub use gas::Gas;
pub use gas::GasError;

pub use tank::Tank;

/// Helper function to convert pressure to the equivalent depth of water that would induce it.
/// # Arguments
/// * `bar` - Pressure measured in bars
/// * `metres_per_bar` - Depth of water required to induce 1 bar of pressure
pub fn bar_mtr(bar: f64, metres_per_bar: f64) -> f64 {
    (bar - 1.0) * metres_per_bar
}

/// Helper function to convert a depth of water to the pressure it will induce.
/// # Arguments
/// * `mtr` - Depth of water.
/// * `metres_per_bar` - Depth of water required to induce 1 bar of pressure.
pub fn mtr_bar(mtr: f64, metres_per_bar: f64) -> f64 {
    (mtr / metres_per_bar) + 1.0
}

/// Helper function to calculate the time taken to change depths, given a rate.
/// # Arguments
/// * `rate` - Rate of depth change
/// * `depth_1` - First depth
/// * `depth_2` - Second depth
/// # Panics
/// Function will panic if the time taken exceeds [`i64::MAX`].
pub fn time_taken(rate: isize, depth_1: usize, depth_2: usize) -> Duration {
    let delta_depth = ((depth_1 as isize) - (depth_2 as isize)).abs();
    let rate_seconds = rate.abs() as f64 / 60.0;
    Duration::seconds(
        i64::from_f64(delta_depth as f64 / rate_seconds).expect("overflow in time taken"),
    )
}
