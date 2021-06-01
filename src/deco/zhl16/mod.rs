use crate::deco::zhl16::gradient_factor::GradientFactor;
use crate::deco::zhl16::tissue_constants::TissueConstants;
use crate::deco::DecoAlgorithm;
use crate::gas::Gas;
use crate::segment::{Segment, SegmentType};
use crate::tissue::Tissue;
use crate::units::depth::Depth;
use crate::units::pressure::Pressure;
use crate::units::water_density::WaterDensity;
use std::f64::consts::{E, LN_2};

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

impl ZHL16 {
    fn add_flat_segment_inner(&mut self, segment: &Segment, gas: &Gas, density: WaterDensity) {
        todo!()
    }

    fn add_depth_change_segment_inner(
        &mut self,
        segment: &Segment,
        gas: &Gas,
        density: WaterDensity,
    ) {
        let delta = segment.end_depth().delta(segment.start_depth());
        let rate = if delta > Depth(0) {
            segment.descent_rate()
        } else {
            segment.ascent_rate()
        };

        let time_fr = segment.time().whole_seconds() as f64 / 60.0;

        // Nitrogen
        for (idx, val) in self.tissue.p_n2().iter_mut().enumerate() {
            // Initial value
            let po = *val;
            let pio = segment.start_depth().compensated_pressure(density) * Pressure(gas.fr_n2());

            let r = (rate.0 as f64 / 10.0) * gas.fr_n2();
            let k = LN_2 / self.tissue_constants.n2_hl()[idx];

            *val = ZHL16::depth_change_loading(time_fr, po, pio, r, k);
        }

        // Helium
        for (idx, val) in self.tissue.p_he().iter_mut().enumerate() {
            // Initial value
            let po = *val;
            let pio = segment.start_depth().compensated_pressure(density) * Pressure(gas.fr_he());

            let r = (rate.0 as f64 / 10.0) * gas.fr_he();
            let k = LN_2 / self.tissue_constants.he_hl()[idx];

            *val = ZHL16::depth_change_loading(time_fr, po, pio, r, k);
        }

        self.diver_depth = segment.end_depth();
    }

    fn depth_change_loading(
        time: f64,
        initial: Pressure,
        initial_ambient: Pressure,
        r: f64,
        k: f64,
    ) -> Pressure {
        Pressure(
            initial_ambient.0 + r * (time - (1.0 / k))
                - ((initial_ambient.0 - initial.0 - (r / k)) * E.powf(-1.0 * k * time)),
        )
    }

    fn set_first_deco_depth(&mut self, depth: Depth) {
        match self.first_deco_depth {
            None => self.first_deco_depth = Some(depth),
            Some(_) => {}
        }
    }
}

impl DecoAlgorithm for ZHL16 {
    fn add_segment(mut self, segment: &Segment, gas: &Gas, density: WaterDensity) -> Self {
        match segment.segment_type() {
            SegmentType::NoDeco => panic!("no-deco segment applied to deco algorithm"),
            SegmentType::DecoStop => {
                self.add_flat_segment_inner(segment, gas, density);
                self.set_first_deco_depth(segment.start_depth());
            }
            SegmentType::Bottom => {
                self.add_flat_segment_inner(segment, gas, density);
            }
            SegmentType::AscDesc => {
                self.add_depth_change_segment_inner(segment, gas, density);
            }
        }
        self
    }

    fn ascent_ceiling(&self) -> Pressure {
        todo!()
    }
}
