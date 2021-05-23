use std::f64::consts::{E, LN_2};

use time::Duration;

use crate::common;
use crate::common::depth::Depth;
use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::Gas;
use crate::common::pressure::Pressure;
use crate::common::time_taken;
use crate::common::water_density::WaterDensity;
use crate::deco::{TISSUE_COUNT, WATER_VAPOUR_PRESSURE};
use crate::deco::deco_algorithm::DecoAlgorithm;
use crate::deco::tissue::Tissue;
use crate::deco::zhl16::gradient_factor::GradientFactor;
use crate::deco::zhl16::tissue_constants::TissueConstants;
use crate::tissue::Tissue;

pub mod tissue_constants;
pub mod util;
pub mod variant;
pub mod builder;
pub mod gradient_factor;

/// A ZHL-16 decompression model of a diver.
/// # Notes
/// For now, each ZHL16 struct should only be used for one dive. This is because calculating decompression
/// stops with Gradient Factors requires some side effects to be stored inside the struct.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
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
    gf: GradientFactor
}

impl ZHL16 {

    /// Update the first deco depth of the diver. This is used to calculate the GF any given point
    /// of the decompression schedule.
    fn update_first_deco_depth(&mut self, deco_depth: Depth) {
        match self.first_deco_depth {
            Some(_t) => {} // If it's already set then don't touch it
            None => self.first_deco_depth = Some(deco_depth), // Otherwise update it
        }
    }

    /// Find the gradient factor to use at a given depth during decompression
    fn gf_at_depth(&self, depth: Depth) -> f64 {
        match self.first_deco_depth {
            Some(t) => {
                // Only calculate the gradient factor if we're below the surface.
                if depth > 0 {
                    return self.gf_high
                        + ((self.gf_high - self.gf_low) / (0.0 - t as f64)) * (depth as f64);
                }
                self.gf_high // We must be on the surface, by definition use gf_high
            }
            None => self.gf_high, // We haven't started decompression yet. use gf_high by definition.
        }
    }

    /// Add a segment that has a depth change according to the Schreiner Equation.
    fn add_depth_changing_segment(&mut self, segment: &DiveSegment, gas: &Gas, density: WaterDensity) {
        let delta_depth = (segment.end_depth() as isize) - (segment.start_depth() as isize);
        let rate;
        if delta_depth > 0 {
            rate = segment.descent_rate()
        } else {
            rate = segment.ascent_rate()
        }

        let t = segment.time().whole_seconds() as f64 / 60.0;

        // Load nitrogen tissue compartments
        for (idx, val) in self.tissue.p_n2.iter_mut().enumerate() {
            let po = *val;
            let pio =
                Pressure(
                    ZHL16::compensated_pressure(segment.start_depth(), density).0 * gas.fr_n2()
                );
            let r = (rate as f64 / 10.0) * gas.fr_n2();
            let k = LN_2 / self.tissue_constants.n2_hl[idx];
            let pn = ZHL16::depth_change_loading(t, po, pio, r, k);
            *val = pn;
            self.tissue.p_t[idx] = pn;
        }

        // Load helium tissue compartments
        for (idx, val) in self.tissue.p_he.iter_mut().enumerate() {
            let po = *val;
            let pio =
                Pressure(
                    ZHL16::compensated_pressure(segment.start_depth(), density).0 * gas.fr_he()
            );
            let r = (rate as f64 / 10.0) * gas.fr_he();
            let k = LN_2 / self.tissue_constants.he_hl[idx];
            let ph = ZHL16::depth_change_loading(t, po, pio, r, k);
            *val = ph;
            self.tissue.p_t[idx] += ph;
        }
        self.diver_depth = segment.end_depth(); // Update diver depth
    }

    /// Calculate the pressure at a given depth minus the ambient water vapour pressure in the lungs.
    fn compensated_pressure(depth: Depth, density: WaterDensity) -> Pressure {
        depth.pressure(&density) - WATER_VAPOUR_PRESSURE
    }

    /// Calculate the gas loading with a depth change.
    fn depth_change_loading(
        time: f64,
        initial_pressure: Pressure,
        initial_ambient_pressure: Pressure,
        r: f64,
        k: f64,
    ) -> Pressure {
        Pressure(
            initial_ambient_pressure.0 + r * (time - (1.0 / k))
            - ((initial_ambient_pressure.0 - initial_pressure.0 - (r / k)) * E.powf(-1.0 * k * time))
        )
    }

