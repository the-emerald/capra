use crate::common::gas::Gas;
use crate::deco::{TISSUE_COUNT, WATER_VAPOUR_PRESSURE};
use crate::gas;
use crate::common::pressure::Pressure;

/// A set of tissues for use in decompression models, comprising a set of tissues for nitrogen
/// and another set for helium.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tissue {
    // Tissue states
    pub(crate) p_n2: [Pressure; TISSUE_COUNT],
    pub(crate) p_he: [Pressure; TISSUE_COUNT],
    pub(crate) p_t: [Pressure; TISSUE_COUNT],
}

impl Tissue {
    /// Returns a new Tissue with the given parameters.
    /// # Arguments
    /// * `p_n2` - Set of tissues for nitrogen
    /// * `p_he` - Set of tissues for nitrogen
    /// * `p_t` - Total pressure of all tissues
    pub fn new(
        p_n2: [Pressure; TISSUE_COUNT],
        p_he: [Pressure; TISSUE_COUNT],
        p_t: [Pressure; TISSUE_COUNT],
    ) -> Self {
        Self { p_n2, p_he, p_t }
    }

    pub fn p_n2(&self) -> [Pressure; TISSUE_COUNT] {
        self.p_n2
    }

    pub fn p_he(&self) -> [Pressure; TISSUE_COUNT] {
        self.p_n2
    }

    // TODO: Methods to work with tissues
}

impl Default for Tissue {
    /// A default value for tissues. This is the tissue loading of a diver who has been breathing
    /// air at 1 atm for a long time.
    fn default() -> Self {
        let air = gas!(21, 0);
        let adj_fr_n2 = Pressure(air.fr_n2() * (1.0 - WATER_VAPOUR_PRESSURE.0));
        Self {
            p_n2: [adj_fr_n2; TISSUE_COUNT],
            p_he: [Pressure(0.0); TISSUE_COUNT],
            p_t: [adj_fr_n2; TISSUE_COUNT],
        }
    }
}
