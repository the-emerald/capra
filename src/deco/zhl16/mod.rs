use crate::deco::zhl16::gradient_factor::GradientFactor;
use crate::deco::zhl16::tissue_constants::TissueConstants;
use crate::deco::{DecoAlgorithm, TISSUE_COUNT};
use crate::environment::Environment;
use crate::gas::Gas;
use crate::segment::{Segment, SegmentType};
use crate::tissue::Tissue;
use crate::units::depth::Depth;
use crate::units::pressure::Pressure;
use crate::units::rate::Rate;
use crate::util::time_taken;
use itertools::izip;
use std::f64::consts::{E, LN_2};
use time::Duration;

pub mod builder;
pub mod gradient_factor;
pub mod tissue_constants;
pub mod variant;

#[derive(Copy, Clone, Debug)]
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
            .p_n2_mut()
            .iter_mut()
            .zip(self.tissue_constants.n2_hl().iter())
        {
            let pi = segment.end_depth().compensated_pressure(environment) * Pressure(gas.fr_n2());
            *pressure = ZHL16::flat_loading(
                *pressure,
                pi,
                segment.time().whole_minutes() as f64,
                *half_life,
            );
        }

        for (pressure, half_life) in self
            .tissue
            .p_he_mut()
            .iter_mut()
            .zip(self.tissue_constants.he_hl().iter())
        {
            let pi = segment.end_depth().compensated_pressure(environment) * Pressure(gas.fr_n2());
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
            .p_n2_mut()
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
            .p_he_mut()
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

    fn update_first_deco_depth(&mut self, depth: Depth) {
        self.first_deco_depth = self.first_deco_depth.or(Some(depth));
    }

    fn fr_gf_at_depth(&self, depth: Depth) -> f64 {
        self.first_deco_depth
            .map(|deco_depth| {
                if deco_depth > Depth(0) {
                    // Only calculate if below surface
                    self.gf.fr_high()
                        + ((self.gf.fr_high() - self.gf.fr_low()) / (0.0 - deco_depth.0 as f64))
                            * (depth.0 as f64)
                } else {
                    // By definition GFH is on the surface
                    self.gf.fr_high()
                }
            })
            // By definition, when decompression hasn't started, use GFH.
            .unwrap_or_else(|| self.gf.fr_high())
    }

    fn tissue_ab_value(n2_ab: f64, he_ab: f64, p_n2: Pressure, p_he: Pressure) -> f64 {
        (n2_ab * p_n2.0 + he_ab * p_he.0) / (p_n2 + p_he).0
    }

    fn tissue_ceiling(gf: f64, p_n2: Pressure, p_he: Pressure, a: f64, b: f64) -> Pressure {
        Pressure(((p_n2 + p_he).0 - (a * gf)) / (gf / b + 1.0 - gf))
    }

    fn ascent_ceiling(&self, fr_gf_override: Option<f64>) -> Pressure {
        let mut ceilings: [Pressure; TISSUE_COUNT] = [Pressure::default(); TISSUE_COUNT];
        let gf = fr_gf_override.unwrap_or_else(|| self.fr_gf_at_depth(self.diver_depth));

        for (ceil, n2_a, he_a, n2_b, he_b, p_n2, p_he) in izip!(
            &mut ceilings,
            self.tissue_constants.n2_a().iter(),
            self.tissue_constants.he_a().iter(),
            self.tissue_constants.n2_b().iter(),
            self.tissue_constants.he_b().iter(),
            self.tissue.p_n2().iter(),
            self.tissue.p_he().iter(),
        ) {
            let a = ZHL16::tissue_ab_value(*n2_a, *he_a, *p_n2, *p_he);
            let b = ZHL16::tissue_ab_value(*n2_b, *he_b, *p_n2, *p_he);
            *ceil = ZHL16::tissue_ceiling(gf, *p_n2, *p_he, a, b);
        }

        *ceilings
            .iter()
            .min_by(|&&a, &b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    fn find_next_stop(
        &self,
        ascent_rate: Rate,
        descent_rate: Rate,
        gas: &Gas,
        environment: Environment,
    ) -> Segment {
        let next_stop_depth = Depth(
            (3.0 * ((self.ascent_ceiling(None).equivalent_depth(environment).0 as f64 / 3.0)
                .ceil())) as u32,
        );
        let mut stop_time = Duration::zero();

        loop {
            let mut virtual_model = *self;
            // If diver is not at the next stop depth, ascend the diver there.
            if virtual_model.diver_depth != next_stop_depth {
                virtual_model.add_segment(
                    &Segment::new(
                        SegmentType::AscDesc,
                        virtual_model.diver_depth,
                        next_stop_depth,
                        time_taken(ascent_rate, virtual_model.diver_depth, next_stop_depth),
                        ascent_rate,
                        descent_rate,
                    )
                    .unwrap(),
                    gas,
                    environment,
                );
            }

            // Try the current stop time
            let segment = Segment::new(
                SegmentType::DecoStop,
                next_stop_depth,
                next_stop_depth,
                stop_time,
                ascent_rate,
                descent_rate,
            )
            .unwrap();

            virtual_model = virtual_model.add_segment(&segment, gas, environment);
            virtual_model.update_first_deco_depth(next_stop_depth);

            // Break if cleared to proceed to the next stop
            if virtual_model.ascent_ceiling(None)
                < next_stop_depth.pressure(environment) - Depth(3).pressure(environment)
                    + Pressure(1.0)
            {
                break segment;
            } else {
                stop_time += Duration::minute()
            }
        }
    }

    fn find_ndl(&self, gas: &Gas, environment: Environment) -> Option<Duration> {
        let mut ndl_duration = Duration::zero();
        loop {
            let mut virtual_deco = *self;
            let segment = Segment::new(
                SegmentType::Bottom,
                virtual_deco.diver_depth,
                virtual_deco.diver_depth,
                ndl_duration,
                Rate::default(),
                Rate::default(),
            )
            .unwrap();

            virtual_deco = virtual_deco.add_segment(&segment, gas, environment);
            if virtual_deco.ascent_ceiling(Some(self.gf.fr_high())) > Pressure(1.0) {
                if ndl_duration == Duration::zero() {
                    // No NDL
                    break None;
                } else {
                    // Return actual value
                    break Some(ndl_duration);
                }
            } else {
                if ndl_duration > Duration::minutes(999) {
                    // Return oversized value
                    break Some(Duration::minutes(i64::MAX));
                }

                ndl_duration += Duration::minute();
            }
        }
    }
}

impl DecoAlgorithm for ZHL16 {
    fn add_segment(mut self, segment: &Segment, gas: &Gas, environment: Environment) -> Self {
        match segment.segment_type() {
            SegmentType::NoDeco => panic!("no-deco segment applied to deco algorithm"),
            SegmentType::DecoStop => {
                self.add_flat_segment_inner(segment, gas, environment);
                self.update_first_deco_depth(segment.start_depth());
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

    fn get_stops(
        mut self,
        ascent_rate: Rate,
        descent_rate: Rate,
        gas: &Gas,
        environment: Environment,
    ) -> Vec<Segment> {
        let mut stops: Vec<Segment> = vec![];

        if self.ascent_ceiling(Some(self.gf.fr_high())) < Pressure(1.0) {
            stops.push(
                Segment::new(
                    SegmentType::NoDeco,
                    self.diver_depth,
                    self.diver_depth,
                    self.find_ndl(gas, environment)
                        .expect("ascent ceiling < 1.0 but no ndl"),
                    Rate::default(),
                    Rate::default(),
                )
                .unwrap(),
            );
            return stops;
        }

        let mut last_depth = self.diver_depth;
        loop {
            let stop = self.find_next_stop(ascent_rate, descent_rate, gas, environment);
            self.update_first_deco_depth(stop.end_depth());

            // Make sure no AscDesc is made if same depth, but deco necessary
            if last_depth != stop.end_depth() {
                let depth_change_segment = Segment::new(
                    SegmentType::AscDesc,
                    last_depth,
                    stop.end_depth(),
                    time_taken(ascent_rate, stop.end_depth(), last_depth),
                    ascent_rate,
                    descent_rate,
                )
                .unwrap();
                self = self.add_segment(&depth_change_segment, gas, environment);
                stops.push(depth_change_segment);
            }

            self = self.add_segment(&stop, gas, environment);

            self.update_first_deco_depth(stop.end_depth());
            last_depth = stop.end_depth();
            stops.push(stop);

            if self.ascent_ceiling(None) > Pressure(1.0) {
                break stops;
            }
        }
    }

    fn tissue(&self) -> Tissue {
        self.tissue
    }

    fn model_depth(&self) -> Depth {
        self.diver_depth
    }
}
