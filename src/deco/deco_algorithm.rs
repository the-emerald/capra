use crate::common::dive_segment;
use std::fmt::Debug;
use crate::common::gas::Gas;

pub trait DecoAlgorithm: Copy + Clone + Debug {
    fn add_bottom_time(&mut self, segment: &dive_segment::DiveSegment, gas: &Gas) -> Option<Vec<dive_segment::DiveSegment>>;
    fn surface(&mut self, ascent_rate: isize, descent_rate: isize, gas: &Gas) -> Vec<dive_segment::DiveSegment>;
    fn get_stops(&self, ascent_rate: isize, descent_rate: isize, gas: &Gas) -> Vec<dive_segment::DiveSegment> {
        let mut virtual_deco = *self;
        virtual_deco.surface(ascent_rate, descent_rate, gas)
    }
}