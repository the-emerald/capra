use crate::common;
use std::f64::consts::{LN_2, E};
use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::Gas;
use crate::deco::deco_algorithm::DecoAlgorithm;
use time::Duration;
use crate::common::time_taken;
use crate::common::dive_segment::SegmentType::DecoStop;
use std::cmp::Ordering;

pub mod util;

const TISSUE_COUNT: usize = 16;

#[derive(Debug, Copy, Clone)]
pub struct ZHL16 {
    p_n2: [f64; TISSUE_COUNT],
    p_he: [f64; TISSUE_COUNT],
    p_t: [f64; TISSUE_COUNT],
    diver_depth: usize,
    n2_a: [f64; TISSUE_COUNT],
    n2_b: [f64; TISSUE_COUNT],
    n2_hl: [f64; TISSUE_COUNT],
    he_a: [f64; TISSUE_COUNT],
    he_b: [f64; TISSUE_COUNT],
    he_hl: [f64; TISSUE_COUNT],

    first_deco_depth: Option<usize>,
    gf_low: f64,
    gf_high: f64,
}

impl ZHL16 {
    pub fn new(tissue_gas: &Gas, n2_a: [f64; TISSUE_COUNT], n2_b: [f64; TISSUE_COUNT],
               n2_hl: [f64; TISSUE_COUNT], he_a: [f64;TISSUE_COUNT], he_b: [f64;TISSUE_COUNT],
               he_hl: [f64;TISSUE_COUNT], gf_low: usize, gf_high: usize) -> Self {

        let adjusted_fr_n2 = tissue_gas.fr_n2() * (1.0 - util::ZHL16_WATER_VAPOUR_PRESSURE);
        let adjusted_fr_he;
        if tissue_gas.fr_he() >= util::ZHL16_WATER_VAPOUR_PRESSURE {
            adjusted_fr_he = tissue_gas.fr_he() * (1.0 - util::ZHL16_WATER_VAPOUR_PRESSURE)
        }
        else {
            adjusted_fr_he = tissue_gas.fr_he() // TODO: Refactor this to be consistent?
        }

        Self {
            p_n2: [adjusted_fr_n2; TISSUE_COUNT],
            p_he: [adjusted_fr_he; TISSUE_COUNT],
            p_t: [adjusted_fr_n2 + adjusted_fr_he; TISSUE_COUNT],
            diver_depth: 0,
            n2_a,
            n2_b,
            n2_hl,
            he_a,
            he_b,
            he_hl,

            first_deco_depth: None,
            gf_low: gf_low as f64/100.0,
            gf_high: gf_high as f64/100.0,
        }
    }

    fn update_first_deco_depth(&mut self, deco_depth: usize) {
        // Update the first deco depth of the diver. This is used to calculate the GF any given point
        // of the decompression schedule.
        match self.first_deco_depth {
            Some(_t) => {}, // If it's already set then don't touch it
            None => self.first_deco_depth = Some(deco_depth) // Otherwise update it
        }
    }

    fn gf_at_depth(&self, depth: usize) -> f64 {
        // Find the gradient factor to use at a given depth during decompression
        match self.first_deco_depth {
            Some(t) => {
                // Only calculate the gradient factor if we're below the surface.
                if depth > 0 {
                    return self.gf_high + ((self.gf_high - self.gf_low) /
                        (0.0 - t as f64)) * (depth as f64)
                }
                self.gf_high // We must be on the surface, by definition use gf_high
            }
            None => self.gf_high // We haven't started decompression yet. use gf_high by definition.
        }
    }

