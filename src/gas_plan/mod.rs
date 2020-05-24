use crate::common::gas::Gas;
use crate::common::dive_segment::DiveSegment;
use crate::gas_plan::tank::Tank;
use crate::dive_plan::dive::Dive;
use crate::deco::deco_algorithm::DecoAlgorithm;
use crate::common::dive_segment::SegmentType::AscDesc;
use crate::common::mtr_bar;
use std::collections::HashMap;

pub mod otu;
pub mod tank;

pub trait GasPlan<U>: Dive<U> where U: DecoAlgorithm {
    fn plan_forwards(&self) -> HashMap<Gas, usize>; // Given a dive profile, how much gas do we need?
    fn plan_backwards(&self, tanks: &[Tank]) -> Vec<(DiveSegment, Gas)>; // Given some amount of gas, how long can we dive?

    fn calculate_consumed(segment: &DiveSegment, sac_rate: usize, metres_per_bar: f64) -> usize { // Calculate gas consumed given a segment.
        let pressure = match segment.segment_type() {
            AscDesc => mtr_bar(((segment.end_depth() + segment.start_depth()) / 2) as f64, metres_per_bar),
            _ => mtr_bar(segment.end_depth() as f64, metres_per_bar)
        };
        (pressure * (segment.time().as_seconds_f64()/60.0) * sac_rate as f64) as usize
    }
}