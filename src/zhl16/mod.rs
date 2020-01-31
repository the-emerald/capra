use crate::common;
use std::f32::consts::{LN_2, E};
use crate::common::dive_segment::{DiveSegment, SegmentType};

mod util;

#[derive(Debug, Copy, Clone)]
pub struct ZHL16 {
    p_n2: [f32; 16],
    p_he: [f32; 16],
    p_t: [f32; 16],
    diver_depth: usize,
}

impl ZHL16 {
    pub fn new(tissue_gas: &common::gas::Gas) -> ZHL16 {
        ZHL16 {
            p_n2: [tissue_gas.fr_n2(); 16],
            p_he: [tissue_gas.fr_he(); 16],
            p_t: [tissue_gas.fr_n2() + tissue_gas.fr_he(); 16],
            diver_depth: 0,
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
            let pio: f32 = common::mtr_bar(segment.get_depth() as f32) * gas.fr_n2();
            let r = (rate as f32 / 10.0) * gas.fr_n2();
            let k = LN_2 / util::ZHL16_N2_HALFLIFE[x];
            //println!("N2 tissue {}:: po: {}, pio: {}, r: {}, k: {}", x+1, po, pio, r, k);
            let pn: f32 = pio + r * (t-(1.0/k)) - (pio - po - (r/k)) * E.powf(-1.0*k*t);
            self.p_n2[x] = pn;
            self.p_t[x] = pn;
        }
        for x in 0..16 {
            let po = self.p_he[x];
            let pio: f32 = common::mtr_bar(segment.get_depth() as f32) * gas.fr_he();
            let r = (rate as f32 / 10.0) * gas.fr_he();
            let k = LN_2 / util::ZHL16_HE_HALFLIFE[x];
            //println!("He tissue {}:: po: {}, pio: {}, r: {}, k: {}", x+1, po, pio, r, k);
            let ph: f32 = pio + r * (t-(1.0/k)) - (pio - po - (r/k)) * E.powf(-1.0*k*t);
            self.p_he[x] = ph;
            self.p_t[x] += ph;
        }
        self.diver_depth = segment.get_depth();
    }

    pub(crate) fn add_bottom(&mut self, segment: &DiveSegment, gas: &common::gas::Gas) {
        for x in 0..16 {
            let po = self.p_n2[x];
            let pi = ((segment.get_depth() as f32 / 10.0) + 1.0) * gas.fr_n2();
            let p = po + (pi - po) *
                (1.0 - (2.0_f32.powf(-1.0*segment.get_time() as f32 / util::ZHL16_N2_HALFLIFE[x])));
            self.p_n2[x] = p;
            self.p_t[x] = p;
        }
        for x in 0..16 {
            let po = self.p_he[x];
            let pi = ((segment.get_depth() as f32 / 10.0) + 1.0) * gas.fr_he();
            let p = po + (pi - po) *
                (1.0 - (2.0_f32.powf(-1.0*segment.get_time() as f32 / util::ZHL16_HE_HALFLIFE[x])));
            self.p_he[x] = p;
            self.p_t[x] += p;
        }
        self.diver_depth = segment.get_depth();
    }

    pub(crate) fn find_ascent_ceiling(&self) -> f32 {
        let mut ceilings: [f32; 16] = [0.0; 16];
        for x in 0..16 {
            let a = (util::ZHL16_N2_A[x] * self.p_n2[x] + util::ZHL16_HE_A[x] * self.p_he[x]) /
                (self.p_n2[x] + self.p_he[x]);

            let b = (util::ZHL16_N2_B[x] * self.p_n2[x] + util::ZHL16_HE_B[x] * self.p_he[x]) /
                (self.p_n2[x] + self.p_he[x]);
            ceilings[x] = ((self.p_n2[x] + self.p_he[x]) - a) * b;
        }
        ceilings.iter().cloned().fold(0./0., f32::max)
    }

    pub(crate) fn next_stop(&self, ascent_rate: isize, descent_rate: isize, gas: &common::gas::Gas) -> DiveSegment {
        let stop_depth = (3.0*(
            (common::bar_mtr(self.find_ascent_ceiling())/3.0)
                .ceil())) as usize;
        let mut stop_time: usize = 0;
        let mut in_limit: bool = false;
        while !in_limit {
            let mut virtual_zhl16 = self.clone();
            let segment = DiveSegment::new(SegmentType::DecoStop,
                                           stop_depth, stop_time, ascent_rate, descent_rate);
            virtual_zhl16.add_depth_change(&segment, gas);
            virtual_zhl16.add_bottom(&segment, gas);
            in_limit = virtual_zhl16.find_ascent_ceiling() < common::mtr_bar(stop_depth as f32)
                - 0.3;
            stop_time += 1;
        }
        DiveSegment::new(SegmentType::DecoStop, stop_depth, stop_time,
                         ascent_rate, descent_rate)
    }

    pub(crate) fn ndl(&self, gas: &common::gas::Gas) -> Option<usize> {
        let mut ndl:usize = 0;
        let mut in_ndl:bool = true;
        while in_ndl {
            let mut virtual_zhl16 = self.clone();
            let virtual_segment = DiveSegment::new(SegmentType::NoDeco,
                                                   virtual_zhl16.diver_depth, ndl, 0,
                                                   0);
            virtual_zhl16.add_bottom(&virtual_segment, gas);
            in_ndl = virtual_zhl16.find_ascent_ceiling() < 1.0;
            ndl += 1;
            if ndl > 999 {
                return Some(std::usize::MAX)
            }
        }
        Some(ndl)
    }
}

impl common::deco_algorithm::DecoAlgorithm for ZHL16 {
    fn add_bottom_time(&mut self, segment: &common::dive_segment::DiveSegment,
                       gas: &common::gas::Gas) {
        self.add_depth_change(&segment, gas);
        self.add_bottom(&segment, gas);
    }

    fn get_stops(&self, ascent_rate: isize, descent_rate: isize, gas: &common::gas::Gas) -> Vec<DiveSegment> {
        let mut stops: Vec<DiveSegment> = Vec::new();
        let mut virtual_zhl16 = self.clone();

        if virtual_zhl16.find_ascent_ceiling() < 1.0 {
            let _ndl = match virtual_zhl16.ndl(gas) {
                Some(t) => {
                    stops.push(DiveSegment::new(SegmentType::NoDeco, 0,
                                                t, 0, 0))
                },
                None =>  {
                    panic!("Ascent ceiling is < 1.0 but NDL was found.")
                }
            };
        }

        while virtual_zhl16.find_ascent_ceiling() > 1.0 {
            let stop = virtual_zhl16.next_stop(ascent_rate, descent_rate, gas);
            // Do the deco stop
            virtual_zhl16.add_depth_change(&stop, gas);
            virtual_zhl16.add_bottom_time(&stop, gas);
            stops.push(stop);
        }
        stops
    }
}