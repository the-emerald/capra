use crate::deco::deco_algorithm::DecoAlgorithm;
use crate::common::dive_segment::DiveSegment;
use crate::common::gas::Gas;
use crate::common::tank::Tank;
use crate::planning::diveresult::DiveResult;

pub mod modes;
pub mod diveresult;

pub const PPO2_MINIMUM: f64 = 0.18;
pub const PPO2_MAXIMUM_DIVE: f64 = 1.4;
pub const PPO2_MAXIMUM_DECO: f64 = 1.6;

pub trait DivePlan<T: DecoAlgorithm> {
    fn plan(&self) -> DiveResult<T>;
    fn plan_backwards(&self, tanks: &[Tank]) -> DiveResult<T>; // Given some amount of gas, how long can we dive?
}

// TODO: Move this somewhere more appropriate
fn gas_in_ppo2_range(depth: usize, min: f64, max: f64, gas: &Gas) -> bool { // Checks if gas is in ppo2 range
    let gas_ppo2 = Gas::partial_pressure(depth, gas.fr_o2(), 10.0);
    gas_ppo2 >= min && gas_ppo2 <= max
}