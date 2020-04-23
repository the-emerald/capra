use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::dive_plan::dive::Dive;
use crate::dive_plan::{gas_in_ppo2_range, equivalent_narcotic_depth, PPO2_MINIMUM, PPO2_MAXIMUM_DECO};
use crate::deco::deco_algorithm::DecoAlgorithm;
use crate::common::gas::{Gas, partial_pressure};
use crate::gas_plan::GasPlan;
use crate::gas_plan::tank::Tank;
use time::Duration;
use crate::common::dive_segment::SegmentType::{AscDesc, DecoStop};
use std::cmp::Ordering;
use std::collections::HashMap;
use crate::common::time_taken;
use std::iter;

#[derive(Copy, Clone, Debug)]
pub struct OpenCircuit<'a, T: DecoAlgorithm> {
    deco_algorithm: T,
    deco_gases: &'a [(Gas, Option<usize>)],
    bottom_segments: &'a [(DiveSegment, Gas)],

    ascent_rate: isize,
    descent_rate: isize,
    metres_per_bar: f64,

    sac_bottom: usize,
    sac_deco: usize
}

impl<'a, T: DecoAlgorithm> OpenCircuit<'a, T> {
    pub fn new(deco_algorithm: T, deco_gases: &'a [(Gas, Option<usize>)],
               bottom_segments: &'a [(DiveSegment, Gas)], ascent_rate: isize,
               descent_rate: isize, water_density: f64, sac_bottom: usize, sac_deco: usize) -> Self {
        OpenCircuit {
            deco_algorithm,
            deco_gases,
            bottom_segments,
            ascent_rate,
            descent_rate,
            metres_per_bar: 10000.0/water_density,
            sac_bottom,
            sac_deco
        }
    }

    fn filter_gases<'b>(segment: &DiveSegment, gases: &'b [(Gas, Option<usize>)], metres_per_bar: f64) -> Vec<&'b Gas> {
        let mut candidates = gases
            .iter()
            .filter(|x| x.1
                .map_or(true, |t| t >= segment.get_start_depth()))
            .map(|x| &x.0)
            .collect::<Vec<&Gas>>();

        candidates = candidates
            .into_iter()
            .filter(|a|
                gas_in_ppo2_range(segment.get_start_depth(), PPO2_MINIMUM, PPO2_MAXIMUM_DECO, a, metres_per_bar))
            .collect(); // filter gases not in ppo2 range

        candidates = candidates.into_iter()
            .filter(|a|
                equivalent_narcotic_depth(segment.get_start_depth(), a) <= segment.get_start_depth())
            .collect(); // filter gases over E.N.D.

        candidates.sort_by(|a, b|
            partial_pressure(segment.get_start_depth(), a.fr_o2(), metres_per_bar)
                .partial_cmp(&partial_pressure(segment.get_start_depth(), b.fr_o2(), metres_per_bar))
                .unwrap()); // sort by descending order of ppo2

        candidates
    }

    fn find_gas_switch_point<'c>(segments: &'c [DiveSegment], current_gas: &Gas, gases: &'c [(Gas, Option<usize>)], metres_per_bar: f64) -> Option<(&'c DiveSegment, &'c Gas)> {
        // Best gas_plan is the gas_plan that has the highest ppO2 (not over max allowed), and not over equivalent_narcotic_depth.
        for stop in segments {
            let candidate_gases = <OpenCircuit<'a, T>>::filter_gases(stop, gases, metres_per_bar);
            if candidate_gases.is_empty(){ // there no fitting candidate gases.
                continue;
            }
            if candidate_gases[candidate_gases.len()-1] != current_gas {
                return Some((stop, &candidate_gases[candidate_gases.len()-1]))
            }
        }
        None
    }

    pub(crate) fn level_to_level(&mut self, start: &(DiveSegment, Gas),
                                 end: Option<&(DiveSegment, Gas)>,
                                 stops_performed: &mut Vec<(DiveSegment, Gas)>) {

        // Check if there is any depth change
        if let Some(t) = end {
            match start.0.get_end_depth().cmp(&t.0.get_start_depth()) {
                Ordering::Less => {
                    // Create a segment with the next segment's gas
                    let descent = DiveSegment::new(
                        SegmentType::AscDesc,
                        start.0.get_end_depth(),
                        t.0.get_start_depth(),
                        time_taken(self.descent_rate, start.0.get_end_depth(), t.0.get_start_depth()),
                        self.ascent_rate,
                        self.descent_rate
                    ).unwrap();
                    self.deco_algorithm.add_dive_segment(&descent, &t.1, self.metres_per_bar);
                    return;
                }
                Ordering::Equal => {
                    // There cannot be any more segments to add.
                    return;
                },
                Ordering::Greater => {} // Continue to main algorithm
            }
        }

        let mut virtual_deco = self.deco_algorithm;
        // Find the stops between start and end using start gas
        let end_depth = match end {
            Some(t) => t.0.get_start_depth(),
            None => 0
        };
        let stops = virtual_deco
            .surface(self.ascent_rate, self.descent_rate, &start.1, self.metres_per_bar)
            .into_iter()
            .take_while(|x| x.get_start_depth() > end_depth)
            .collect::<Vec<DiveSegment>>();

        let switch_gases: Vec<(Gas, Option<usize>)> = match end {
            Some(t) => {
                vec![(start.1, None), (t.1, None)]
            },
            None => {
                self.deco_gases.to_vec()
            }
        };

        let switch_point = <OpenCircuit<'a, T>>::find_gas_switch_point(&stops, &start.1, &switch_gases, self.metres_per_bar);

        // If there are deco stops in between
        if stops.iter().any(|x| x.get_segment_type() == DecoStop) && switch_point.is_some() {
            let switch = switch_point.unwrap();
            // Rewind the algorithm
            virtual_deco = self.deco_algorithm;

            // Replay between stops until gas switch point
            for stop in stops.iter().take_while(|x| x.get_start_depth() > switch.0.get_start_depth()) {
                virtual_deco.add_dive_segment(&stop, &start.1, self.metres_per_bar);
                stops_performed.push((*stop, start.1));
            }

            // At gas switch point, use new gas and calculate new deco schedule
            let new_stop = virtual_deco.get_stops(self.ascent_rate, self.descent_rate, switch.1, self.metres_per_bar)
                .into_iter().find(|x| x.get_segment_type() == DecoStop).expect("no deco wtf");
            virtual_deco.add_dive_segment(&new_stop, switch.1, self.metres_per_bar);
            stops_performed.push((new_stop, *switch.1));

            // Call recursively with first new gas stop as start, end same
            self.deco_algorithm = virtual_deco;
            self.level_to_level(&(new_stop, *switch.1), end, stops_performed);
        }
        else {
            // Push segments and return
            // TODO: Check NDL behaviour?
            stops_performed.append(&mut stops.into_iter().zip(iter::repeat(start.1)).collect());
            self.deco_algorithm = virtual_deco;
        }
    }
}

