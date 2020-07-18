use crate::common::dive_segment;
use std::fmt::Debug;
use crate::common::gas::Gas;
use crate::common::dive_segment::DiveSegment;
use crate::deco::tissue::Tissue;

pub trait DecoAlgorithm: Copy + Clone + Debug {
    fn add_dive_segment(&mut self, segment: &DiveSegment, gas: &Gas, metres_per_bar: f64);
    fn surface(&mut self, ascent_rate: isize, descent_rate: isize, gas: &Gas, metres_per_bar: f64) -> Vec<dive_segment::DiveSegment>;
    fn get_tissue(&self) -> Tissue;
    fn get_stops(&self, ascent_rate: isize, descent_rate: isize, gas: &Gas, metres_per_bar: f64) -> Vec<dive_segment::DiveSegment> {
        let mut virtual_deco = *self;
        virtual_deco.surface(ascent_rate, descent_rate, gas, metres_per_bar)
    }
}