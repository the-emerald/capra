use crate::deco::DecoAlgorithm;
use crate::gas::Gas;
use crate::parameter::Parameters;
use crate::plan::plan_result::PlanResult;
use crate::plan::DivePlan;
use crate::segment::{Segment, SegmentType};
use crate::units::depth::Depth;
use crate::util::time_taken;
use std::cmp::Ordering;
use std::collections::HashSet;

pub struct OpenCircuit<T>
where
    T: DecoAlgorithm + Clone,
{
    deco: T,
    bottom_segments: Vec<(Segment, Gas)>,
    deco_gases: HashSet<(Gas, Option<Depth>)>,
    parameters: Parameters,
}

impl<T> OpenCircuit<T>
where
    T: DecoAlgorithm + Clone,
{
    pub fn new(
        deco: T,
        bottom_segments: &[(Segment, Gas)],
        deco_gases: &[(Gas, Option<Depth>)],
        parameters: Parameters,
    ) -> Self {
        Self {
            deco,
            bottom_segments: bottom_segments.to_vec(),
            deco_gases: deco_gases.to_vec().into_iter().collect::<HashSet<_>>(),
            parameters,
        }
    }

    fn level_to_level(
        &self,
        mut running_model: T,
        start: &(Segment, Gas),
        end: Option<&(Segment, Gas)>,
        stops_performed: &mut Vec<(Segment, Gas)>,
    ) -> T {
        // If end segment is defined, check if there is a depth change
        if let Some(end) = end {
            match start.0.end_depth().cmp(&end.0.start_depth()) {
                Ordering::Less => {
                    // Descend to link up start <-> end
                    let descent = Segment::new(
                        SegmentType::AscDesc,
                        start.0.end_depth(),
                        end.0.start_depth(),
                        time_taken(
                            self.parameters.descent_rate(),
                            start.0.end_depth(),
                            end.0.start_depth(),
                        ),
                        self.parameters.ascent_rate(),
                        self.parameters.descent_rate(),
                    )
                    .unwrap();
                    // Add to model
                    stops_performed.push((descent, start.1));
                    return running_model.add_segment(
                        &descent,
                        &start.1,
                        self.parameters.environment(),
                    );
                }
                Ordering::Equal => {
                    // If both are equal then no segments to add.
                    return running_model;
                }
                Ordering::Greater => {} // Continue to main algorithm
            }
        }
        let mut virtual_deco = running_model;
        let end_depth = end
            .map(|(segment, _)| segment.end_depth())
            .unwrap_or_default();

        todo!()
    }
}

impl<T> DivePlan for OpenCircuit<T>
where
    T: DecoAlgorithm + Clone,
{
    fn plan() -> PlanResult {
        todo!()
    }
}
