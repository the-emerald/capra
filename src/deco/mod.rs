use crate::gas::Gas;
use crate::segment::Segment;
use crate::units::pressure::Pressure;
use crate::environment::Environment;

pub mod zhl16;

pub const TISSUE_COUNT: usize = 16;

pub trait DecoAlgorithm {
    fn add_segment(self, segment: &Segment, gas: &Gas, environment: Environment) -> Self;
    fn ascent_ceiling(&self) -> Pressure;
}
