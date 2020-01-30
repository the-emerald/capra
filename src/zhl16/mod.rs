use crate::common;
use std::f32::consts::{LN_2, E};

mod util;

#[derive(Debug)]
pub struct ZHL16 {
    p_n2: [f32; 16],
    p_he: [f32; 16],
    p_t: [f32; 16]
}

impl ZHL16 {

    fn set_tissue_pressure(&mut self, tissue: usize, p_n: f32, p_h: f32) {
        self.p_n2[tissue-1] = p_n;
        self.p_he[tissue-1] = p_h;
        self.p_t[tissue-1] = p_n + p_h;
    }

    pub fn new() -> ZHL16 {
        ZHL16 {
            p_n2: [0.0; 16],
            p_he: [0.0; 16],
            p_t: [0.0; 16]
        }
    }

    pub fn initialise_tissues(&mut self, gas: &common::gas::Gas) {
        for i in 1..17 {
            self.set_tissue_pressure(i, gas.fr_n2(), gas.fr_he());
        }
    }

    pub(crate) fn add_depth_change(&mut self, depth: usize, rate: isize, gas: &common::gas::Gas) {
        // Only supports change of depth downwards
        let t: f32 = depth as f32 / rate as f32;
        for x in 0..16 {
            // Po = Pn [this is the current loading at the start]
            // Pio = (Depth - WATERVAPOUR) * fN2
            // R = DESCENT_RATE * fN2
            // k = log(2) / [nitrogen half-time of this tissue]
            // Pn = Pio + R*(t - (1/k)) - (Pio - Po - (R / k))* exp(-k*t)

            let po = self.p_n2[x];
            let pio: f32 = ((depth as f32 / 10.0) + 1.0) * gas.fr_n2();
            let r = (rate as f32 / 10.0) * gas.fr_n2();
            let k = LN_2 / util::ZHL16_N2_HALFLIFE[x];
            //println!("N2 tissue {}:: po: {}, pio: {}, r: {}, k: {}", x+1, po, pio, r, k);
            let pn: f32 = pio + r * (t-(1.0/k)) - (pio - po - (r/k)) * E.powf(-1.0*k*t);
            self.p_n2[x] = pn;
            self.p_t[x] = pn;
        }

        for x in 0..16 {
            let po = self.p_he[x];
            let pio: f32 = ((depth as f32 / 10.0) + 1.0) * gas.fr_he();
            let r = (rate as f32 / 10.0) * gas.fr_he();
            let k = LN_2 / util::ZHL16_HE_HALFLIFE[x];
            //println!("He tissue {}:: po: {}, pio: {}, r: {}, k: {}", x+1, po, pio, r, k);
            let ph: f32 = pio + r * (t-(1.0/k)) - (pio - po - (r/k)) * E.powf(-1.0*k*t);
            self.p_he[x] = ph;
            self.p_t[x] += ph;
        }
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
    }
}

impl common::deco_algorithm::DecoAlgorithm for ZHL16 {
    fn add_bottom_time(&mut self, depth: usize, time: usize, gas: &common::gas::Gas) {
        self.add_depth_change(depth, common::DESCENT_RATE, gas);
        println!("Descend to {}m with {:?}:: {:?}\n", depth, gas, self);

        self.add_bottom(depth, time, gas);
        println!("Bottom time of {}m for {}min with {:?}:: {:?}\n", depth, time, gas, self);

    }

    fn get_stops() -> Vec<common::deco_stop::DecoStop> {
        unimplemented!()
    }
}