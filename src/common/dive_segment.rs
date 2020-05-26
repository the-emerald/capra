use crate::common::dive_segment::SegmentType::AscDesc;
use crate::common::dive_segment::DiveSegmentError::IncorrectSegmentTypeError;
use time::Duration;
use crate::common::mtr_bar;

#[derive(thiserror::Error, Debug)]
pub enum DiveSegmentError {
    #[error("segment type and start/end depths are inconsistent")]
    IncorrectSegmentTypeError
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum SegmentType {
    NoDeco, DecoStop, DiveSegment, AscDesc
}

#[derive(Debug, Copy, Clone)]
pub struct DiveSegment {
    segment_type: SegmentType,
    start_depth: usize,
    end_depth: usize,
    time: Duration,
    // Ascent rate and descent rate are for reaching the current DiveSegment from the previous.
    ascent_rate: isize,
    descent_rate: isize,
}

impl DiveSegment {
    pub fn new(segment_type: SegmentType, start_depth: usize, end_depth: usize, time: Duration,
               ascent_rate: isize, descent_rate: isize) -> Result<Self, DiveSegmentError> {

        // Only allow AscDesc segments with a differing start/end depth.
        // As well as any other segment type without a consistent start/end depth,
        match (segment_type, start_depth == end_depth) {
            (AscDesc, true) => return Err(IncorrectSegmentTypeError),
            (AscDesc, false) => {} // Needed to completely match AscDesc
            (_, false) => return Err(IncorrectSegmentTypeError),
            _ => {}
        }

        Ok(Self {
            segment_type,
            start_depth,
            end_depth,
            time,
            ascent_rate,
            descent_rate,
        })
    }

    pub fn segment_type(&self) -> SegmentType {
        self.segment_type
    }

    pub fn start_depth(&self) -> usize {
        self.start_depth
    }

    pub fn end_depth(&self) -> usize {
        self.end_depth
    }

    pub fn time(&self) -> &Duration {
        &self.time
    }

    pub fn ascent_rate(&self) -> isize {
        self.ascent_rate
    }

    pub fn descent_rate(&self) -> isize {
        self.descent_rate
    }

    pub fn gas_consumed(&self, sac_rate: usize, metres_per_bar: f64) -> usize { // Calculate gas consumed given a segment.
        let pressure = match self.segment_type() {
            AscDesc => mtr_bar(((self.end_depth() + self.start_depth()) / 2) as f64, metres_per_bar),
            _ => mtr_bar(self.end_depth() as f64, metres_per_bar)
        };

        (pressure * (self.time().as_seconds_f64()/60.0) * sac_rate as f64) as usize
    }
}