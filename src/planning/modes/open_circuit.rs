use capra_core::common::dive_segment::SegmentType::{AscDesc, DecoStop};
use capra_core::common::dive_segment::{DiveSegment, SegmentType};
use capra_core::common::gas::Gas;
use capra_core::common::tank::Tank;
use capra_core::common::time_taken;
use capra_core::deco::deco_algorithm::DecoAlgorithm;
use crate::planning::dive_result::DiveResult;
use crate::planning::{DivePlan, PPO2_MAXIMUM_DECO, PPO2_MINIMUM};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;
use time::Duration;

/// An open circuit dive plan.
#[derive(Copy, Clone, Debug)]
pub struct OpenCircuit<'a, T: DecoAlgorithm> {
    /// Decompression algorithm to use
    deco_algorithm: T,
    /// Deco gases to use, and an optional corresponding Maximum Operating Depth
    deco_gases: &'a [(Gas, Option<usize>)],
    /// Bottom segments of the dive and corresponding gas used
    bottom_segments: &'a [(DiveSegment, Gas)],

    /// Ascent rate
    ascent_rate: isize,
    /// Descent rate
    descent_rate: isize,
    /// Depth of water required to induce 1 bar of pressure.
    metres_per_bar: f64,

    /// Surface Air Consumption rate for bottom segments (measured in bar min^-1)
    sac_bottom: usize,
    /// Surface Air Consumption rate for bottom segments (measured in bar min^-1)
    sac_deco: usize,
}

impl<'a, T: DecoAlgorithm> OpenCircuit<'a, T> {
    /// Returns a open circuit dive plan with the given parameters.
    /// # Arguments
    /// * `deco_algorithm` - Decompression algorithm to use
    /// * `deco_gases` - Deco gases to use, and an optional corresponding Maximum Operating Depth
    /// * `bottom_segments` - Bottom segments of the dive and corresponding gas used
    /// * `ascent_rate` - Ascent rate to use throughout the dive
    /// * `descent_rate` - Descent rate to use throughout the dive
    /// * `water_density` - Density of water during the dive (measured in kg m^-3)
    /// * `sac_bottom` - Surface Air Consumption on bottom segments (measured in bar min^-1)
    /// * `sac_deco` - Surface Air Consumption during decompression (measured in bar min^-1)
    pub fn new(
        deco_algorithm: T,
        deco_gases: &'a [(Gas, Option<usize>)],
        bottom_segments: &'a [(DiveSegment, Gas)],
        ascent_rate: isize,
        descent_rate: isize,
        water_density: f64,
        sac_bottom: usize,
        sac_deco: usize,
    ) -> Self {
        OpenCircuit {
            deco_algorithm,
            deco_gases,
            bottom_segments,
            ascent_rate,
            descent_rate,
            metres_per_bar: 10000.0 / water_density,
            sac_bottom,
            sac_deco,
        }
    }

