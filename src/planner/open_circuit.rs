use crate::common::deco_algorithm::DecoAlgorithm;
use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::{Gas, partial_pressure};
use crate::planner::dive::Dive;
use crate::planner::{gas_in_ppo2_range, equivalent_narcotic_depth, PPO2_MINIMUM, PPO2_MAXIMUM_DECO};

#[derive(Copy, Clone, Debug)]
pub struct OpenCircuit<'a, T: DecoAlgorithm + Copy + Clone> {
    deco_algorithm: T,
    deco_gases: &'a [(Gas, Option<usize>)],
    bottom_segments: &'a [(DiveSegment, Gas)],

    ascent_rate: isize,
    descent_rate: isize
}

impl<'a, T: DecoAlgorithm + Copy + Clone> OpenCircuit<'a, T> {
    pub fn new(deco_algorithm: T, deco_gases: &'a [(Gas, Option<usize>)],
               bottom_segments: &'a [(DiveSegment, Gas)], ascent_rate: isize,
               descent_rate: isize) -> Self {
        OpenCircuit {
            deco_algorithm,
            deco_gases,
            bottom_segments,
            ascent_rate,
            descent_rate
        }
    }

    fn determine_gas_switch(segments: &[DiveSegment], current_gas: &Gas, gases: &'a [(Gas, Option<usize>)]) -> Option<(DiveSegment, &'a Gas)> {
        // Best gas is the gas that has the highest ppO2 (not over max allowed), and not over equivalent_narcotic_depth.
        for stop in segments {
            if stop.get_segment_type() == SegmentType::AscDesc {
                continue;
            }
            let mut candidate_gases = Vec::new();
            for gas in gases { // Do not push any candidates that are deeper than MOD
                match gas.1 {
                    Some(t) => {
                        if t >= stop.get_end_depth() {
                            candidate_gases.push(&gas.0);
                        }
                    }
                    None => {
                        candidate_gases.push(&gas.0)
                    }
                }
            }
            candidate_gases = candidate_gases.iter().filter(|a|
                gas_in_ppo2_range(stop.get_end_depth(), PPO2_MINIMUM, PPO2_MAXIMUM_DECO, a)).cloned().collect(); // filter gases not in ppo2 range

            candidate_gases = candidate_gases.iter().filter(|a|
                equivalent_narcotic_depth(stop.get_end_depth(), a) <= stop.get_end_depth()).cloned().collect(); // filter gases over E.N.D.

            candidate_gases.sort_by(|a, b|
                partial_pressure(stop.get_end_depth(), a.fr_o2())
                    .partial_cmp(&partial_pressure(stop.get_end_depth(), b.fr_o2()))
                    .unwrap()); // sort by descending order of ppo2

            if candidate_gases.is_empty(){ // there no fitting candidate gases.
                continue;
            }

            if candidate_gases[candidate_gases.len()-1] != current_gas {
                return Some((*stop, &candidate_gases[candidate_gases.len()-1]))
            }
        }
        None
    }

    pub(crate) fn level_to_level(&mut self, start_segment: &DiveSegment,
                                 end_segment: Option<&DiveSegment>, start_gas: &Gas,
                                 stops_performed: &mut Vec<(DiveSegment, Gas)>) {
        // Returns the deco model AFTER operations are done.

        if let Some(t) = end_segment {
            if start_segment.get_end_depth() == t.get_end_depth() { // Check if there is a depth change
                return;
            }
        }

        let mut virtual_deco = self.deco_algorithm.clone();
        let intermediate_stops = match end_segment { // Check if there are intermediate stops
            Some(t) => {
                let zero_to_t_segment = DiveSegment::new(SegmentType::AscDesc,
                                                         t.get_start_depth(), t.get_end_depth(),
                                                         0, self.ascent_rate, self.descent_rate).unwrap();
                virtual_deco.add_bottom_time(&zero_to_t_segment, start_gas)
            }, // More stops: add the next bottom.
            None => { // Next "stop" is a surface:
                let s = virtual_deco.get_stops(start_segment.get_ascent_rate(), start_segment.get_descent_rate(), start_gas);
                match s[0].get_segment_type() {
                    SegmentType::NoDeco => {
                        return;
                    },
                    _ => Some(s)
                }
            }
        };
        match intermediate_stops {
            Some(t) => { // There are deco stops to perform.
                let switch = <OpenCircuit<'a, T>>::determine_gas_switch(&t, start_gas, self.deco_gases);
                match switch {
                    Some(u) => { // There are gas switches to perform. u = target stop
                        virtual_deco = self.deco_algorithm.clone(); // Rewind to beginning of level
                        for i in t {
                            if i.get_segment_type() == SegmentType::AscDesc {
                                continue;
                            }
                            if i.get_end_depth() == u.0.get_end_depth() { // Replay to stop **before** u
                                break;
                            }
                            virtual_deco.add_bottom_time(&i, start_gas);
                            stops_performed.push((i, *start_gas));
                        }

                        let mut new_stop_time_deco = virtual_deco; // Calculate the new stop time
                        let test_segment = DiveSegment::new(SegmentType::DiveSegment,
                                                            u.0.get_start_depth(), u.0.get_end_depth(),
                                                            0, -self.ascent_rate, self.descent_rate).unwrap();
                        new_stop_time_deco.add_bottom_time(&test_segment, start_gas); // Add a zero-minute stop

                        let new_stops = new_stop_time_deco.get_stops(self.ascent_rate, self.descent_rate, u.1); // Use next gas on the stops
                        let u2 = DiveSegment::new(SegmentType::DecoStop,
                                                  u.0.get_start_depth(), u.0.get_end_depth(),
                                                  new_stops[1].get_time(), self.ascent_rate, self.descent_rate).unwrap(); // Use the second segment (first is AscDesc)

                        // We do not push any stops or add bottom time here because function will do so already.
                        self.level_to_level(&u2, end_segment,
                                       u.1, stops_performed) // Recursively call level_to_level with the new start segment as u
                    }
                    None => { // There are deco stops to perform but no gas switches necessary.
                        for x in t {
                            stops_performed.push((x, *start_gas));
                        }
                    }
                }
            }
            None => {} // There are no deco stops to perform.
        }
    }
}

impl<'a, T: DecoAlgorithm + Copy + Clone> Dive for OpenCircuit<'a, T> {
    fn plan_dive(&mut self) -> Vec<(DiveSegment, Gas)> {
        let mut total_segments: Vec<(DiveSegment, Gas)> = Vec::new();
        if self.bottom_segments.len() != 1 { // If this is a multi-level dive then use a sliding window.
            let windowed_segments = self.bottom_segments.windows(2);
            for win in windowed_segments {
                let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
                let start = win[0];
                let end = win[1];

                self.deco_algorithm.add_bottom_time(&start.0, &start.1);
                total_segments.push(start);
                self.level_to_level(&start.0, Some(&end.0), &start.1, &mut stops_performed);
                total_segments.append(&mut stops_performed);
            }
        }
        // However the sliding window does not capture the final element. Convenient!
        let final_stop = self.bottom_segments.last().unwrap();
        self.deco_algorithm.add_bottom_time(&final_stop.0, &final_stop.1);
        total_segments.push(*final_stop);
        let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
        self.level_to_level(&final_stop.0, None, &final_stop.1, &mut stops_performed);
        total_segments.append(&mut stops_performed);
        total_segments
    }
}
