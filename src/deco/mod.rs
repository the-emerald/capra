use crate::environment::Environment;
use crate::gas::Gas;
use crate::segment::Segment;

use crate::tissue::Tissue;
use crate::units::depth::Depth;
use crate::units::rate::Rate;

pub mod zhl16;

pub const TISSUE_COUNT: usize = 16;

pub trait DecoAlgorithm {
    fn add_segment(self, segment: &Segment, gas: &Gas, environment: Environment) -> Self;
    fn get_stops(
        self,
        ascent_rate: Rate,
        descent_rate: Rate,
        gas: &Gas,
        environment: Environment,
    ) -> Vec<Segment>;
    fn tissue(&self) -> Tissue;
    fn model_depth(&self) -> Depth;
}