impl<'a, T: DecoAlgorithm> Dive<T> for OpenCircuit<'a, T> {
    fn execute_dive(&self) -> (T, Vec<(DiveSegment, Gas)>) {
        let mut total_segments: Vec<(DiveSegment, Gas)> = Vec::new();
        let mut virtual_dive = *self;

        // Create the AscDesc to the first segment
        let descent_to_beginning = DiveSegment::new(
            AscDesc,
            0,
            self.bottom_segments[0].0.get_start_depth(),
            time_taken(
                self.descent_rate, 0, self.bottom_segments[0].0.get_start_depth()
            ),
            self.ascent_rate,
            self.descent_rate
        ).unwrap();

        virtual_dive.deco_algorithm.add_dive_segment(&descent_to_beginning, &self.bottom_segments[0].1, self.metres_per_bar);
        total_segments.push((descent_to_beginning, self.bottom_segments[0].1));

        for win in self.bottom_segments.windows(2) {
            let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
            let start = win[0];
            let end = win[1];

            virtual_dive.deco_algorithm.add_dive_segment(&start.0, &start.1, self.metres_per_bar);
            total_segments.push(start);

            virtual_dive.level_to_level(&start, Some(&end),&mut stops_performed);
            total_segments.append(&mut stops_performed);
        }

        // However the sliding window does not capture the final element.
        let final_stop = self.bottom_segments.last().unwrap();
        virtual_dive.deco_algorithm.add_dive_segment(&final_stop.0, &final_stop.1, self.metres_per_bar);
        total_segments.push(*final_stop);

        let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
        virtual_dive.level_to_level(&final_stop, None, &mut stops_performed);
        total_segments.append(&mut stops_performed);

        (virtual_dive.deco_algorithm, total_segments)
    }
}

impl<'a, T: DecoAlgorithm> GasPlan<T> for OpenCircuit<'a, T> {
    fn plan_forwards(&self) -> HashMap<Gas, usize> {  // Given a dive profile, how much gas do we need?
        let mut gas_plan: HashMap<Gas, usize> = HashMap::new();
        let all_segments = self.execute_dive().1;

        // All segments
        for (segment, gas) in all_segments {
            let gas_consumed = match segment.get_segment_type() {
                SegmentType::DecoStop => <Self as GasPlan<T>>::calculate_consumed(&segment, self.sac_deco, self.metres_per_bar),
                _ => <Self as GasPlan<T>>::calculate_consumed(&segment, self.sac_bottom, self.metres_per_bar)
            };
            let gas_needed = *(gas_plan.entry(gas).or_insert(0)) + gas_consumed;
            gas_plan.insert(gas, gas_needed);
        }
        gas_plan
    }

    fn plan_backwards(&self, tanks: &[Tank]) -> Vec<(DiveSegment, Gas)> {
        unimplemented!() // TODO: Implement backwards planning
    }
}
