use crate::common::dive_segment;
use crate::gas::Gas;
use std::fmt::Debug;

pub trait DecoAlgorithm: Copy + Clone + Debug {
    fn add_bottom_time(&mut self, segment: &dive_segment::DiveSegment, gas: &Gas) -> Option<Vec<dive_segment::DiveSegment>>;
    fn get_stops(&self, ascent_rate: isize, descent_rate: isize, gas: &Gas) -> Vec<dive_segment::DiveSegment>;
}