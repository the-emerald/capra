use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::{Gas, partial_pressure};
use crate::common::deco_algorithm::DecoAlgorithm;
use std::fmt::Debug;

pub const PPO2_MINIMUM: f64 = 0.18;
pub const PPO2_MAXIMUM_DIVE: f64 = 1.4;
pub const PPO2_MAXIMUM_DECO: f64 = 1.6;

fn equivalent_narcotic_depth(depth: usize, gas: &Gas) -> usize { // Returns a depth
    (((depth + 10) as f64 * (1.0 - gas.fr_he())) - 10.0) as usize
}

fn gas_in_ppo2_range(depth: usize, min: f64, max: f64, gas: &Gas) -> bool { // Checks if gas is in ppo2 range
    let gas_ppo2 = partial_pressure(depth, gas.fr_o2());
    gas_ppo2 >= min && gas_ppo2 <= max
}

fn determine_gas_switch<'a>(segments: &Vec<DiveSegment>, current_gas: &Gas, gases: &'a Vec<(Gas, Option<usize>)>) -> Option<(DiveSegment, &'a Gas)> {
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

        if candidate_gases.len() == 0 { // there no fitting candidate gases.
            continue;
        }

        if candidate_gases[candidate_gases.len()-1] != current_gas {
            return Some((*stop, &candidate_gases[candidate_gases.len()-1]))
        }
    }
    return None;
}

// Start at a level (gas = a)
// Calculate all stops
// If there are stops where gas switch is required
// - Rewind to beginning of level
// - Add stops until the gas switch with gas=a
// - Add the switch stop with gas=b
// - a = b
// - go to start of level

fn level_to_level<T: DecoAlgorithm + Copy + Clone + Debug>(deco: T, start_segment: &DiveSegment,
                                                   end_segment: Option<&DiveSegment>,
                                                   start_gas: &Gas,
                                                   gases: &Vec<(Gas, Option<usize>)>,
                                                   stops_performed: &mut Vec<(DiveSegment, Gas)>,
                                                   ascent_rate: isize, descent_rate: isize) -> T {
    // Returns the deco model AFTER operations are done.

    match end_segment {
        Some(t) => {
            if start_segment.get_end_depth() == t.get_end_depth() { // Base case for recursion
                return deco;
            }
        }
        None => {}
    }
    let mut virtual_deco = deco.clone();
    let intermediate_stops = match end_segment {
        Some(t) => {
            let zero_to_t_segment = DiveSegment::new(SegmentType::AscDesc,
                                                     t.get_start_depth(), t.get_end_depth(),
                                                     0, ascent_rate, descent_rate).unwrap();
            virtual_deco.add_bottom_time(&zero_to_t_segment, start_gas)
        }, // More stops: add the next bottom.
        None => { // Next "stop" is a surface:
            let s = virtual_deco.get_stops(start_segment.get_ascent_rate(), start_segment.get_descent_rate(), start_gas);
            match s[0].get_segment_type() {
                SegmentType::NoDeco => {
                    return virtual_deco
                },
                _ => Some(s)
            }
        }
    };
    match intermediate_stops {
        Some(t) => { // There are deco stops to perform.
            let switch = determine_gas_switch(&t, start_gas, &gases);
            match switch {
                Some(u) => { // There are gas switches to perform. u = target stop
                    virtual_deco = deco.clone(); // Rewind to beginning of level
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

                    let mut new_stop_time_deco = virtual_deco.clone(); // Calculate the new stop time
                    let test_segment = DiveSegment::new(SegmentType::DiveSegment,
                                                        u.0.get_start_depth(), u.0.get_end_depth(),
                                                        0, -ascent_rate, descent_rate).unwrap();
                    new_stop_time_deco.add_bottom_time(&test_segment, start_gas); // Add a zero-minute stop

                    let new_stops = new_stop_time_deco.get_stops(ascent_rate, descent_rate, u.1); // Use next gas on the stops
                    let u2 = DiveSegment::new(SegmentType::DecoStop,
                                              u.0.get_start_depth(), u.0.get_end_depth(),
                                              new_stops[1].get_time(), ascent_rate, descent_rate).unwrap(); // Use the second segment (first is AscDesc)

                    // We do not push any stops or add bottom time here because function will do so already.
                    level_to_level(virtual_deco, &u2, end_segment,
                                   u.1, gases, stops_performed, ascent_rate, descent_rate) // Recursively call level_to_level with the new start segment as u
                }
                None => { // There are deco stops to perform but no gas switches necessary.
                    for x in t {
                        stops_performed.push((x, *start_gas));
                    }
                    return virtual_deco
                }
            }
        }
        None => {
            return virtual_deco
        } // There are no deco stops to perform.
    }
}

pub fn plan_dive<T: DecoAlgorithm + Copy + Clone + Debug>(mut deco: T, bottom_segments: &Vec<(DiveSegment, Gas)>,
                                   deco_gases: &Vec<(Gas, Option<usize>)>, ascent_rate: isize, descent_rate: isize) -> Vec<(DiveSegment, Gas)> {

    let mut total_segs: Vec<(DiveSegment, Gas)> = Vec::new();
    if bottom_segments.len() != 1 { // If this is a multi-level dive then use a sliding window.
        let windowed_segments = bottom_segments.windows(2);
        for win in windowed_segments {
            let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
            let start = win[0];
            let end = win[1];
            deco.add_bottom_time(&start.0, &start.1);
            total_segs.push(start);

            deco = level_to_level(deco, &start.0, Some(&end.0), &start.1, deco_gases, &mut stops_performed, ascent_rate, descent_rate);
            total_segs.append(&mut stops_performed);
        }
    }
    // However the sliding window does not capture the final element. Convenient!
    let final_stop = bottom_segments.last().unwrap();
    deco.add_bottom_time(&final_stop.0, &final_stop.1);
    total_segs.push(*final_stop);
    let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
    level_to_level(deco, &final_stop.0, None, &final_stop.1, deco_gases, &mut stops_performed, ascent_rate, descent_rate);
    total_segs.append(&mut stops_performed);

    return total_segs
}