use crate::common::dive_segment::DiveSegment;
use crate::gas::Gas;

pub trait Dive {
    fn execute_dive(&mut self) -> Vec<(DiveSegment, Gas)>;
}