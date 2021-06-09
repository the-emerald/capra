use crate::deco::zhl16::gradient_factor::GradientFactor;
use crate::deco::zhl16::tissue_constants::TissueConstants;
use crate::deco::DecoAlgorithm;
use crate::environment::Environment;
use crate::gas::Gas;
use crate::segment::{Segment, SegmentType};
use crate::tissue::Tissue;
use crate::units::depth::Depth;
use crate::units::pressure::Pressure;
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
    fn add_flat_segment_inner(&mut self, segment: &Segment, gas: &Gas, environment: Environment) {
        for (pressure, half_life) in self
            .tissue
            .p_n2()
            .iter_mut()
            .zip(self.tissue_constants.n2_hl().iter())
        {
            let pi = segment.end_depth().compensated_pressure(density) * Pressure(gas.fr_n2());
            *pressure = ZHL16::flat_loading(
                *pressure,
                pi,
                segment.time().whole_minutes() as f64,
                *half_life,
            );
        }

        for (pressure, half_life) in self
            .tissue
            .p_he()
            .iter_mut()
            .zip(self.tissue_constants.he_hl().iter())
        {
            let pi = segment.end_depth().compensated_pressure(density) * Pressure(gas.fr_n2());
            *pressure = ZHL16::flat_loading(
                *pressure,
                pi,
                segment.time().whole_minutes() as f64,
                *half_life,
            );
        }
    }

    fn flat_loading(po: Pressure, pi: Pressure, time: f64, half_life: f64) -> Pressure {
        po + (pi - po) * Pressure(1.0 - (2.0_f64.powf(-1.0 * time / half_life)))
    }

    fn add_depth_change_segment_inner(
        &mut self,
        segment: &Segment,
        gas: &Gas,
        environment: Environment,
    ) {
        let delta = segment.end_depth().delta(segment.start_depth());
        let rate = if delta > Depth(0) {
            segment.descent_rate()
        } else {
            segment.ascent_rate()
        };

        let time_fr = segment.time().whole_seconds() as f64 / 60.0;

        // Nitrogen
        for (pressure, half_life) in self
            .tissue
            .p_n2()
            .iter_mut()
            .zip(self.tissue_constants.n2_hl().iter())
        {
            let pio =
                segment.start_depth().compensated_pressure(environment) * Pressure(gas.fr_n2());
            let r = (rate.0 as f64 / 10.0) * gas.fr_n2();
            let k = LN_2 / half_life;

            *pressure = ZHL16::depth_change_loading(time_fr, *pressure, pio, r, k);
        }

        // Helium
        for (pressure, half_life) in self
            .tissue
            .p_he()
            .iter_mut()
            .zip(self.tissue_constants.n2_hl().iter())
        {
            let pio =
                segment.start_depth().compensated_pressure(environment) * Pressure(gas.fr_he());
            let r = (rate.0 as f64 / 10.0) * gas.fr_he();
            let k = LN_2 / half_life;

            *pressure = ZHL16::depth_change_loading(time_fr, *pressure, pio, r, k);
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
        self.first_deco_depth = self.first_deco_depth.or(Some(depth));
    }
}

impl DecoAlgorithm for ZHL16 {
    fn add_segment(mut self, segment: &Segment, gas: &Gas, environment: Environment) -> Self {
        match segment.segment_type() {
            SegmentType::NoDeco => panic!("no-deco segment applied to deco algorithm"),
            SegmentType::DecoStop => {
                self.add_flat_segment_inner(segment, gas, environment);
                self.set_first_deco_depth(segment.start_depth());
            }
            SegmentType::Bottom => {
                self.add_flat_segment_inner(segment, gas, environment);
            }
            SegmentType::AscDesc => {
                self.add_depth_change_segment_inner(segment, gas, environment);
            }
        }
        self.diver_depth = segment.end_depth();

        self
    }

    fn ascent_ceiling(&self) -> Pressure {
        todo!()
    }
}