    /// Add a segment without depth change according to the Schreiner Equation.
    fn add_bottom_segment(&mut self, segment: &DiveSegment, gas: &Gas, density: WaterDensity) {
        for (idx, val) in self.tissue.p_n2.iter_mut().enumerate() {
            let po = *val;
            let pi = Pressure(
                ZHL16::compensated_pressure(segment.end_depth(), density).0 * gas.fr_n2()
            );
            let p = po
                + (pi - po)
                    * Pressure(1.0
                        - (2.0_f64.powf(
                            -1.0 * segment.time().whole_minutes() as f64
                                / self.tissue_constants.n2_hl[idx],
                        )));
            *val = p;
            self.tissue.p_t[idx] = p;
        }

        for (idx, val) in self.tissue.p_he.iter_mut().enumerate() {
            let po = *val;
            let pi = Pressure(
                ZHL16::compensated_pressure(segment.end_depth(), density).0 * gas.fr_he()
            );
            let p = po
                + (pi - po)
                    * Pressure(1.0
                        - (2.0_f64.powf(
                            -1.0 * segment.time().whole_minutes() as f64
                                / self.tissue_constants.he_hl[idx],
                        )));
            *val = p;
            self.tissue.p_t[idx] += p;
        }
        self.diver_depth = segment.end_depth();
    }

    /// Returns the ascent ceiling of the model.
    pub(crate) fn find_ascent_ceiling(&self, gf_override: Option<f64>) -> Pressure {
        let mut ceilings: [Pressure; TISSUE_COUNT] = [Pressure::default(); TISSUE_COUNT];
        let gf = match gf_override {
            Some(t) => t,
            None => match self.first_deco_depth {
                Some(_) => self.gf_at_depth(self.diver_depth),
                None => self.gf_low,
            },
        };

        for (idx, val) in ceilings.iter_mut().enumerate() {
            let a = self.tissue_a_value(idx);
            let b = self.tissue_b_value(idx);
            *val = self.tissue_ceiling(gf, idx, a, b)
        }

        ceilings.iter().cloned()
            .fold(Pressure(f64::NAN), |a, x| Pressure(
                f64::max(a.0, x.0))
            )
    }

    /// Calculate the tissue ceiling of a compartment.
    fn tissue_ceiling(&self, gf: f64, nth: usize, a: f64, b: f64) -> Pressure {
        Pressure(
            ((self.tissue.p_n2[nth] + self.tissue.p_he[nth]).0 - (a * gf)) / (gf / b + 1.0 - gf)
        )
    }

    /// Calculate the B-value of a compartment.
    fn tissue_b_value(&self, x: usize) -> f64 {
        (self.tissue_constants.n2_b[x] * self.tissue.p_n2[x]
            + self.tissue_constants.he_b[x] * self.tissue.p_he[x])
            / (self.tissue.p_n2[x] + self.tissue.p_he[x])
    }

    /// Calculate the A-value of a compartment.
    fn tissue_a_value(&self, x: usize) -> f64 {
        (self.tissue_constants.n2_a[x] * self.tissue.p_n2[x]
            + self.tissue_constants.he_a[x] * self.tissue.p_he[x])
            / (self.tissue.p_n2[x] + self.tissue.p_he[x])
    }

    /// Return the next deco stop of the model.
    pub(crate) fn next_stop(
        &self,
        ascent_rate: i32,
        descent_rate: i32,
        gas: &Gas,
        density: WaterDensity,
    ) -> DiveSegment {
        let stop_depth = Depth(
            (3.0
            * ((common::bar_mtr(self.find_ascent_ceiling(None), density) / 3.0).ceil()))
        ); // Find the next stop depth rounded to 3m
        let mut stop_time: usize = 0;
        let mut in_limit: bool = false;
        while !in_limit {
            let mut virtual_zhl16 = *self;
            // This is done for the exact same reason as the check in the surface implementation.
            if virtual_zhl16.diver_depth != stop_depth {
                let depth_change_segment = DiveSegment::new(
                    SegmentType::AscDesc,
                    virtual_zhl16.diver_depth,
                    stop_depth,
                    time_taken(ascent_rate, virtual_zhl16.diver_depth, stop_depth),
                    ascent_rate,
                    descent_rate,
                )
                .unwrap();
                virtual_zhl16.add_segment(&depth_change_segment, gas, density);
            }
            let segment = DiveSegment::new(
                SegmentType::DecoStop,
                stop_depth,
                stop_depth,
                Duration::minutes(stop_time as i64),
                ascent_rate,
                descent_rate,
            )
            .unwrap();

            virtual_zhl16.add_segment(&segment, gas, density);
            virtual_zhl16.update_first_deco_depth(segment.end_depth());

            in_limit = virtual_zhl16.find_ascent_ceiling(None)
                < common::mtr_bar(stop_depth as f64, density)
                    - (common::mtr_bar(3.0, density) - 1.0);

            if !in_limit {
                stop_time += 1;
            }
        }
        DiveSegment::new(
            SegmentType::DecoStop,
            stop_depth,
            stop_depth,
            Duration::minutes(stop_time as i64),
            ascent_rate,
            descent_rate,
        )
        .unwrap()
    }

