use crate::common::dive_segment::DiveSegment;
use crate::common::gas::Gas;

pub trait Dive {
    fn plan_dive(&mut self) -> Vec<(DiveSegment, Gas)>;
}