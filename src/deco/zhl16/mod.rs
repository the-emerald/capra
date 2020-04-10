use crate::common;
use std::f64::consts::{LN_2, E};
use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::Gas;
use crate::deco::deco_algorithm::DecoAlgorithm;
use time::Duration;
use crate::common::time_taken;

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
            adjusted_fr_he = tissue_gas.fr_he()
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
        match self.first_deco_depth {
            Some(_t) => {},
            None => self.first_deco_depth = Some(deco_depth)
        }
    }

    fn gf_at_depth(&self, depth: usize) -> f64 {
        match self.first_deco_depth {
            Some(t) => {
                if depth > 0 {
                    return self.gf_high + ((self.gf_high - self.gf_low) /
                        (0.0 - t as f64)) * (depth as f64)
                }
                self.gf_high // We must be on the surface, by definition use gf_high
            }
            None => self.gf_high
        }
    }

    fn add_depth_change(&mut self, segment: &DiveSegment, gas: &Gas, metres_per_bar: f64) {
        let delta_depth = (segment.get_end_depth() as isize) - (segment.get_start_depth() as isize);
        let rate;
        if delta_depth > 0 {
            rate = segment.get_descent_rate()
        }
        else {
            rate = segment.get_ascent_rate()
        }

        // let t: f64 = delta_depth as f64 / rate as f64;
        let t = time_taken(rate, segment.get_end_depth(), segment.get_start_depth()).whole_seconds() as f64 / 60.0;
        for (idx, val) in self.p_n2.iter_mut().enumerate() {
            let po = *val;
            let pio: f64 = ZHL16::compensated_pressure(segment.get_end_depth(), metres_per_bar) * gas.fr_n2();
            let r = (rate as f64 / 10.0) * gas.fr_n2();
            let k = LN_2 / self.n2_hl[idx];
            let pn: f64 = ZHL16::depth_change_loading(t, po, pio, r, k);
            *val = pn;
            self.p_t[idx] = pn;
        }

        for (idx, val) in self.p_he.iter_mut().enumerate() {
            let po = *val;
            let pio: f64 = ZHL16::compensated_pressure(segment.get_end_depth(), metres_per_bar) * gas.fr_he();
            let r = (rate as f64 / 10.0) * gas.fr_he();
            let k = LN_2 / self.he_hl[idx];
            let ph: f64 = ZHL16::depth_change_loading(t, po, pio, r, k);
            *val = ph;
            self.p_t[idx] += ph;
        }
        self.diver_depth = segment.get_end_depth();
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
                .ceil())) as usize;
        let mut stop_time: usize = 0;
        let mut in_limit: bool = false;
        while !in_limit {
            let mut virtual_zhl16 = *self;
            dbg!(self.diver_depth);
            let depth_change_segment = DiveSegment::new(SegmentType::AscDesc,
                                                        virtual_zhl16.diver_depth,
                                                        stop_depth,
                                                        time_taken(ascent_rate, virtual_zhl16.diver_depth, stop_depth),
                                                        ascent_rate, descent_rate).unwrap();

            let segment = DiveSegment::new(SegmentType::DecoStop,
                                           stop_depth, stop_depth,
                                           Duration::minutes(stop_time as i64), ascent_rate, descent_rate).unwrap();
            dbg!(&depth_change_segment);
            dbg!(&segment);
            println!("----\n");
            virtual_zhl16.apply_segment(&depth_change_segment, gas, metres_per_bar);
            virtual_zhl16.apply_segment(&segment, gas, metres_per_bar);
            virtual_zhl16.update_first_deco_depth(segment.get_end_depth());
            dbg!(virtual_zhl16.find_ascent_ceiling(None));

            dbg!(common::mtr_bar(stop_depth as f64, metres_per_bar));
            dbg!((common::mtr_bar(3.0, metres_per_bar) - 1.0));

            dbg!(0.3);

            in_limit = virtual_zhl16.find_ascent_ceiling(None) < common::mtr_bar(stop_depth as f64, metres_per_bar)
                - (common::mtr_bar(3.0, metres_per_bar) - 1.0);
            dbg!(in_limit);
            stop_time += 1;
        }
        println!("Ok we're done here");
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

    pub(crate) fn apply_segment(&mut self, segment: &DiveSegment, gas: &Gas, metres_per_bar: f64) {
        match segment.get_segment_type() {
            SegmentType::AscDesc => {
                self.add_depth_change(segment, gas, metres_per_bar);
                // self.diver_depth = segment.get_end_depth()
            }
            _ => {
                self.add_bottom_segment(segment, gas, metres_per_bar);
            }
        }
    }
}

impl DecoAlgorithm for ZHL16 {
    fn add_dive_segment(&mut self, segment: &common::dive_segment::DiveSegment,
                        gas: &Gas, metres_per_bar: f64) -> Option<Vec<DiveSegment>> {
        let intermediate_stops = self.get_stops(
            segment.get_ascent_rate(), segment.get_descent_rate(), gas, metres_per_bar);
        let mut used_stops:Vec<DiveSegment> = Vec::new();

        if intermediate_stops.iter().any(|x| x.get_segment_type() == SegmentType::DecoStop) {
            for stop in intermediate_stops.iter() {
                if stop.get_end_depth() > segment.get_end_depth() { // Deco stop is below desired depth
                    self.apply_segment(stop, gas, metres_per_bar);
                    self.update_first_deco_depth(stop.get_end_depth());
                    used_stops.push((*stop).clone());
                }
            }
        }
        self.apply_segment(segment, gas, metres_per_bar);

        if used_stops.is_empty() {
            None
        }
        else {
            Some(used_stops)
        }
    }

    fn surface(&mut self, ascent_rate: isize, descent_rate: isize, gas: &Gas, metres_per_bar: f64) -> Vec<DiveSegment> {
        let mut stops: Vec<DiveSegment> = Vec::new();

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
            dbg!(self.find_ascent_ceiling(None));
            println!("Next stop:");
            let stop = self.next_stop(ascent_rate, descent_rate, gas, metres_per_bar);
            self.update_first_deco_depth(stop.get_end_depth());
            // Do the deco stop
            self.apply_segment(&stop, gas, metres_per_bar);

            dbg!(last_depth);
            dbg!(stop.get_end_depth());
            let depth_change_segment = DiveSegment::new(SegmentType::AscDesc,
                                                   last_depth, stop.get_end_depth(),
                                                        time_taken(
                                                            ascent_rate,
                                                            stop.get_end_depth(),
                                                            last_depth
                                                        ), ascent_rate, descent_rate).unwrap();
            self.apply_segment(&depth_change_segment, gas, metres_per_bar);
            stops.push(depth_change_segment);


            last_depth = stop.get_end_depth();
            self.diver_depth = last_depth;
            stops.push(stop);
        }
        stops
    }
}