use crate::common::gas;
use crate::common::dive_segment;

pub trait DecoAlgorithm {
    fn add_bottom_time(&mut self, segment: &dive_segment::DiveSegment, gas: &gas::Gas) -> Option<Vec<dive_segment::DiveSegment>>;
    fn get_stops(&self, ascent_rate: isize, descent_rate: isize, gas: &gas::Gas) -> Vec<dive_segment::DiveSegment>;
}