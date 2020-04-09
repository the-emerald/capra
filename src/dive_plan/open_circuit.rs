use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::dive_plan::dive::Dive;
use crate::dive_plan::{gas_in_ppo2_range, equivalent_narcotic_depth, PPO2_MINIMUM, PPO2_MAXIMUM_DECO};
use crate::deco::deco_algorithm::DecoAlgorithm;
use crate::common::gas::{Gas, partial_pressure};
use crate::gas_plan::GasPlan;
use crate::gas_plan::tank::Tank;
use time::Duration;
use crate::common::dive_segment::SegmentType::AscDesc;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct OpenCircuit<'a, T: DecoAlgorithm> {
    deco_algorithm: T,
    deco_gases: &'a [(Gas, Option<usize>)],
    bottom_segments: &'a [(DiveSegment, Gas)],

    ascent_rate: isize,
    descent_rate: isize,

    sac_bottom: usize,
    sac_deco: usize
}

impl<'a, T: DecoAlgorithm> OpenCircuit<'a, T> {
    pub fn new(deco_algorithm: T, deco_gases: &'a [(Gas, Option<usize>)],
               bottom_segments: &'a [(DiveSegment, Gas)], ascent_rate: isize,
               descent_rate: isize, sac_bottom: usize, sac_deco: usize) -> Self {
        OpenCircuit {
            deco_algorithm,
            deco_gases,
            bottom_segments,
            ascent_rate,
            descent_rate,
            sac_bottom,
            sac_deco
        }
    }

