pub const DESCENT_RATE: isize = 30;
pub const ASCENT_RATE: isize = -18;

pub mod deco_algorithm;
pub mod gas;
pub mod deco_stop;

pub fn bar_mtr(bar: f32) -> f32 {
    (bar-1.0) * 10.0
}

pub fn mtr_bar(mtr: f32) -> f32 {
    (mtr/10.0) + 1.0
}