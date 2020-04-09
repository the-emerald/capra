use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::{partial_pressure, Gas};

// TODO: Refactor OTU calculations!
pub fn calculate_otu(segments: &[(DiveSegment, Gas)]) -> f64 {
    let mut otu= 0.0;
    for (segment, gas) in segments {
        match segment.get_segment_type() {
            SegmentType::AscDesc => {
                otu += ascent_descent_constant(segment.get_time().whole_minutes() as usize,
                                               partial_pressure(segment.get_start_depth(),
                                                                gas.fr_o2()),
                                               partial_pressure(segment.get_start_depth(),
                                                                gas.fr_o2()));
            },
            _ => {
                otu += constant_depth(segment.get_time().whole_minutes() as usize,
                                      partial_pressure(segment.get_end_depth(), gas.fr_o2()));
            }
        }
    }
    otu
}

fn constant_depth(time: usize, p_o2: f64) -> f64 {
    (time as f64) * (0.5 / (p_o2 - 0.5)).powf(-5.0/6.0)
}

fn ascent_descent_constant(time: usize, p_o2i: f64, p_o2f: f64) -> f64 {
    ((3.0 / 11.0) * (time as f64) / (p_o2f - p_o2i)) * (((p_o2f - 0.5) / 0.5).powf(11.0 / 6.0)
        - ((p_o2i - 0.5) / 0.5).powf(11.0 / 6.0))
}