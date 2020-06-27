use crate::common::dive_segment::{DiveSegment, SegmentType};
use crate::common::gas::Gas;

pub fn otu(segment: &DiveSegment, gas: &Gas) -> f64 {
    match segment.segment_type() {
        SegmentType::AscDesc => {
            ascent_descent_constant(
                segment.time().whole_seconds() as usize,
                 Gas::partial_pressure(segment.start_depth(), gas.fr_o2(), 1000.0),
                 Gas::partial_pressure(segment.end_depth(), gas.fr_o2(), 1000.0),
            )
        }
        _ => {
            constant_depth(
                segment.time().whole_seconds() as usize,
                Gas::partial_pressure(segment.start_depth(), gas.fr_o2(), 1000.0)
            )
        }
    }
}

fn constant_depth(time: usize, p_o2: f64) -> f64 {
    (time as f64) * (0.5 / (p_o2 - 0.5)).powf(-5.0/6.0)
}

fn ascent_descent_constant(time: usize, p_o2i: f64, p_o2f: f64) -> f64 {
    ((3.0 / 11.0) * (time as f64) / (p_o2f - p_o2i)) * (((p_o2f - 0.5) / 0.5).powf(11.0 / 6.0)
        - ((p_o2i - 0.5) / 0.5).powf(11.0 / 6.0))
}