    /// Return the no-decompression limit of the model, if it exists.
    pub(crate) fn ndl(&self, gas: &Gas, density: WaterDensity) -> Option<DiveSegment> {
        let mut ndl = 0;
        let mut in_ndl = true;
        while in_ndl {
            let mut virtual_zhl16 = *self;
            let virtual_segment = DiveSegment::new(
                SegmentType::NoDeco,
                virtual_zhl16.diver_depth,
                virtual_zhl16.diver_depth,
                Duration::minutes(ndl),
                0,
                0,
            )
            .unwrap();
            virtual_zhl16.add_bottom_segment(&virtual_segment, gas, density);
            in_ndl = virtual_zhl16.find_ascent_ceiling(Some(self.gf_high)) < 1.0;
            if in_ndl {
                ndl += 1;
            }
            if ndl > 999 {
                return Some(
                    DiveSegment::new(
                        SegmentType::NoDeco,
                        self.diver_depth,
                        self.diver_depth,
                        Duration::seconds(std::i64::MAX),
                        0,
                        0,
                    )
                    .unwrap(),
                );
            }
        }
        Some(
            DiveSegment::new(
                SegmentType::NoDeco,
                self.diver_depth,
                self.diver_depth,
                Duration::minutes(ndl),
                0,
                0,
            )
            .unwrap(),
        )
    }

    /// Returns the tissue of the deco model.
    pub fn tissue(&self) -> Tissue {
        self.tissue
    }

    fn add_segment(&mut self, segment: &DiveSegment, gas: &Gas, density: WaterDensity) {
        match segment.segment_type() {
            SegmentType::AscDesc => self.add_depth_changing_segment(segment, gas, density),
            SegmentType::DecoStop => {
                self.add_bottom_segment(segment, gas, density);
                self.update_first_deco_depth(segment.start_depth());
            }
            _ => {
                self.add_bottom_segment(segment, gas, density);
            }
        }
    }
}

impl DecoAlgorithm for ZHL16 {
    fn add_dive_segment(&mut self, segment: &DiveSegment, gas: &Gas, metres_per_bar: f64) {
        self.add_segment(segment, gas, metres_per_bar);
    }

    fn surface(
        &mut self,
        ascent_rate: i32,
        descent_rate: i32,
        gas: &Gas,
        density: WaterDensity,
    ) -> Vec<DiveSegment> {
        let mut stops: Vec<DiveSegment> = Vec::new();
        // If the ascent ceiling with a GFH override is less than 1.0 then we're in NDL times
        if self.find_ascent_ceiling(Some(self.gf_high)) < 1.0 {
            match self.ndl(gas, density) {
                Some(t) => stops.push(t),
                None => panic!("ascent ceiling is < 1.0 but NDL was found"),
            };
            return stops;
        }

        let mut last_depth = self.diver_depth;
        while self.find_ascent_ceiling(None) > 1.0 {
            // Find the next stop and apply it.
            let stop = self.next_stop(ascent_rate, descent_rate, gas, density);
            self.update_first_deco_depth(stop.end_depth());

            // This is done because of the nature of segment processing!
            // If a diver has just ascended from 18m to 15m, for example, their depth would be
            // at 15m, yet the next stop will be 15m. In that case, do not generate an AscDesc
            // segment.
            if last_depth != stop.end_depth() {
                let depth_change_segment = DiveSegment::new(
                    SegmentType::AscDesc,
                    last_depth,
                    stop.end_depth(),
                    time_taken(ascent_rate, stop.end_depth(), last_depth),
                    ascent_rate,
                    descent_rate,
                )
                .unwrap();
                self.add_dive_segment(&depth_change_segment, gas, density);
                stops.push(depth_change_segment);
            }

            self.add_dive_segment(&stop, gas, density);
            self.update_first_deco_depth(stop.end_depth());

            last_depth = stop.end_depth();

            stops.push(stop);
        }
        stops
    }

    fn get_tissue(&self) -> Tissue {
        self.tissue
    }
}
