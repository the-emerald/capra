use crate::deco::zhl16::gradient_factor::GradientFactor;
use crate::deco::zhl16::tissue_constants::TissueConstants;
use crate::deco::zhl16::variant::Variant;
use crate::deco::zhl16::ZHL16;
use crate::tissue::Tissue;
use crate::units::depth::Depth;

pub struct ZHL16Builder {
    tissue: Option<Tissue>,
    diver_depth: Depth,
    first_deco_depth: Option<Depth>,
    gradient_factor: GradientFactor,
    constants: Option<TissueConstants>,
}

impl ZHL16Builder {
    pub fn new() -> Self {
        Self {
            tissue: None,
            diver_depth: Depth(0),
            first_deco_depth: None,
            gradient_factor: GradientFactor::default(),
            constants: None,
        }
    }

    pub fn tissue(&mut self, tissue: Tissue) -> &mut Self {
        self.tissue = Some(tissue);
        self
    }

    pub fn current_depth(&mut self, depth: Depth) -> &mut Self {
        self.diver_depth = depth;
        self
    }

    pub fn first_deco_depth(&mut self, depth: Depth) -> &mut Self {
        self.first_deco_depth = if depth == Depth(0) { None } else { Some(depth) };

        self
    }

    pub fn gradient_factor(&mut self, gf: GradientFactor) -> &mut Self {
        self.gradient_factor = gf;
        self
    }

    pub fn variant(&mut self, variant: Variant) -> &mut Self {
        self.constants = Some(TissueConstants::new_by_variant(variant));
        self
    }

    pub fn tissue_constants(&mut self, constants: TissueConstants) -> &mut Self {
        self.constants = Some(constants);
        self
    }

    pub fn finish(self) -> ZHL16 {
        ZHL16 {
            tissue: self.tissue.unwrap(),
            tissue_constants: self.constants.unwrap(),
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