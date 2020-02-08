pub const DEFAULT_DESCENT_RATE: isize = 30;
pub const DEFAULT_ASCENT_RATE: isize = -18;

pub mod deco_algorithm;
pub mod gas;
pub mod dive_segment;
pub mod otu;

pub fn bar_mtr(bar: f32) -> f32 {
    (bar-1.0) * 10.0
}

pub fn mtr_bar(mtr: f32) -> f32 {
    (mtr/10.0) + 1.0
}