use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::{Gas, partial_pressure};
use crate::common::deco_algorithm::DecoAlgorithm;
use std::fmt::Debug;

const PPO2_MINIMUM: f64 = 0.18;
const PPO2_MAXIMUM_DIVE: f64 = 1.4;
const PPO2_MAXIMUM_DECO: f64 = 1.6;

fn equivalent_narcotic_depth(depth: usize, gas: &Gas) -> usize { // Returns a depth
    (((depth + 10) as f64 * (1.0 - gas.fr_he())) - 10.0) as usize
}

//fn best_gas_for_segment(segment: &DiveSegment, gases: &Vec<Gas>, max_allowed_ppo2: f64) -> &Gas {
//    // Best gas is the gas that has the highest ppO2 (not over max allowed), and not over equivalent_narcotic_depth.
////    let mut sorted = gases.clone();
////    sorted.sort_by(|a, b| (equivalent_narcotic_depth(segment.get_end_depth(), a).cmp(&equivalent_narcotic_depth(segment.get_end_depth(), b))));
////
////
////    return &Gas::new(0.5, 0.5, 0.5).unwrap()
//}
fn gas_in_ppo2_range(depth: usize, min: f64, max: f64, gas: &Gas) -> bool {
    let gas_ppo2 = partial_pressure(depth, gas.fr_o2());
//    println!("{:?} {}", gas, gas_ppo2);
    gas_ppo2 >= min && gas_ppo2 <= max
}

fn determine_gas_switch<'a>(segments: &Vec<DiveSegment>, current_gas: &Gas, gases: &'a Vec<(Gas, Option<usize>)>) -> Option<(DiveSegment, &'a Gas)> {
    for stop in segments {
        if stop.get_segment_type() == SegmentType::AscDesc {
            continue;
        }

//        println!("{:?} stop", stop);
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

//        println!("g {:?}", candidate_gases);

        candidate_gases = candidate_gases.iter().filter(|a|
            equivalent_narcotic_depth(stop.get_end_depth(), a) <= stop.get_end_depth()).cloned().collect(); // filter gases over end

//        println!("g {:?}", candidate_gases);

        candidate_gases.sort_by(|a, b|
            partial_pressure(stop.get_end_depth(), a.fr_o2())
                .partial_cmp(&partial_pressure(stop.get_end_depth(), b.fr_o2()))
                .unwrap()); // sort by descending order of ppo2

//        println!("g {:?}", candidate_gases);

        if candidate_gases.len() == 0 {
            continue;
        }

        if candidate_gases[candidate_gases.len()-1] != current_gas {
            return Some(
                (*stop, &candidate_gases[candidate_gases.len()-1])
            )
        }
    }
    return None;
}

// Start at a level (gas = a)
// Calculate all stops
// If there are stops where gas switch is required
// Rewind to beginning of level
// Add stops until the gas switch with gas=a
// Add the switch stop with gas=b
// a = b
// go to start of level

fn level_to_level<T: DecoAlgorithm + Copy + Clone>(deco: T, start_segment: &DiveSegment, end_segment: Option<&DiveSegment>,
                                                   start_gas: &Gas, gases: &Vec<(Gas, Option<usize>)>, stops_performed: &mut Vec<(DiveSegment, Gas)>) -> T {
    // Returns the deco model AFTER operations are done.
    match end_segment {
        Some(t) => {
            if start_segment.get_end_depth() == t.get_end_depth() {
                return deco;
            }
        }
        None => {}
    }
    let mut virtual_deco = deco.clone();
    let intermediate_stops = match end_segment {
        Some(t) => virtual_deco.add_bottom_time(t, start_gas), // More stops: add the next bottom.
        None => { // Surface:
            virtual_deco.add_bottom_time(start_segment, start_gas);
            Some(virtual_deco.get_stops(start_segment.get_ascent_rate(), start_segment.get_descent_rate(), start_gas))
        }
    };
//    println!("{:?} intermediate", intermediate_stops);
    match intermediate_stops {
        Some(t) => { // There are deco stops to perform.
//            println!("STOP");
            let switch = determine_gas_switch(&t, start_gas, &gases);
            match switch {
                Some(u) => { // There are gas switches to perform. u = target stop
//                    println!("SWITCH");
                    virtual_deco = deco.clone(); // Rewind to beginning of level
                    for i in t {
                        if i.get_segment_type() == SegmentType::AscDesc {
                            continue;
                        }

                        if i.get_end_depth() == u.0.get_end_depth() { // Replay to stop before u
                            break;
                        }
                        virtual_deco.add_bottom_time(&i, start_gas);
                        stops_performed.push((i, *start_gas));
                    }
                    virtual_deco.add_bottom_time(&u.0, u.1); // Add u with gas switch.1
                    stops_performed.push((u.0, *u.1));
                    // Recursively call level_to_level with the new start segment as u
                    level_to_level(virtual_deco, &u.0, end_segment, u.1, gases, stops_performed)
                }
                None => { // There are deco stops to perform but no gas switches necessary.
//                    println!("NO SWITCH");
                    for x in t {
                        stops_performed.push((x, *start_gas));
                    }
                    return virtual_deco
                }
            }
        }
        None => return deco // There are no deco stops to perform.
    }
}

pub fn plan_dive<T: DecoAlgorithm + Copy + Clone + Debug>(mut deco: T, bottom_segments: &Vec<(DiveSegment, Gas)>,
                                   deco_gases: &Vec<(Gas, Option<usize>)>) -> Vec<(DiveSegment, Gas)> {

    let mut total_segs: Vec<(DiveSegment, Gas)> = Vec::new();
    deco.add_bottom_time(&bottom_segments[0].0, &bottom_segments[0].1);

    if bottom_segments.len() != 1 { // If this is a multi-level dive then use a sliding window.
        let windowed_segments = bottom_segments.windows(2);
        for win in windowed_segments {
            let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
            let start = win[0];
            let end = win[1];
//            println!("{:?} {:?} window", start.0, end.0);
            deco = level_to_level(deco, &start.0, Some(&end.0), &start.1, deco_gases, &mut stops_performed);
//            println!("{:?}", deco);
            total_segs.push(start);
            total_segs.append(&mut stops_performed);
        }
    }
//    println!("{:?} segs before final", total_segs);
//    println!("{:?} deco immediately?", deco.get_stops(-10, 20, &bottom_segments[1].1));
    // However the sliding window does not capture the final element. Convenient!
//    println!("Final");
    let final_stop = bottom_segments.last().unwrap();
    let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
    deco = level_to_level(deco, &final_stop.0, None, &final_stop.1, deco_gases, &mut stops_performed);
    total_segs.push(*final_stop);
    total_segs.append(&mut stops_performed);

    return total_segs
}