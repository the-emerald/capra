use crate::deco::deco_algorithm::DecoAlgorithm;
use crate::common::tank::Tank;
use crate::planning::dive_result::DiveResult;

pub mod modes;
pub mod dive_result;
pub mod otu;

pub const PPO2_MINIMUM: f64 = 0.18;
pub const PPO2_MAXIMUM_DIVE: f64 = 1.4;
pub const PPO2_MAXIMUM_DECO: f64 = 1.6;

pub trait DivePlan<T: DecoAlgorithm> {
    fn plan(&self) -> DiveResult<T>;
    fn plan_backwards(&self, tanks: &[Tank]) -> DiveResult<T>; // Given some amount of gas, how long can we dive?
}