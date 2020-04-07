pub const DEFAULT_DESCENT_RATE: isize = 30;
pub const DEFAULT_ASCENT_RATE: isize = -18;

pub mod deco_algorithm;
pub mod dive_segment;

pub fn bar_mtr(bar: f64) -> f64 {
    (bar-1.0) * 10.0
}

pub fn mtr_bar(mtr: f64) -> f64 {
    (mtr/10.0) + 1.0
}