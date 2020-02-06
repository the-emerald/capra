use crate::common;
use std::f32::consts::{LN_2, E};
use crate::common::dive_segment::{DiveSegment, SegmentType};

pub mod util;

#[derive(Debug, Copy, Clone)]
pub struct ZHL16 {
    p_n2: [f32; 16],
    p_he: [f32; 16],
    p_t: [f32; 16],
    diver_depth: usize,
    n2_a: [f32; 16],
    n2_b: [f32; 16],
    n2_hl: [f32; 16],
    he_a: [f32; 16],
    he_b: [f32; 16],
    he_hl: [f32; 16],

    first_deco_depth: Option<usize>,
    gf_low: f32,
    gf_high: f32,
}

impl ZHL16 {
    pub fn new(tissue_gas: &common::gas::Gas, n2_a: [f32; 16], n2_b: [f32; 16], n2_hl: [f32; 16],
    he_a: [f32;16], he_b: [f32;16], he_hl: [f32;16], gf_low: usize, gf_high: usize) -> Self {
        let adjusted_fr_n2 = tissue_gas.fr_n2() * (1.0 - util::ZHL16_WATER_VAPOUR_PRESSURE);
        let adjusted_fr_he;
        if tissue_gas.fr_he() >= util::ZHL16_WATER_VAPOUR_PRESSURE {
            adjusted_fr_he = tissue_gas.fr_he() * (1.0 - util::ZHL16_WATER_VAPOUR_PRESSURE)
        }
        else {
            adjusted_fr_he = tissue_gas.fr_he()
        }

        Self {
            p_n2: [adjusted_fr_n2; 16],
            p_he: [adjusted_fr_he; 16],
            p_t: [adjusted_fr_n2 + adjusted_fr_he; 16],
            diver_depth: 0,
            n2_a,
            n2_b,
            n2_hl,
            he_a,
            he_b,
            he_hl,

            first_deco_depth: None,
            gf_low: gf_low as f32/100.0,
            gf_high: gf_high as f32/100.0,
        }
    }

    fn update_first_deco_depth(&mut self, deco_depth: usize) {
        match self.first_deco_depth {
            Some(_t) => return,
            None => self.first_deco_depth = Some(deco_depth)
        }
    }

    fn gf_at_depth(&self, depth: usize) -> f32 {
        match self.first_deco_depth {
            Some(t) => {
                if depth > 0 {
                    return self.gf_high + ((self.gf_high - self.gf_low) /
                        (0.0 - t as f32)) * (depth as f32)
                }
                self.gf_high // We must be on the surface, by definition use gf_high
            }
            None => self.gf_high
        }
    }

    pub(crate) fn add_depth_change(&mut self, segment: &DiveSegment, gas: &common::gas::Gas) {
        let delta_depth = (segment.get_depth() as isize) - (self.diver_depth as isize);
        let rate;
        if delta_depth > 0 {
            rate = segment.get_descent_rate()
        }
        else {
            rate = segment.get_ascent_rate()
        }

        let t: f32 = delta_depth as f32 / rate as f32;
        for x in 0..16 {
            let po = self.p_n2[x];
            let pio: f32 = ZHL16::compensated_pressure(segment) * gas.fr_n2();
            let r = (rate as f32 / 10.0) * gas.fr_n2();
            let k = LN_2 / self.n2_hl[x];
            let pn: f32 = ZHL16::depth_change_loading(t, po, pio, r, k);
            self.p_n2[x] = pn;
            self.p_t[x] = pn;
        }
        for x in 0..16 {
            let po = self.p_he[x];
            let pio: f32 = ZHL16::compensated_pressure(segment) * gas.fr_he();
            let r = (rate as f32 / 10.0) * gas.fr_he();
            let k = LN_2 / self.he_hl[x];
            let ph: f32 = ZHL16::depth_change_loading(t, po, pio, r, k);
            self.p_he[x] = ph;
            self.p_t[x] += ph;
        }
        self.diver_depth = segment.get_depth();
        //self.update_max_depth(segment.get_depth());
    }

    fn compensated_pressure(segment: &DiveSegment) -> f32 {
        (common::mtr_bar(segment.get_depth() as f32) -
            util::ZHL16_WATER_VAPOUR_PRESSURE)
    }

    fn depth_change_loading(time: f32, initial_pressure: f32, initial_ambient_pressure: f32,
                            r: f32, k: f32) -> f32 {
        initial_ambient_pressure + r * (time - (1.0 / k)) -
            (initial_ambient_pressure - initial_pressure - (r / k)) * E.powf(-1.0 * k * time)
    }

    pub(crate) fn add_bottom(&mut self, segment: &DiveSegment, gas: &common::gas::Gas) {
        for x in 0..16 {
            let po = self.p_n2[x];
            let pi = ZHL16::compensated_pressure(segment) * gas.fr_n2();
            let p = po + (pi - po) *
                (1.0 - (2.0_f32.powf(-1.0*segment.get_time() as f32 / self.n2_hl[x])));
            self.p_n2[x] = p;
            self.p_t[x] = p;
        }
        for x in 0..16 {
            let po = self.p_he[x];
            let pi = ZHL16::compensated_pressure(segment) * gas.fr_he();
            let p = po + (pi - po) *
                (1.0 - (2.0_f32.powf(-1.0*segment.get_time() as f32 / self.he_hl[x])));
            self.p_he[x] = p;
            self.p_t[x] += p;
        }
        self.diver_depth = segment.get_depth();
        //self.update_max_depth(segment.get_depth());
    }

