//! Dive planning library with support for open circuit dives

pub mod dive_result;
pub mod modes;
pub mod dive_plan;

pub use dive_plan::DivePlan;
pub use dive_result::DiveResult;

/// A default, placeholder minimum ppO2.
pub const PPO2_MINIMUM: f64 = 0.18;

/// A default, placeholder maximum ppO2 for use during bottom segments.
pub const PPO2_MAXIMUM_DIVE: f64 = 1.4;

/// A default, placeholder maximum ppO2 for use during decompression stops.
pub const PPO2_MAXIMUM_DECO: f64 = 1.6;

