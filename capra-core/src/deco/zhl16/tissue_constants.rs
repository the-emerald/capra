use crate::deco::zhl16::*;
use crate::deco::TISSUE_COUNT;

#[derive(Copy, Clone, Debug)]
pub struct TissueConstants {
    /// Nitrogen A-values.
    pub(crate) n2_a: [f64; TISSUE_COUNT],
    /// Nitrogen B-values.
    pub(crate) n2_b: [f64; TISSUE_COUNT],
    /// Nitrogen half-lives.
    pub(crate) n2_hl: [f64; TISSUE_COUNT],
    /// Helium A-values.
    pub(crate) he_a: [f64; TISSUE_COUNT],
    /// Helium B-values.
    pub(crate) he_b: [f64; TISSUE_COUNT],
    /// Helium half-lives.
    pub(crate) he_hl: [f64; TISSUE_COUNT],
}

impl TissueConstants {
    pub fn new(
        n2_a: [f64; TISSUE_COUNT],
        n2_b: [f64; TISSUE_COUNT],
        n2_hl: [f64; TISSUE_COUNT],
        he_a: [f64; TISSUE_COUNT],
        he_b: [f64; TISSUE_COUNT],
        he_hl: [f64; TISSUE_COUNT],
    ) -> Self {
        Self {
            n2_a,
            n2_b,
            n2_hl,
            he_a,
            he_b,
            he_hl,
        }
    }

    /// Returns a `TissueConstant` with the tissue loading constants of a defined variant.
    /// # Arguments
    /// * `variant` - Variant to use
    pub fn new_by_variant(variant: Variant) -> Self {
        match variant {
            Variant::B => Self::new(
                ZHL16B_N2_A,
                ZHL16B_N2_B,
                ZHL16B_N2_HALFLIFE,
                ZHL16B_HE_A,
                ZHL16B_HE_B,
                ZHL16B_HE_HALFLIFE,
            ),
            Variant::C => Self::new(
                ZHL16C_N2_A,
                ZHL16C_N2_B,
                ZHL16C_N2_HALFLIFE,
                ZHL16C_HE_A,
                ZHL16C_HE_B,
                ZHL16C_HE_HALFLIFE,
            ),
        }
    }
}
