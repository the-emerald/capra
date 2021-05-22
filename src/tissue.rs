use crate::deco::TISSUE_COUNT;
use crate::gas::Gas;
use crate::units::pressure::{Pressure, WATER_VAPOUR_PRESSURE};

pub struct Tissue {
    p_n2: [Pressure; TISSUE_COUNT],
    p_he: [Pressure; TISSUE_COUNT],
}

impl Tissue {
    pub fn new(p_n2: [Pressure; TISSUE_COUNT], p_he: [Pressure; TISSUE_COUNT]) -> Self {
        Self { p_n2, p_he }
    }

    pub fn p_n2(&self) -> &mut [Pressure; TISSUE_COUNT] {
        todo!()
    }

    pub fn p_he(&self) -> &mut [Pressure; TISSUE_COUNT] {
        todo!()
    }
}

impl Default for Tissue {
    fn default() -> Self {
        let air = Gas::new(21, 0, 79).unwrap();
        let adj_fr_n2 = Pressure(air.fr_n2() * (1.0 - WATER_VAPOUR_PRESSURE.0));
        Self {
            p_n2: [adj_fr_n2; TISSUE_COUNT],
            p_he: [Pressure::default(); TISSUE_COUNT],
        }
    }
}
