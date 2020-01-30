use crate::common;
use std::f32::consts::{LN_2, E};
use crate::common::deco_stop::DecoStop;
use crate::common::deco_stop::StopType;
use crate::common::deco_stop::StopType::Stop;

mod util;

#[derive(Debug, Copy, Clone)]
pub struct ZHL16 {
    p_n2: [f32; 16],
    p_he: [f32; 16],
    p_t: [f32; 16],
    diver_depth: usize,
    ascent_rate: isize,
    descent_rate: isize,
}

impl ZHL16 {

    fn set_tissue_pressure(&mut self, tissue: usize, p_n: f32, p_h: f32) {
        self.p_n2[tissue-1] = p_n;
        self.p_he[tissue-1] = p_h;
        self.p_t[tissue-1] = p_n + p_h;
    }

    pub fn new(ascent_rate: isize, descent_rate: isize, tissue_gas: &common::gas::Gas) -> ZHL16 {
        ZHL16 {
            p_n2: [tissue_gas.fr_n2(); 16],
            p_he: [tissue_gas.fr_he(); 16],
            p_t: [tissue_gas.fr_n2() + tissue_gas.fr_he(); 16],
            diver_depth: 0,
            ascent_rate,
            descent_rate
        }
    }

    pub(crate) fn add_depth_change(&mut self, depth: usize, rate: isize, gas: &common::gas::Gas) {
        let delta_depth = (depth as isize) - (self.diver_depth as isize);
        //let rate = common::mtr_bar(rate as f32);
        let t: f32 = delta_depth as f32 / rate as f32;
        for x in 0..16 {
            // Po = Pn [this is the current loading at the start]
            // Pio = (Depth - WATERVAPOUR) * fN2
            // R = DESCENT_RATE * fN2
            // k = log(2) / [nitrogen half-time of this tissue]
            // Pn = Pio + R*(t - (1/k)) - (Pio - Po - (R / k))* exp(-k*t)

            let po = self.p_n2[x];
            let pio: f32 = common::mtr_bar(depth as f32) * gas.fr_n2();
            let r = (rate as f32 / 10.0) * gas.fr_n2();
            let k = LN_2 / util::ZHL16_N2_HALFLIFE[x];
            //println!("N2 tissue {}:: po: {}, pio: {}, r: {}, k: {}", x+1, po, pio, r, k);
            let pn: f32 = pio + r * (t-(1.0/k)) - (pio - po - (r/k)) * E.powf(-1.0*k*t);
            self.p_n2[x] = pn;
            self.p_t[x] = pn;
        }
        for x in 0..16 {
            let po = self.p_he[x];
            let pio: f32 = common::mtr_bar(depth as f32) * gas.fr_he();
            let r = (rate as f32 / 10.0) * gas.fr_he();
            let k = LN_2 / util::ZHL16_HE_HALFLIFE[x];
            //println!("He tissue {}:: po: {}, pio: {}, r: {}, k: {}", x+1, po, pio, r, k);
            let ph: f32 = pio + r * (t-(1.0/k)) - (pio - po - (r/k)) * E.powf(-1.0*k*t);
            self.p_he[x] = ph;
            self.p_t[x] += ph;
        }
        self.diver_depth = depth;
    }

    pub(crate) fn add_bottom(&mut self, depth: usize, time: usize, gas: &common::gas::Gas) {
        for x in 0..16 {
            let po = self.p_n2[x];
            let pi = ((depth as f32 / 10.0) + 1.0) * gas.fr_n2();
            //println!("N2 tissue {}:: po: {}, pi: {}", x+1, po, pi);
            let p = po + (pi - po) *
                (1.0 - (2.0_f32.powf(-1.0*time as f32 / util::ZHL16_N2_HALFLIFE[x])));
            self.p_n2[x] = p;
            self.p_t[x] = p;
        }
        for x in 0..16 {
            let po = self.p_he[x];
            let pi = ((depth as f32 / 10.0) + 1.0) * gas.fr_he();
            let p = po + (pi - po) *
                (1.0 - (2.0_f32.powf(-1.0*time as f32 / util::ZHL16_HE_HALFLIFE[x])));
            self.p_he[x] = p;
            self.p_t[x] += p;
        }
        self.diver_depth = depth;
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

    pub(crate) fn next_stop(&self, gas: &common::gas::Gas) -> DecoStop {
        let stop_depth = (3.0*(
            (common::bar_mtr(self.find_ascent_ceiling())/3.0)
                .ceil())) as usize;
        let mut stop_time: usize = 0;
        let mut in_limit: bool = false;
        while !in_limit {
            let mut virtual_zhl16 = self.clone();
            virtual_zhl16.add_depth_change(stop_depth, common::DEFAULT_ASCENT_RATE, gas);
            virtual_zhl16.add_bottom(stop_depth, stop_time, gas);
            in_limit = virtual_zhl16.find_ascent_ceiling() < common::mtr_bar(stop_depth as f32)
                - 0.3;
            stop_time += 1;
        }
        DecoStop::new(StopType::Stop, stop_depth, stop_time)
    }

    pub(crate) fn ndl(&self, gas: &common::gas::Gas) -> Option<usize> {
        let mut ndl:usize = 0;
        let mut in_ndl:bool = true;
        while in_ndl {
            let mut virtual_zhl16 = self.clone();
            virtual_zhl16.add_bottom(virtual_zhl16.diver_depth, ndl, gas);
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
    fn add_bottom_time(&mut self, depth: usize, time: usize, gas: &common::gas::Gas) {
        self.add_depth_change(depth, self.descent_rate, gas);
        //println!("Descend to {}m with {:?}:: {:?}\n", depth, gas, self);

        self.add_bottom(depth, time, gas);
        //println!("Bottom time of {}m for {}min with {:?}:: {:?}\n", depth, time, gas, self);
    }

    fn get_stops(&self, gas: &common::gas::Gas) -> Vec<common::deco_stop::DecoStop> {
        let mut stops: Vec<common::deco_stop::DecoStop> = Vec::new();
        let mut virtual_zhl16 = self.clone();

        if virtual_zhl16.find_ascent_ceiling() < 1.0 {
            let _ndl = match virtual_zhl16.ndl(gas) {
                Some(t) => {
                    stops.push(DecoStop::new(StopType::NoDeco, 0, t))
                },
                None =>  {
                    panic!("Ascent ceiling is < 1.0 but NDL was found.")
                }
            };
        }

        while virtual_zhl16.find_ascent_ceiling() > 1.0 {
            let stop = virtual_zhl16.next_stop(gas);
            // Do the deco stop
            virtual_zhl16.add_depth_change(stop.get_depth() as usize,
                                           self.ascent_rate, gas);
            virtual_zhl16.add_bottom_time(stop.get_depth() as usize, stop.get_time(),
                                          gas);
            stops.push(stop);
        }
        stops
    }
}