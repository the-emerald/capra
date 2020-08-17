//! Dive planning library
//! # Quick example
//! A quick example showing how to use this library:
//! ```
//! use time::Duration;
//! use capra_core::{common, deco};
//! use capra::DivePlan;
//! use capra::modes::OpenCircuit;
//!
//! fn main() {
//!     // Make a new gas
//!     let air = common::Gas::new(21, 0, 79).unwrap();
//!
//!     // Make a new ZHL16 decompression model
//!     let zhl16 = deco::zhl16::ZHL16::new_by_variant(
//!         deco::Tissue::default(),
//!         50,
//!         70,
//!         deco::zhl16::Variant::B,
//!     );
//!
//!     // Make a dive segment
//!     let dive_segment = common::DiveSegment::new(
//!         common::SegmentType::DiveSegment,
//!         45,
//!         45,
//!         Duration::minutes(60),
//!         -10,
//!         20,
//!     )
//!     .unwrap();
//!
//!     let deco_gases = vec![(air, None)];
//!     let segments = vec![(dive_segment, air)];
//!     let open_circuit = OpenCircuit::new(
//!         zhl16,
//!         &deco_gases,
//!         &segments,
//!         -10,
//!         20,
//!         common::DENSITY_SALTWATER,
//!         20,
//!         15,
//!     );
//!
//!     let results = open_circuit.plan();
//!
//!     for (segment, gas) in results.total_segments() {
//!         println!("{:?}, {:?}", segment, gas);
//!     }
//!
//!     for (gas, qty) in results.gas_used() {
//!         println!("{:?}: {}", gas, qty);
//!     }
//! }
//! ```

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

