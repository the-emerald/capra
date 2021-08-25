use crate::plan::plan_result::PlanResult;

pub mod open_circuit;
pub mod plan_result;

pub trait DivePlan {
    fn plan(self) -> PlanResult;
    // TODO: Plan with tanks
}