    /// Returns gases that are suitable for use at a dive segment
    fn filter_gases<'b>(
        segment: &DiveSegment,
        gases: &'b [(Gas, Option<usize>)],
        metres_per_bar: f64,
    ) -> Vec<&'b Gas> {
        let mut candidates = gases
            .iter()
            .filter(|x| x.1.map_or(true, |t| t >= segment.start_depth()))
            .map(|x| &x.0)
            .collect::<Vec<&Gas>>();

        candidates = candidates
            .into_iter()
            .filter(|a| a.in_ppo2_range(segment.start_depth(), PPO2_MINIMUM, PPO2_MAXIMUM_DECO))
            .collect(); // filter gases not in ppo2 range

        candidates = candidates
            .into_iter()
            .filter(|a| a.equivalent_narcotic_depth(segment.start_depth()) <= segment.start_depth())
            .collect(); // filter gases over E.N.D.

        candidates.sort_by(|a, b| {
            a.pp_o2(segment.start_depth(), metres_per_bar)
                .partial_cmp(&b.pp_o2(segment.start_depth(), metres_per_bar))
                .unwrap()
        }); // sort by descending order of ppo2

        candidates
    }

    /// Return a gas switch point (if exists) in a list of dive segments.
    fn find_gas_switch_point<'c>(
        segments: &'c [DiveSegment],
        current_gas: &Gas,
        gases: &'c [(Gas, Option<usize>)],
        metres_per_bar: f64,
    ) -> Option<(&'c DiveSegment, &'c Gas)> {
        // Best gasplan is the gasplan that has the highest ppO2 (not over max allowed), and not over equivalent_narcotic_depth.
        for stop in segments.iter().filter(|x| x.segment_type() != AscDesc) {
            let candidate_gases = <OpenCircuit<'a, T>>::filter_gases(stop, gases, metres_per_bar);
            if candidate_gases.is_empty() {
                // there no fitting candidate gases.
                continue;
            }
            if candidate_gases[candidate_gases.len() - 1] != current_gas {
                return Some((stop, &candidate_gases[candidate_gases.len() - 1]));
            }
        }
        None
    }

    /// Perform an open circuit dive plan from the `start` segment to the `end` segment.
    /// The `end` segment is set to `None` if the end is the surface.
    pub(crate) fn level_to_level(
        &self,
        mut deco: T,
        start: &(DiveSegment, Gas),
        end: Option<&(DiveSegment, Gas)>,
        stops_performed: &mut Vec<(DiveSegment, Gas)>,
    ) -> T {
        // Check if there is any depth change
        if let Some(t) = end {
            match start.0.end_depth().cmp(&t.0.start_depth()) {
                Ordering::Less => {
                    // Create a segment with the next segment's gas
                    let descent = DiveSegment::new(
                        SegmentType::AscDesc,
                        start.0.end_depth(),
                        t.0.start_depth(),
                        time_taken(self.descent_rate, start.0.end_depth(), t.0.start_depth()),
                        self.ascent_rate,
                        self.descent_rate,
                    )
                    .unwrap();
                    deco.add_dive_segment(&descent, &start.1, self.metres_per_bar);
                    stops_performed.push((descent, start.1));
                    return deco;
                }
                Ordering::Equal => {
                    // There cannot be any more segments to add.
                    return deco;
                }
                Ordering::Greater => {} // Continue to main algorithm
            }
        }

        let mut virtual_deco = deco;
        // Find the stops between start and end using start gas
        let end_depth = match end {
            Some(t) => t.0.start_depth(),
            None => 0,
        };
        let stops = virtual_deco
            .surface(
                self.ascent_rate,
                self.descent_rate,
                &start.1,
                self.metres_per_bar,
            )
            .into_iter()
            .take_while(|x| x.start_depth() > end_depth)
            .collect::<Vec<DiveSegment>>();

        let switch_gases: Vec<(Gas, Option<usize>)> = match end {
            Some(t) => vec![(start.1, None), (t.1, None)],
            None => self.deco_gases.to_vec(),
        };

        let switch_point = <OpenCircuit<'a, T>>::find_gas_switch_point(
            &stops,
            &start.1,
            &switch_gases,
            self.metres_per_bar,
        );

        // If there are deco stops in between
        if stops.iter().any(|x| x.segment_type() == DecoStop) && switch_point.is_some() {
            let switch = switch_point.unwrap();
            // Rewind the algorithm
            virtual_deco = deco;

            // Replay between stops until gas switch point
            for stop in stops
                .iter()
                .take_while(|x| x.start_depth() > switch.0.start_depth())
            {
                virtual_deco.add_dive_segment(&stop, &start.1, self.metres_per_bar);
                stops_performed.push((*stop, start.1));
            }

            // At gas switch point, use new gas and calculate new deco schedule
            let new_stop = match virtual_deco
                .get_stops(
                    self.ascent_rate,
                    self.descent_rate,
                    &switch.1,
                    self.metres_per_bar,
                )
                .into_iter()
                .find(|x| x.segment_type() == DecoStop && x.start_depth() == switch.0.start_depth())
            {
                Some(t) => t,
                None => DiveSegment::new(
                    SegmentType::DecoStop,
                    switch.0.start_depth(),
                    switch.0.end_depth(),
                    Duration::minute(),
                    self.ascent_rate,
                    self.descent_rate,
                )
                .unwrap(),
            };

            virtual_deco.add_dive_segment(&new_stop, switch.1, self.metres_per_bar);
            stops_performed.push((new_stop, *switch.1));

            // Call recursively with first new gas stop as start, end same
            deco = virtual_deco;
            self.level_to_level(deco, &(new_stop, *switch.1), end, stops_performed)
        } else {
            // Push segments and return
            stops_performed.append(&mut stops.into_iter().zip(iter::repeat(start.1)).collect());
            deco = virtual_deco;
            deco
        }
    }
}

impl<'a, T: DecoAlgorithm> DivePlan<T> for OpenCircuit<'a, T> {
    fn plan(&self) -> DiveResult<T> {
        let mut total_segments: Vec<(DiveSegment, Gas)> = Vec::new();
        let mut deco = self.deco_algorithm;

        // Create the AscDesc to the first segment
        let descent_to_beginning = DiveSegment::new(
            AscDesc,
            0,
            self.bottom_segments[0].0.start_depth(),
            time_taken(
                self.descent_rate,
                0,
                self.bottom_segments[0].0.start_depth(),
            ),
            self.ascent_rate,
            self.descent_rate,
        )
        .unwrap();

        deco.add_dive_segment(
            &descent_to_beginning,
            &self.bottom_segments[0].1,
            self.metres_per_bar,
        );
        total_segments.push((descent_to_beginning, self.bottom_segments[0].1));

        for win in self.bottom_segments.windows(2) {
            let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
            let start = win[0];
            let end = win[1];

            deco.add_dive_segment(&start.0, &start.1, self.metres_per_bar);
            total_segments.push(start);

            deco = self.level_to_level(deco, &start, Some(&end), &mut stops_performed);
            total_segments.append(&mut stops_performed);
        }

        // However the sliding window does not capture the final element.
        let final_stop = self.bottom_segments.last().unwrap();
        deco.add_dive_segment(&final_stop.0, &final_stop.1, self.metres_per_bar);
        total_segments.push(*final_stop);

        let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
        deco = self.level_to_level(deco, &final_stop, None, &mut stops_performed);
        total_segments.append(&mut stops_performed);

        // Gas planning
        let mut gas_plan: HashMap<Gas, usize> = HashMap::new();
        for (segment, gas) in &total_segments {
            let gas_consumed = match segment.segment_type() {
                SegmentType::DecoStop => segment.gas_consumed(self.sac_deco, self.metres_per_bar),
                SegmentType::NoDeco => 0, // No deco segments aren't actually segments
                _ => segment.gas_consumed(self.sac_bottom, self.metres_per_bar),
            };
            let gas_needed = *(gas_plan.entry(*gas).or_insert(0)) + gas_consumed;
            gas_plan.insert(*gas, gas_needed);
        }

        DiveResult::new(deco, total_segments, gas_plan)
    }

    fn plan_backwards(&self, _tanks: &[Tank]) -> DiveResult<T> {
        unimplemented!()
    }
}
