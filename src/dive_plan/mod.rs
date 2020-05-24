use crate::common::gas::{Gas, partial_pressure};

pub mod dive;
pub mod open_circuit;

pub const PPO2_MINIMUM: f64 = 0.18;
pub const PPO2_MAXIMUM_DIVE: f64 = 1.4;
pub const PPO2_MAXIMUM_DECO: f64 = 1.6;

fn gas_in_ppo2_range(depth: usize, min: f64, max: f64, gas: &Gas, metres_per_bar: f64) -> bool { // Checks if gas is in ppo2 range
    let gas_ppo2 = partial_pressure(depth, gas.fr_o2(), 10.0);
    gas_ppo2 >= min && gas_ppo2 <= max
}