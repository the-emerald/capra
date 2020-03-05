use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::Gas;
use crate::common::deco_algorithm::DecoAlgorithm;

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

fn determine_gas_switch<'a>(segments: &Vec<DiveSegment>, current_gas: &Gas, gases: &'a Vec<(Gas, Option<usize>)>) -> Option((DiveSegment, &'a Gas)) {
    let mut eligible_gases: Vec<Gas> = Vec::new();
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
        Some(t) => virtual_deco.add_bottom_time(t, start_gas),
        None => Some(virtual_deco.get_stops(start_segment.get_ascent_rate(), start_segment.get_descent_rate(), start_gas))
    };
    match intermediate_stops {
        Some(t) => { // There are deco stops to perform.
            let switch = determine_gas_switch(&t, start_gas, &gases);
            match switch.0 {
                Some(u) => { // There are gas switches to perform. u = target stop
                    virtual_deco = deco.clone(); // Rewind to beginning of level
                    for i in t {
                        if i.get_end_depth() == u.get_end_depth() { // Replay to stop before u
                            break;
                        }
                        virtual_deco.add_bottom_time(&i, start_gas);
                        stops_performed.push((i, *start_gas));
                    }
                    virtual_deco.add_bottom_time(&u, switch.1); // Add u with gas switch.1
                    stops_performed.push((u, *switch.1));
                    // Recursively call level_to_level with the new start segment as u
                    level_to_level(virtual_deco, &u, end_segment, switch.1, gases, stops_performed)
                }
                None => { // There are deco stops to perform but no gas switches necessary.
                    for x in t {
                        stops_performed.push((x, *start_gas));
                    }
                    return virtual_deco
                }
            }
        }
        None => return virtual_deco // There are no deco stops to perform.
    }
}

pub fn plan_dive<T: DecoAlgorithm + Copy + Clone>(mut deco: T, bottom_segments: &Vec<(DiveSegment, Gas)>,
                                   deco_gases: &Vec<(Gas, Option<usize>)>) -> Vec<(DiveSegment, Gas)> {

    let mut total_segs: Vec<(DiveSegment, Gas)> = Vec::new();

    if bottom_segments.len() != 1 { // If this is a multi-level dive then use a sliding window.
        let windowed_segments = bottom_segments.windows(2);
        for win in windowed_segments {
            let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
            let start = win[0];
            let end = win[1];
            deco = level_to_level(deco, &start.0, Some(&end.0), &start.1, deco_gases, &mut stops_performed);
            total_segs.push(start);
            total_segs.append(&mut stops_performed);
        }
    }
    // However the sliding window does not capture the final element. Convenient!
    let surface = DiveSegment::new(SegmentType::DiveSegment, 0, 0, 0, 0, 0).unwrap();
    // TODO: Handle surfacing.
    let final_stop = bottom_segments.last().unwrap();
    let mut stops_performed: Vec<(DiveSegment, Gas)> = Vec::new();
    deco = level_to_level(deco, &final_stop.0, None, &final_stop.1, deco_gases, &mut stops_performed);
    total_segs.push(*final_stop);
    total_segs.append(&mut stops_performed);

    return total_segs
}