    fn add_depth_change(&mut self, segment: &DiveSegment, gas: &Gas, metres_per_bar: f64) {
        // Add a segment that has a depth change according to the Schreiner Equation.

        let delta_depth = (segment.get_end_depth() as isize) - (segment.get_start_depth() as isize);
        let rate;
        if delta_depth > 0 {
            rate = segment.get_descent_rate()
        }
        else {
            rate = segment.get_ascent_rate()
        }

        let t = time_taken(rate, segment.get_end_depth(), segment.get_start_depth()).whole_seconds() as f64 / 60.0;

        // Load nitrogen tissue compartments
        for (idx, val) in self.p_n2.iter_mut().enumerate() {
            let po = *val;
            let pio: f64 = ZHL16::compensated_pressure(segment.get_end_depth(), metres_per_bar) * gas.fr_n2();
            let r = (rate as f64 / 10.0) * gas.fr_n2();
            let k = LN_2 / self.n2_hl[idx];
            let pn: f64 = ZHL16::depth_change_loading(t, po, pio, r, k);
            *val = pn;
            self.p_t[idx] = pn;
        }

        // Load helium tissue compartments
        for (idx, val) in self.p_he.iter_mut().enumerate() {
            let po = *val;
            let pio: f64 = ZHL16::compensated_pressure(segment.get_end_depth(), metres_per_bar) * gas.fr_he();
            let r = (rate as f64 / 10.0) * gas.fr_he();
            let k = LN_2 / self.he_hl[idx];
            let ph: f64 = ZHL16::depth_change_loading(t, po, pio, r, k);
            *val = ph;
            self.p_t[idx] += ph;
        }
        self.diver_depth = segment.get_end_depth(); // Update diver depth
    }

    fn compensated_pressure(depth: usize, metres_per_bar: f64) -> f64 {
        common::mtr_bar(depth as f64, metres_per_bar) - util::ZHL16_WATER_VAPOUR_PRESSURE
    }

    fn depth_change_loading(time: f64, initial_pressure: f64, initial_ambient_pressure: f64,
                            r: f64, k: f64) -> f64 {
        initial_ambient_pressure + r * (time - (1.0 / k)) -
            (initial_ambient_pressure - initial_pressure - (r / k)) * E.powf(-1.0 * k * time)
    }

    fn add_bottom_segment(&mut self, segment: &DiveSegment, gas: &Gas, metres_per_bar: f64) {
        for (idx, val) in self.p_n2.iter_mut().enumerate() {
            let po = *val;
            let pi = ZHL16::compensated_pressure(segment.get_end_depth(), metres_per_bar) * gas.fr_n2();
            let p = po + (pi - po) *
                (1.0 - (2.0_f64.powf(-1.0*segment.get_time().whole_minutes() as f64 / self.n2_hl[idx])));
            *val = p;
            self.p_t[idx] = p;
        }

        for (idx, val) in self.p_he.iter_mut().enumerate() {
            let po = *val;
            let pi = ZHL16::compensated_pressure(segment.get_end_depth(), metres_per_bar) * gas.fr_he();
            let p = po + (pi - po) *
                (1.0 - (2.0_f64.powf(-1.0*segment.get_time().whole_minutes() as f64 / self.he_hl[idx])));
            *val = p;
            self.p_t[idx] += p;
        }
        self.diver_depth = segment.get_end_depth();
    }

    pub(crate) fn find_ascent_ceiling(&self, gf_override: Option<f64>) -> f64 {
        let mut ceilings: [f64; TISSUE_COUNT] = [0.0; TISSUE_COUNT];
        let gf;
        match gf_override {
            Some(t) => gf = t,
            None => {
                match self.first_deco_depth {
                    Some(_t) => gf = self.gf_at_depth(self.diver_depth),
                    None => gf = self.gf_low
                }
            }
        }

        for (idx, val) in ceilings.iter_mut().enumerate() {
            let a = self.tissue_a_value(idx);
            let b = self.tissue_b_value(idx);
            *val = self.tissue_ceiling(gf, idx, a, b)
        }

        ceilings.iter().cloned().fold(std::f64::NAN, f64::max)
    }

    fn tissue_ceiling(&self, gf: f64, x: usize, a: f64, b: f64) -> f64 {
        ((self.p_n2[x] + self.p_he[x]) - (a * gf)) / (gf / b + 1.0 - gf)
    }

