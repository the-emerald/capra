use time::Duration;

pub const DEFAULT_DESCENT_RATE: isize = 30;
pub const DEFAULT_ASCENT_RATE: isize = -18;

pub mod dive_segment;
pub mod gas;

pub fn bar_mtr(bar: f64) -> f64 {
    (bar-1.0) * 10.0
}

pub fn mtr_bar(mtr: f64) -> f64 {
    (mtr/10.0) + 1.0
}

pub fn time_taken(rate: isize, depth_1: usize, depth_2: usize) -> Duration {
    let delta_depth = ((depth_1 as isize) - (depth_2 as isize)).abs();
    let rate_seconds: f64 = rate.abs() as f64 / 60.0;
    Duration::seconds(
        ((1.0 / rate_seconds) * delta_depth as f64) as i64
    )
}