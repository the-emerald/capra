use crate::gas::Gas;
use crate::segment::Segment;
use crate::tissue::Tissue;
use std::collections::HashSet;

pub struct PlanResult {
    end_tissue: Tissue,
    segments: Vec<(Segment, Gas)>,
    gas_used: HashSet<(Gas, u32)>,
}

impl PlanResult {
    pub fn end_tissue(&self) -> Tissue {
        self.end_tissue
    }
    pub fn segments(&self) -> &[(Segment, Gas)] {
        &self.segments
    }
    pub fn gas_used(&self) -> &HashSet<(Gas, u32)> {
        &self.gas_used
    }
}

impl PlanResult {
    pub fn new(end_tissue: Tissue, segments: &[(Segment, Gas)], gas_used: &[(Gas, u32)]) -> Self {
        Self {
            end_tissue,
            segments: segments.to_vec(),
            gas_used: gas_used.to_vec().into_iter().collect(),
        }
    }
}
