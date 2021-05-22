use crate::deco::zhl16::gradient_factor::GradientFactor;
use crate::deco::zhl16::tissue_constants::TissueConstants;
use crate::tissue::Tissue;
use crate::units::depth::Depth;

pub mod builder;
pub mod gradient_factor;
pub mod tissue_constants;
pub mod variant;

pub struct ZHL16 {
    /// Current tissue model of the diver.
    tissue: Tissue,
    /// Tissue constants of the diver
    tissue_constants: TissueConstants,
    /// Current depth of the diver.
    diver_depth: Depth,
    /// First deco depth of the diver
    first_deco_depth: Option<Depth>,
    /// GF value
    gf: GradientFactor,
}

// TODO: ZHL16 Implementation

// TODO: ZHL16 trait implementation
