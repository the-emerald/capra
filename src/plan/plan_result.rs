use crate::gas::Gas;
use crate::segment::Segment;
use crate::tissue::Tissue;
use crate::units::consumption::GasConsumption;
use std::collections::HashMap;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlanResult {
    end_tissue: Tissue,
    segments: Vec<(Segment, Gas)>,
    gas_used: HashMap<Gas, GasConsumption>,
}

impl PlanResult {
    pub fn end_tissue(&self) -> Tissue {
        self.end_tissue
    }
    pub fn segments(&self) -> &Vec<(Segment, Gas)> {
        &self.segments
    }
    pub fn gas_used(&self) -> &HashMap<Gas, GasConsumption> {
        &self.gas_used
    }
}

impl PlanResult {
    pub fn new(
        end_tissue: Tissue,
        segments: &[(Segment, Gas)],
        gas_used: &HashMap<Gas, GasConsumption>,
    ) -> Self {
        Self {
            end_tissue,
            segments: segments.to_vec(),
            gas_used: gas_used.clone(),
        }
    }
}
