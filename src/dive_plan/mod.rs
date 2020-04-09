use crate::common::gas::{Gas, partial_pressure};

pub mod dive;
pub mod open_circuit;

pub const PPO2_MINIMUM: f64 = 0.18;
pub const PPO2_MAXIMUM_DIVE: f64 = 1.4;
pub const PPO2_MAXIMUM_DECO: f64 = 1.6;

fn equivalent_narcotic_depth(depth: usize, gas: &Gas) -> usize { // Returns a depth
    (((depth + 10) as f64 * (1.0 - gas.fr_he())) - 10.0) as usize
}

fn gas_in_ppo2_range(depth: usize, min: f64, max: f64, gas: &Gas, metres_per_bar: f64) -> bool { // Checks if gas is in ppo2 range
    let gas_ppo2 = partial_pressure(depth, gas.fr_o2(), 10.0);
    println!("{}m, {} max ppO2, {} calculated ppO2, {:?}, result: {}", depth, max, gas_ppo2, gas, gas_ppo2 >= min && gas_ppo2 <= max);

    gas_ppo2 >= min && gas_ppo2 <= max
}