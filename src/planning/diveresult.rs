use crate::deco::deco_algorithm::DecoAlgorithm;
use crate::common::dive_segment::DiveSegment;
use crate::common::gas::Gas;
use std::collections::HashMap;

#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct DiveResult<T: DecoAlgorithm> {
    deco_algorithm: T,
    total_segments: Vec<(DiveSegment, Gas)>,
    gas_used: HashMap<Gas, usize>
}

impl<T: DecoAlgorithm> DiveResult<T> {
    pub fn new(deco_algorithm: T, total_segments: Vec<(DiveSegment, Gas)>, gas_used: HashMap<Gas, usize>) -> Self {
        Self {
            deco_algorithm,
            total_segments,
            gas_used
        }
    }

    pub fn total_segments(&self) -> &Vec<(DiveSegment, Gas)> {
        &self.total_segments
    }

    pub fn gas_used(&self) -> &HashMap<Gas, usize> {
        &self.gas_used
    }
}