    fn tissue_b_value(&self, x: usize) -> f64 {
        (self.n2_b[x] * self.p_n2[x] + self.he_b[x] * self.p_he[x]) /
            (self.p_n2[x] + self.p_he[x])
    }

    fn tissue_a_value(&self, x: usize) -> f64 {
        (self.n2_a[x] * self.p_n2[x] + self.he_a[x] * self.p_he[x]) /
            (self.p_n2[x] + self.p_he[x])
    }

    pub(crate) fn next_stop(&self, ascent_rate: isize, descent_rate: isize,
                            gas: &Gas, metres_per_bar: f64) -> DiveSegment {
        let stop_depth = (3.0*(
            (common::bar_mtr(self.find_ascent_ceiling(None), metres_per_bar)/3.0)
                .ceil())) as usize; // Find the next stop depth rounded to 3m
        let mut stop_time: usize = 0;
        let mut in_limit: bool = false;
        while !in_limit {
            let mut virtual_zhl16 = *self;
            // This is done for the exact same reason as the check in the surface implementation.
            if virtual_zhl16.diver_depth != stop_depth {
                let depth_change_segment = DiveSegment::new(SegmentType::AscDesc,
                                                            virtual_zhl16.diver_depth,
                                                            stop_depth,
                                                            time_taken(ascent_rate, virtual_zhl16.diver_depth, stop_depth),
                                                            ascent_rate, descent_rate).unwrap();
                virtual_zhl16.add_dive_segment(&depth_change_segment, gas, metres_per_bar);
            }
            let segment = DiveSegment::new(SegmentType::DecoStop,
                                           stop_depth, stop_depth,
                                           Duration::minutes(stop_time as i64), ascent_rate, descent_rate).unwrap();

            virtual_zhl16.add_dive_segment(&segment, gas, metres_per_bar);
            virtual_zhl16.update_first_deco_depth(segment.get_end_depth());

            in_limit = virtual_zhl16.find_ascent_ceiling(None) < common::mtr_bar(stop_depth as f64, metres_per_bar)
                - (common::mtr_bar(3.0, metres_per_bar) - 1.0);
            stop_time += 1;
        }
        DiveSegment::new(SegmentType::DecoStop, stop_depth, stop_depth,
                         Duration::minutes(stop_time as i64), ascent_rate, descent_rate).unwrap()
    }

    pub(crate) fn ndl(&self, gas: &Gas, metres_per_bar: f64) -> Option<DiveSegment> {
        let mut ndl = 0;
        let mut in_ndl= true;
        while in_ndl {
            let mut virtual_zhl16 = *self;
            let virtual_segment = DiveSegment::new(SegmentType::NoDeco,
                                                   virtual_zhl16.diver_depth,
                                                   virtual_zhl16.diver_depth, Duration::minutes(ndl),
                                                   0, 0).unwrap();
            virtual_zhl16.add_bottom_segment(&virtual_segment, gas, metres_per_bar);
            in_ndl = virtual_zhl16.find_ascent_ceiling(Some(self.gf_high)) < 1.0;
            if in_ndl {
                ndl += 1;
            }
            if ndl > 999 {
                return Some(DiveSegment::new(SegmentType::NoDeco,
                                              self.diver_depth, self.diver_depth,
                                              Duration::seconds(std::i64::MAX), 0, 0).unwrap())
            }
        }
        Some(DiveSegment::new(SegmentType::NoDeco, self.diver_depth,
                              self.diver_depth, Duration::minutes(ndl), 0, 0).unwrap())
    }
}

impl DecoAlgorithm for ZHL16 {
    fn add_dive_segment(&mut self, segment: &DiveSegment, gas: &Gas, metres_per_bar: f64) {
        match segment.get_segment_type() {
            SegmentType::AscDesc => {
                self.add_depth_change(segment ,gas, metres_per_bar);
            }
            SegmentType::DecoStop => {
                self.add_bottom_segment(segment, gas, metres_per_bar);
                self.update_first_deco_depth(segment.get_start_depth());
            }
            _ => {
                self.add_bottom_segment(segment, gas, metres_per_bar);
            }
        }
    }

