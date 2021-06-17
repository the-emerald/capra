use crate::units::depth::Depth;
use crate::units::rate::Rate;
use time::Duration;

/// Helper function to calculate the time taken to change depths, given a rate.
/// # Arguments
/// * `rate` - Rate of depth change
/// * `depth_1` - First depth
/// * `depth_2` - Second depth
/// # Panics
/// Function will panic if the time taken exceeds [`i64::MAX`].
pub fn time_taken(rate: Rate, depth_1: Depth, depth_2: Depth) -> Duration {
    let delta_depth = depth_1.delta(depth_2);
    let rate_seconds = rate.0.abs() as f64 / 60.0;
    Duration::seconds((delta_depth.0 as f64 / rate_seconds) as i64)
}
