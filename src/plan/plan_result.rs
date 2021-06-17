use crate::gas::Gas;
use crate::segment::Segment;
use crate::tissue::Tissue;
use std::collections::HashSet;

pub struct PlanResult {
    end_tissue: Tissue,
    segments: Vec<Segment>,
    gas_used: HashSet<(Gas, u32)>,
}
