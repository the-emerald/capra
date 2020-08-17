//! Diver decompression and dive planning library.
//! # Quick example
//! A quick example showing how to use this library:
//! ```
//! use capra::planning::DivePlan;
//! use capra::{common, deco, planning};
//! use time::Duration;
//!
//! fn main() {
//!     // Make a new gas
//!     use capra::common::dive_segment::SegmentType::AscDesc;
//! let air = common::Gas::new(21, 0, 79).unwrap();
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
//!     let open_circuit = planning::modes::OpenCircuit::new(
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

#![cfg_attr(not(feature = "std"), allow(unused_imports), allow(dead_code))]

pub mod common;
pub mod deco;
pub mod planning;
