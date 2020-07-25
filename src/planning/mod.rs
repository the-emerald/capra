//! Items related to dive planning and the application of decompression models

use crate::common::tank::Tank;
use crate::deco::deco_algorithm::DecoAlgorithm;

pub mod dive_result;
pub mod modes;
pub mod otu;

pub use dive_result::DiveResult;
pub use otu::otu;

/// A default, placeholder minimum ppO2.
pub const PPO2_MINIMUM: f64 = 0.18;

/// A default, placeholder maximum ppO2 for use during bottom segments.
pub const PPO2_MAXIMUM_DIVE: f64 = 1.4;

/// A default, placeholder maximum ppO2 for use during decompression stops.
pub const PPO2_MAXIMUM_DECO: f64 = 1.6;

/// Trait for dive planning structs.
pub trait DivePlan<T: DecoAlgorithm> {
    /// Run the dive plan, returning a `DiveResult` that contains the results.
    fn plan(&self) -> DiveResult<T>;

    /// Run the dive plan "backwards". Given the amount of gas in the tanks, how much of the dive plan
    /// can actually be done?
    fn plan_backwards(&self, tanks: &[Tank]) -> DiveResult<T>; // Given some amount of gas, how long can we dive?
}
