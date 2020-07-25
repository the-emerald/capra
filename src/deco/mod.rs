//! Decompression models

pub mod zhl16;
pub mod deco_algorithm;
pub mod tissue;

pub use deco_algorithm::DecoAlgorithm;

pub use tissue::Tissue;

/// Number of tissues in a typical decompression algorithm.
pub const TISSUE_COUNT: usize = 16;
/// Pressure of water vapour. (measured in bar)
pub const WATER_VAPOUR_PRESSURE: f64 = 0.06257;