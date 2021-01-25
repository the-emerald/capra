//! Dive planning library
//! # Quick example
//! A quick example showing how to use this library:
//! ```
//! use capra::modes::open_circuit::OpenCircuitParams;
//! use capra_core::{common, deco};
//! use time::Duration;
//! use capra::modes::OpenCircuit;
//! use capra::DivePlan;
//! use capra_core::deco::zhl16::tissue_constants::TissueConstants;
//! use capra_core::deco::zhl16::variant::Variant::B;
//! let air = common::Gas::new(21, 0, 79).unwrap();
//!
//! // Make a new ZHL16 decompression model
//! let zhl16 = deco::zhl16::ZHL16::new(
//!     deco::Tissue::default(),
//!     TissueConstants::new_by_variant(B),
//!     50,
//!     70,
//! );
//!
//! // Make a dive segment
//! let dive_segment = common::DiveSegment::new(
//!     common::SegmentType::DiveSegment,
//!     45,
//!     45,
//!     Duration::minutes(60),
//!     -10,
//!     20,
//! )
//! .unwrap();
//!
//! let deco_gases = vec![(air, None)];
//! let segments = vec![(dive_segment, air)];
//! let parameters = OpenCircuitParams {
//!     ascent_rate: -10,
//!     descent_rate: 20,
//!     metres_per_bar: 10000.0 / common::DENSITY_SALTWATER,
//!     sac_bottom: 20,
//!     sac_deco: 15
//! };
//! let open_circuit = OpenCircuit::new(
//!     zhl16,
//!     &deco_gases,
//!     &segments,
//!     parameters
//! );
//!
//! let results = open_circuit.plan();
//!
//! for (segment, gas) in results.total_segments() {
//!     println!("{:?}, {:?}", segment, gas);
//! }
//!
//! for (gas, qty) in results.gas_used() {
//!     println!("{:?}: {}", gas, qty);
//! }
//! ```

pub mod plan;
pub mod result;
pub mod modes;
pub mod parameters;

pub use plan::DivePlan;
pub use result::DiveResult;

/// A default, placeholder minimum ppO2.
pub const PPO2_MINIMUM: f64 = 0.18;

/// A default, placeholder maximum ppO2 for use during bottom segments.
pub const PPO2_MAXIMUM_DIVE: f64 = 1.4;

/// A default, placeholder maximum ppO2 for use during decompression stops.
pub const PPO2_MAXIMUM_DECO: f64 = 1.6;
