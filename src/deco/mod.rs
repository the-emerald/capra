//! Decompression models

pub mod deco_algorithm;

pub mod zhl16;

pub use deco_algorithm::DecoAlgorithm;
pub use tissue::Tissue;
use crate::common::pressure::Pressure;

/// Number of tissues in a typical decompression algorithm.
pub const TISSUE_COUNT: usize = 16;
/// Pressure of water vapour. (measured in bar)
pub const WATER_VAPOUR_PRESSURE: Pressure = Pressure(0.06257);
