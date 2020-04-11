use crate::common::dive_segment::SegmentType::AscDesc;
use crate::common::dive_segment::DiveSegmentError::IncorrectSegmentTypeError;
use time::Duration;

#[derive(Debug)]
pub enum DiveSegmentError {
    IncorrectSegmentTypeError
}

impl std::error::Error for DiveSegmentError {}

impl std::fmt::Display for DiveSegmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
           -> std::fmt::Result {
        match self {
            DiveSegmentError::IncorrectSegmentTypeError => write!(f, "segment type and depths are inconsistent"),
        }
    }
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

    pub fn get_segment_type(&self) -> SegmentType {
        self.segment_type
    }

    pub fn get_start_depth(&self) -> usize {
        self.start_depth
    }

    pub fn get_end_depth(&self) -> usize {
        self.end_depth
    }

    pub fn get_time(&self) -> &Duration {
        &self.time
    }

    pub fn get_ascent_rate(&self) -> isize {
        self.ascent_rate
    }

    pub fn get_descent_rate(&self) -> isize {
        self.descent_rate
    }
}