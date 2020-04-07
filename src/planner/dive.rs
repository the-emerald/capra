use crate::common::dive_segment::DiveSegment;
use crate::gas::Gas;
use crate::common::deco_algorithm::DecoAlgorithm;

pub trait Dive<T: DecoAlgorithm> {
    fn execute_dive(&mut self) -> Vec<(DiveSegment, Gas)>;
    fn finish(self) -> T;
}