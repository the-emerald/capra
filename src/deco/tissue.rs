use crate::deco::{TISSUE_COUNT, WATER_VAPOUR_PRESSURE};
use crate::gas;
use crate::common::gas::Gas;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tissue {
    // Tissue states
    pub(crate) p_n2: [f64; TISSUE_COUNT],
    pub(crate) p_he: [f64; TISSUE_COUNT],
    pub(crate) p_t: [f64; TISSUE_COUNT],
}

impl Default for Tissue {
    fn default() -> Self {
        let air = gas!(21, 0);
        let adj_fr_n2 = air.fr_n2() * (1.0 - WATER_VAPOUR_PRESSURE);
        Self {
            p_n2: [adj_fr_n2; TISSUE_COUNT],
            p_he: [0.0; TISSUE_COUNT],
            p_t: [adj_fr_n2; TISSUE_COUNT]
        }
    }
}