use crate::result::DiveResult;
use capra_core::common::tank::Tank;
use capra_core::deco::deco_algorithm::DecoAlgorithm;

/// Trait for dive planning structs.
pub trait DivePlan<T: DecoAlgorithm> {
    /// Run the dive plan, returning a `DiveResult` that contains the results.
    fn plan(&self) -> DiveResult<T>;

    /// Run the dive plan "backwards". Given the amount of gas in the tanks, how much of the dive plan
    /// can actually be done?
    fn plan_backwards(&self, tanks: &[Tank]) -> DiveResult<T>; // Given some amount of gas, how long can we dive?
}
