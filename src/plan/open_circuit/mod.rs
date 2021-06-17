use crate::deco::DecoAlgorithm;
use crate::environment::Environment;
use crate::gas::Gas;
use crate::parameter::Parameters;
use crate::plan::plan_result::PlanResult;
use crate::plan::DivePlan;
use crate::segment::SegmentType::{AscDesc, DecoStop};
use crate::segment::{Segment, SegmentType};
use crate::units::depth::Depth;
use crate::units::pressure::Pressure;
use crate::util::time_taken;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter;
use time::Duration;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
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

    fn find_gas_switch_point(
        segments: &[Segment],
        current_gas: &Gas,
        available_gases: &HashSet<(Gas, Option<Depth>)>,
        environment: Environment,
    ) -> Option<(Segment, Gas)> {
        for stop in segments
            .iter()
            .filter(|segment| segment.segment_type() != AscDesc)
        {
            let best_gas = available_gases
                .iter()
                // Fill in all MODs
                .map(|(gas, depth)| {
                    (
                        gas,
                        depth.unwrap_or_else(|| {
                            gas.max_operating_depth(
                                stop.start_depth(),
                                Pressure(1.601),
                                environment,
                            )
                        }),
                    )
                })
                // Remove gases with too high MOD
                .filter(|(_, depth)| depth <= &stop.start_depth())
                // Remove gases with too high END
                .filter(|(gas, _)| {
                    gas.equivalent_narcotic_depth(stop.start_depth()) <= stop.start_depth()
                })
                // Sort by ppO2 (descending)
                .sorted_by(|(fst, _), (snd, _)| {
                    snd.pp_o2(stop.start_depth(), environment)
                        .partial_cmp(&fst.pp_o2(stop.start_depth(), environment))
                        .unwrap()
                })
                // Take first
                .next();

            if let Some((best, _)) = best_gas {
                if best != current_gas {
                    return Some((*stop, *best));
                }
            }
        }
        None
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

        let end_depth = end
            .map(|(segment, _)| segment.end_depth())
            .unwrap_or_default();

        // Find stops between start and end using start gas
        let stops = running_model
            .clone()
            .get_stops(
                self.parameters.ascent_rate(),
                self.parameters.descent_rate(),
                &start.1,
                self.parameters.environment(),
            )
            .into_iter()
            .take_while(|x| x.start_depth() > end_depth)
            .collect::<Vec<_>>();

        // Only use deco gases using the surfacing stops (not for between-levels)
        let available_gases = end
            .map(|(_, gas)| vec![(start.1, None), (*gas, None)].into_iter().collect())
            .unwrap_or_else(|| self.deco_gases.clone());

        // Determine a switch-point
        let switch_point = OpenCircuit::<T>::find_gas_switch_point(
            &stops,
            &start.1,
            &available_gases,
            self.parameters.environment(),
        );

        if let (Some((switch_point, switch_gas)), true) = (
            switch_point,
            stops.iter().any(|x| x.segment_type() == DecoStop),
        ) {
            // Replay the stops until the gas switch point occurs
            for stop in stops
                .iter()
                .take_while(|stop| stop.start_depth() > switch_point.start_depth())
            {
                running_model =
                    running_model.add_segment(&stop, &start.1, self.parameters.environment());
                stops_performed.push((*stop, start.1));
            }

            // At the gas switch point, use new gas to calculate new deco schedule
            let new_stop = running_model
                .clone()
                .get_stops(
                    self.parameters.ascent_rate(),
                    self.parameters.descent_rate(),
                    &switch_gas,
                    self.parameters.environment(),
                )
                .into_iter()
                .find(|stop| {
                    stop.segment_type() == DecoStop
                        && stop.start_depth() == switch_point.start_depth()
                })
                .unwrap_or_else(|| {
                    Segment::new(
                        SegmentType::DecoStop,
                        switch_point.start_depth(),
                        switch_point.end_depth(),
                        Duration::minute(),
                        self.parameters.ascent_rate(),
                        self.parameters.descent_rate(),
                    )
                    .unwrap()
                });

            running_model =
                running_model.add_segment(&new_stop, &switch_gas, self.parameters.environment());
            stops_performed.push((new_stop, switch_gas));
            self.level_to_level(running_model, &(new_stop, switch_gas), end, stops_performed)
        } else {
            // Push segments and return
            stops_performed.append(&mut stops.into_iter().zip(iter::repeat(start.1)).collect());
            running_model
        }
    }
}

impl<T> DivePlan for OpenCircuit<T>
where
    T: DecoAlgorithm + Clone,
{
    fn plan(mut self) -> PlanResult {
        let mut segments: Vec<(Segment, Gas)> = Vec::new();

        // Adjust diver depth to beginning of segments
        let asc_desc_to_beginning = {
            let start_depth = self.deco.model_depth();
            let end_depth = self.bottom_segments[0].0.start_depth();
            Segment::new(
                SegmentType::AscDesc,
                start_depth,
                end_depth,
                time_taken(
                    if start_depth > end_depth {
                        self.parameters.descent_rate()
                    } else {
                        self.parameters.ascent_rate()
                    },
                    start_depth,
                    end_depth,
                ),
                self.parameters.ascent_rate(),
                self.parameters.descent_rate(),
            )
        }
        .unwrap();

        self.deco = self.deco.add_segment(
            &asc_desc_to_beginning,
            &self.bottom_segments[0].1,
            self.parameters.environment(),
        );
        segments.push((asc_desc_to_beginning, self.bottom_segments[0].1));

        // Use a sliding window to go from segment to segment.
        for window in self.bottom_segments.windows(2) {
            let mut stops_performed: Vec<(Segment, Gas)> = Vec::new();
            let (start, end) = (window[0], window[1]);

            self.deco = self
                .deco
                .add_segment(&start.0, &start.1, self.parameters.environment());
            segments.push(start);

            self.deco =
                self.level_to_level(self.deco.clone(), &start, Some(&end), &mut stops_performed);
            segments.append(&mut stops_performed);
        }

        // Final element not captured.
        let final_stop = self.bottom_segments.last().unwrap();
        self.deco =
            self.deco
                .add_segment(&final_stop.0, &final_stop.1, self.parameters.environment());
        segments.push(*final_stop);

        let mut stops_performed: Vec<(Segment, Gas)> = Vec::new();
        self.deco = self.level_to_level(self.deco.clone(), &final_stop, None, &mut stops_performed);
        segments.append(&mut stops_performed);

        PlanResult::new(self.deco.tissue(), &segments, &[])
    }
}