    pub(crate) fn find_ascent_ceiling(&self, gf_override: Option<f32>) -> f32 {
        let mut ceilings: [f32; 16] = [0.0; 16];
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

        for x in 0..16 {
            let a = self.tissue_a_value(x);
            let b = self.tissue_b_value(x);
            ceilings[x] = self.tissue_ceiling(gf, x, a, b);
        }
        ceilings.iter().cloned().fold(0./0., f32::max)
    }

    fn tissue_ceiling(&self, gf: f32, x: usize, a: f32, b: f32) -> f32 {
        ((self.p_n2[x] + self.p_he[x]) - (a * gf)) / (gf / b + 1.0 - gf)
    }

    fn tissue_b_value(&self, x: usize) -> f32 {
        (self.n2_b[x] * self.p_n2[x] + self.he_b[x] * self.p_he[x]) /
            (self.p_n2[x] + self.p_he[x])
    }

    fn tissue_a_value(&self, x: usize) -> f32 {
        (self.n2_a[x] * self.p_n2[x] + self.he_a[x] * self.p_he[x]) /
            (self.p_n2[x] + self.p_he[x])
    }

    pub(crate) fn next_stop(&self, ascent_rate: isize, descent_rate: isize,
                            gas: &common::gas::Gas) -> DiveSegment {
        let stop_depth = (3.0*(
            (common::bar_mtr(self.find_ascent_ceiling(None))/3.0)
                .ceil())) as usize;
        let mut stop_time: usize = 0;
        let mut in_limit: bool = false;
        while !in_limit {
            let mut virtual_zhl16 = self.clone();
            let segment = DiveSegment::new(SegmentType::DecoStop,
                                           stop_depth, stop_time, ascent_rate, descent_rate);
            virtual_zhl16.add_depth_change(&segment, gas);
            virtual_zhl16.add_bottom(&segment, gas);
            virtual_zhl16.update_first_deco_depth(segment.get_depth());
            in_limit = virtual_zhl16.find_ascent_ceiling(None) < common::mtr_bar(stop_depth as f32)
                - 0.3;
            stop_time += 1;
        }
        DiveSegment::new(SegmentType::DecoStop, stop_depth, stop_time,
                         ascent_rate, descent_rate)
    }

    pub(crate) fn ndl(&self, gas: &common::gas::Gas) -> Option<usize> {
        let mut ndl = 0;
        let mut in_ndl= true;
        while in_ndl {
            let mut virtual_zhl16 = self.clone();
            let virtual_segment = DiveSegment::new(SegmentType::NoDeco,
                                                   virtual_zhl16.diver_depth, ndl, 0,
                                                   0);
            virtual_zhl16.add_bottom(&virtual_segment, gas);
            in_ndl = virtual_zhl16.find_ascent_ceiling(Some(self.gf_high)) < 1.0;
            if in_ndl {
                ndl += 1;
            }
            if ndl > 999 {
                return Some(std::usize::MAX)
            }
        }
        Some(ndl)
    }
}

impl common::deco_algorithm::DecoAlgorithm for ZHL16 {
    fn add_bottom_time(&mut self, segment: &common::dive_segment::DiveSegment,
                       gas: &common::gas::Gas) -> Option<Vec<DiveSegment>> {
        let intermediate_stops = self.get_stops(
            segment.get_ascent_rate(), segment.get_descent_rate(), gas);
        let mut used_stops:Vec<DiveSegment> = Vec::new();

        if intermediate_stops[0].get_segment_type() == SegmentType::DecoStop {
            for stop in intermediate_stops.iter() {
                if stop.get_depth() > segment.get_depth() { // Deco stop is below desired depth
                    self.add_depth_change(stop, gas);
                    self.add_bottom(stop, gas);
                    self.update_first_deco_depth(stop.get_depth());
                    used_stops.push((*stop).clone());
                }
            }
        }
        self.add_depth_change(&segment, gas);
        self.add_bottom(&segment, gas);
        if used_stops.is_empty() {
            None
        }
        else {
            Some(used_stops)
        }
    }

    fn get_stops(&self, ascent_rate: isize, descent_rate: isize, gas: &common::gas::Gas)
        -> Vec<DiveSegment> {
        let mut stops: Vec<DiveSegment> = Vec::new();
        let mut virtual_zhl16 = self.clone();

        if virtual_zhl16.find_ascent_ceiling(Some(self.gf_high)) < 1.0 {
            let _ndl = match virtual_zhl16.ndl(gas) {
                Some(t) => {
                    stops.push(DiveSegment::new(SegmentType::NoDeco, 0,
                                                t, 0, 0))
                },
                None =>  {
                    panic!("Ascent ceiling is < 1.0 but NDL was found.")
                }
            };
            return stops;
        }

        while virtual_zhl16.find_ascent_ceiling(None) > 1.0 {
            let stop = virtual_zhl16.next_stop(ascent_rate, descent_rate, gas);
            virtual_zhl16.update_first_deco_depth(stop.get_depth());
            // Do the deco stop
            virtual_zhl16.add_depth_change(&stop, gas);
            virtual_zhl16.add_bottom(&stop, gas);
            stops.push(stop);
        }
        stops
    }
}