    fn filter_gases<'b>(segment: &DiveSegment, gases: &'b [(Gas, Option<usize>)]) -> Vec<&'b Gas> {
        let mut candidates = Vec::new();
        for gas in gases { // Do not push any candidates that are deeper than MOD
            match gas.1 {
                Some(t) => {
                    if t >= segment.get_start_depth() {
                        candidates.push(&gas.0);
                    }
                }
                None => {
                    candidates.push(&gas.0)
                }
            }
        }

        candidates = candidates.iter().filter(|a|
            gas_in_ppo2_range(segment.get_start_depth(), PPO2_MINIMUM, PPO2_MAXIMUM_DECO, a)).cloned().collect(); // filter gases not in ppo2 range

        candidates = candidates.iter().filter(|a|
            equivalent_narcotic_depth(segment.get_start_depth(), a) <= segment.get_start_depth()).cloned().collect(); // filter gases over E.N.D.

        candidates.sort_by(|a, b|
            partial_pressure(segment.get_start_depth(), a.fr_o2())
                .partial_cmp(&partial_pressure(segment.get_start_depth(), b.fr_o2()))
                .unwrap()); // sort by descending order of ppo2

        candidates
    }

    fn find_gas_switch_point(segments: &[DiveSegment], current_gas: &Gas, gases: &'a [(Gas, Option<usize>)]) -> Option<(DiveSegment, &'a Gas)> {
        // Best gas_plan is the gas_plan that has the highest ppO2 (not over max allowed), and not over equivalent_narcotic_depth.
        for stop in segments {
            let candidate_gases = <OpenCircuit<'a, T>>::filter_gases(stop, gases);
            if stop.get_segment_type() == AscDesc {
                println!("CG: stop: {:?} gases: {:?} filtered:{:?}", &stop, &gases, &candidate_gases);
            }

            if candidate_gases.is_empty(){ // there no fitting candidate gases.
                continue;
            }

            if candidate_gases[candidate_gases.len()-1] != current_gas {
                return Some((*stop, &candidate_gases[candidate_gases.len()-1]))
            }
        }
        None
    }

    // fn check_gas_swap<'c>(segments: &[DiveSegment], current_gas: &'c Gas, gases: &'c [(Gas, Option<usize>)]) -> Option<&'c Gas> {
    //     println!("Check");
    //     match <OpenCircuit<'c, T>>::find_gas_switch_point(segments, current_gas, gases) {
    //         Some(t) => { Some(t.1) },
    //         None => None
    //     }
    // }

    pub(crate) fn level_to_level(&mut self, start_segment: &DiveSegment,
                                 end_segment: Option<&DiveSegment>, start_gas: &Gas,
                                 stops_performed: &mut Vec<(DiveSegment, Gas)>) {

        if let Some(t) = end_segment {
            if start_segment.get_end_depth() == t.get_end_depth() { // Check if there is a depth change
                return;
            }
        }

        let mut virtual_deco = self.deco_algorithm;
        let intermediate_stops = match end_segment { // Check if there are intermediate stops
            Some(t) => {
                let zero_to_t_segment = DiveSegment::new(SegmentType::DiveSegment,
                                                         t.get_start_depth(), t.get_end_depth(),
                                                         // time_taken(self.ascent_rate, t.get_end_depth(), t.get_start_depth()),
                                                         Duration::seconds(0),
                                                         self.ascent_rate, self.descent_rate).unwrap(); // TODO: Use actual times
                virtual_deco.add_bottom_time(&zero_to_t_segment, start_gas)
            }, // More stops: add the next bottom.
            None => { // Next "stop" is a surface:
                let s = virtual_deco.surface(start_segment.get_ascent_rate(), start_segment.get_descent_rate(), start_gas);
                match s[0].get_segment_type() {
                    SegmentType::NoDeco => {
                        self.deco_algorithm = virtual_deco;
                        return;
                    },
                    _ => Some(s)
                }
            }
        };
        match intermediate_stops {
            Some(t) => { // There are deco stops to perform.
                // println!("Vec<D> time");
                let switch = <OpenCircuit<'a, T>>::find_gas_switch_point(
                    &t
                        .iter()
                        .filter(|x| x.get_segment_type() != AscDesc)
                        .cloned()
                        .collect::<Vec<DiveSegment>>()
                    ,
                    start_gas, self.deco_gases);
                match switch {
                    Some(u) => { // There are gas_plan switches to perform. u = target stop
                        virtual_deco = self.deco_algorithm; // Rewind to beginning of level
                        // println!("A: {:?}\n{:?}\n", *start_gas, t);

                        // println!("Switch time");
                        for i in t {
                            if i.get_start_depth() == u.0.get_start_depth() { // Replay to stop **before** u
                                break;
                            }

                            if i.get_segment_type() != SegmentType::AscDesc {
                                virtual_deco.add_bottom_time(&i, start_gas);
                            }
                            else {
                                let normalised_segment = DiveSegment::new(SegmentType::DiveSegment,
                                                                          i.get_end_depth(),
                                                                          i.get_end_depth(),
                                                                          Duration::zero(),
                                                                          self.ascent_rate, self.descent_rate).unwrap();
                                virtual_deco.add_bottom_time(&normalised_segment, start_gas);
                            }

                            // println!("! Current gas: {:?}, Seg: {:?}", start_gas, i);
                            // match <OpenCircuit<'a, T>>::check_gas_swap(&[i], start_gas, self.deco_gases) {
                            //     Some(v) => {
                            //         stops_performed.push((i, *v));
                            //         println!("yo!");
                            //     }
                            //     None => {}
                            // }
                            stops_performed.push((i, *start_gas));
                        }
                        // println!("We're leaving\n");

                        let mut new_stop_time_deco = virtual_deco; // Calculate the new stop time
                        // println!("u0: {:?}", u.0);
                        let test_segment = DiveSegment::new(SegmentType::DiveSegment,
                                                            u.0.get_start_depth(), u.0.get_end_depth(),
                                                            Duration::seconds(0), self.ascent_rate, self.descent_rate).unwrap();
                        new_stop_time_deco.add_bottom_time(&test_segment, start_gas); // Add a zero-minute stop

                        let new_stops = new_stop_time_deco.surface(self.ascent_rate, self.descent_rate, u.1); // Use next gas_plan on the stops
                        // println!("newstops: {:?}", &new_stops);
                        let mut force_add = false;
                        let u2_time = match &new_stops[0].get_time().cmp(&Duration::minute()) {
                            Ordering::Less => {
                                force_add = true;
                                Duration::minute()
                            }
                            _ => *{
                                new_stops[0].get_time()
                            }
                        };
                        let u2 = DiveSegment::new(SegmentType::DecoStop,
                                                  u.0.get_start_depth(), u.0.get_end_depth(),
                                                  u2_time, self.ascent_rate, self.descent_rate).unwrap();
                        if force_add {
                            stops_performed.push((u2, *u.1));
                        }

                        // println!("u2: {:?}", u2);
                        // println!("Model: {:?}", &virtual_deco);
                        // We do not push any stops or add bottom time here because function will do so already.
                        self.deco_algorithm = virtual_deco;
                        self.level_to_level(&u2, end_segment,
                                       u.1, stops_performed) // Recursively call level_to_level with the new start segment as u
                    }
                    None => { // There are deco stops to perform but no gas_plan switches necessary.
                        // println!("B: {:?}\n{:?}\n", *start_gas, t);
                        for x in t {
                            // println!("... Current gas: {:?}, Seg: {:?}", start_gas, &x);
                            // match <OpenCircuit<'a, T>>::check_gas_swap(&[x], start_gas, self.deco_gases) {
                            //     Some(u) => {
                            //         stops_performed.push((x, *u));
                            //         println!("yo... {:?}", u);
                            //     }
                            //     None => {stops_performed.push((x, *start_gas));}
                            // }
                            stops_performed.push((x, *start_gas));
                        }
                        self.deco_algorithm = virtual_deco;
                    }
                }
            }
            None => {
               self.deco_algorithm = virtual_deco;
            } // There are no deco stops to perform.
        }
    }
}

impl<'a, T: DecoAlgorithm> Dive<T> for OpenCircuit<'a, T> {
    fn execute_dive(&mut self) -> Vec<(DiveSegment, Gas)> {
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

    fn finish(self) -> T {
        self.deco_algorithm
    }
}

impl<'a, U: Dive<T>, T: DecoAlgorithm> GasPlan<T, U> for OpenCircuit<'a, T> {
    fn plan_forwards(&self) -> Vec<(Gas, usize)> {  // Given a dive profile, how much gas do we need?
        let mut gas_plan: Vec<(Gas, usize)> = Vec::new();

        // Bottom segments
        for (segment, gas) in self.bottom_segments {
            gas_plan.push((
                *gas,
                <Self as GasPlan<T, U>>::calculate_consumed(segment, self.sac_bottom)
            ))
        }

        // Deco segments
        let mut virtual_dive = *self;
        let virtual_deco = virtual_dive.execute_dive();
        for (segment, gas) in virtual_deco {
            // TODO: Add gas consumption calculation here
        }
        gas_plan
    }

    fn plan_backwards(&self, tanks: &[Tank]) -> Vec<(DiveSegment, Gas)> {
        unimplemented!() // TODO: Implement backwards planning
    }
}
