use crate::common::dive_segment::DiveSegment;
use crate::deco::deco_algorithm::DecoAlgorithm;
use crate::common::gas::Gas;

pub trait Dive<T: DecoAlgorithm> {
    fn execute_dive(&self) -> (T, Vec<(DiveSegment, Gas)>);
}