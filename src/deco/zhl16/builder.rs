use crate::deco::zhl16::gradient_factor::GradientFactor;
use crate::deco::zhl16::tissue_constants::TissueConstants;
use crate::deco::zhl16::variant::Variant;
use crate::deco::zhl16::ZHL16;
use crate::tissue::Tissue;
use crate::units::depth::Depth;

#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZHL16Builder {
    tissue: Tissue,
    diver_depth: Depth,
    first_deco_depth: Option<Depth>,
    gradient_factor: GradientFactor,
    constants: TissueConstants,
}

impl ZHL16Builder {
    /// Create a new builder with default values (diver on the surface, using ZHL16-B).
    pub fn new() -> Self {
        Self {
            tissue: Tissue::default(),
            diver_depth: Depth(0),
            first_deco_depth: None,
            gradient_factor: GradientFactor::default(),
            constants: TissueConstants::new_by_variant(Variant::B),
        }
    }

    /// Set the tissue compartments to use.
    pub fn tissue(&mut self, tissue: Tissue) -> &mut Self {
        self.tissue = tissue;
        self
    }

    /// Set the current depth of the diver.
    pub fn current_depth(&mut self, depth: Depth) -> &mut Self {
        self.diver_depth = depth;
        self
    }

    /// Set the first deco depth of the diver.
    pub fn first_deco_depth(&mut self, depth: Depth) -> &mut Self {
        self.first_deco_depth = if depth == Depth(0) { None } else { Some(depth) };

        self
    }

    /// Set the gradient factor to use.
    pub fn gradient_factor(&mut self, gf: GradientFactor) -> &mut Self {
        self.gradient_factor = gf;
        self
    }

    /// Set the tissue constants to use by ZHL16 variant (B or C).
    pub fn variant(&mut self, variant: Variant) -> &mut Self {
        self.constants = TissueConstants::new_by_variant(variant);
        self
    }

    /// Set the tissue constants.
    pub fn tissue_constants(&mut self, constants: TissueConstants) -> &mut Self {
        self.constants = constants;
        self
    }

    /// Finish the builder and create a ZHL16 struct.
    pub fn finish(&mut self) -> ZHL16 {
        ZHL16 {
            tissue: self.tissue,
            tissue_constants: self.constants,
            diver_depth: self.diver_depth,
            first_deco_depth: self.first_deco_depth,
            gf: self.gradient_factor,
        }
    }
}

impl Default for ZHL16Builder {
    fn default() -> Self {
        Self::new()
    }
}