    // fn add_segment(&mut self, segment: &common::dive_segment::DiveSegment,
    //                gas: &Gas, metres_per_bar: f64) -> Vec<DiveSegment> {
    //
    //     // There may be intermediate stops between the current state and the diver!
    //     let intermediate_stops = self.get_stops(
    //         segment.get_ascent_rate(), segment.get_descent_rate(), gas, metres_per_bar);
    //
    //     let mut used_segs:Vec<DiveSegment> = Vec::new(); // Keep track of the stops we used
    //
    //     // If there are deco stops that are deeper than the segment:
    //     if intermediate_stops.iter().any(|x| x.get_segment_type() == SegmentType::DecoStop && x.get_start_depth() > segment.get_start_depth()) {
    //         // Apply them one by one until ready to begin next segment
    //         for stop in intermediate_stops.iter().take_while(|x| x.get_start_depth() > segment.get_start_depth()) {
    //             self.add_dive_segment(stop, gas, metres_per_bar);
    //             self.update_first_deco_depth(stop.get_end_depth());
    //             used_segs.push(*stop);
    //         }
    //     }
    //     else {
    //         // Otherwise make an depth change to the next bottom segment
    //         // If we're not already there, obviously!
    //         if segment.get_start_depth() != self.diver_depth {
    //             let ascent_rate = match self.diver_depth.cmp(&segment.get_start_depth()) {
    //                 Ordering::Less => segment.get_ascent_rate(),
    //                 _ => segment.get_descent_rate()
    //             };
    //
    //             let asc_desc_to_segment = DiveSegment::new(
    //                 SegmentType::AscDesc,
    //                 self.diver_depth,
    //                 segment.get_start_depth(),
    //                 time_taken(ascent_rate, self.diver_depth, segment.get_start_depth()),
    //                 segment.get_ascent_rate(), segment.get_descent_rate()
    //             ).unwrap();
    //             self.add_dive_segment(&asc_desc_to_segment, gas, metres_per_bar);
    //         }
    //     }
    //
    //     // Apply the actual segment we wanted
    //     self.add_dive_segment(segment, gas, metres_per_bar);
    //     used_segs
    // }

    fn surface(&mut self, ascent_rate: isize, descent_rate: isize, gas: &Gas, metres_per_bar: f64) -> Vec<DiveSegment> {
        let mut stops: Vec<DiveSegment> = Vec::new();
        // If the ascent ceiling with a GFH override is less than 1.0 then we're in NDL times
        if self.find_ascent_ceiling(Some(self.gf_high)) < 1.0 {
            match self.ndl(gas, metres_per_bar) {
                Some(t) => {
                    stops.push(t)
                },
                None => panic!("ascent ceiling is < 1.0 but NDL was found"),
            };
            return stops;
        }

        let mut last_depth = self.diver_depth;
        while self.find_ascent_ceiling(None) > 1.0 {
            // Find the next stop and apply it.
            let stop = self.next_stop(ascent_rate, descent_rate, gas, metres_per_bar);
            self.update_first_deco_depth(stop.get_end_depth());
            self.add_dive_segment(&stop, gas, metres_per_bar);

            // This is done because of the nature of segment processing!
            // If a diver has just ascended from 18m to 15m, for example, their depth would be
            // at 15m, yet the next stop will be 15m. In that case, do not generate an AscDesc
            // segment.
            if last_depth != stop.get_end_depth() {
                let depth_change_segment = DiveSegment::new(SegmentType::AscDesc,
                                                            last_depth, stop.get_end_depth(),
                                                            time_taken(
                                                                ascent_rate,
                                                                stop.get_end_depth(),
                                                                last_depth
                                                            ), ascent_rate, descent_rate).unwrap();
                self.add_dive_segment(&depth_change_segment, gas, metres_per_bar);
                stops.push(depth_change_segment);
            }

            last_depth = stop.get_end_depth();
            stops.push(stop);
        }
        stops
    }
}