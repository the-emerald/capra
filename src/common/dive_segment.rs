use crate::common::dive_segment::SegmentType::AscDesc;
use crate::common::dive_segment::DiveSegmentError::IncorrectSegmentTypeError;
use time::Duration;
use crate::common::mtr_bar;

/// Represents errors that occur while working with DiveSegments.
#[derive(thiserror::Error, Debug)]
pub enum DiveSegmentError {
    #[error("segment type and start/end depths are inconsistent")]
    /// The SegmentType supplied to create a DiveSegment were inconsistent with its parameters.
    IncorrectSegmentTypeError
}

/// Represents different types of DiveSegments possible.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SegmentType {
    /// Segment represents a no decompression limit.
    NoDeco,
    /// Segment represents a mandatory decompression stop.
    DecoStop,
    /// Segment represents a bottom segment.
    DiveSegment,
    /// Segment represents a change in depth.
    AscDesc
}

/// The atomic unit of a dive. Every dive can be represented by a list of DiveSegments.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DiveSegment {
    /// Type of this segment. See [`SegmentType`].
    segment_type: SegmentType,
    /// Depth at the beginning of segment.
    start_depth: usize,
    /// Depth at the end of segment.
    end_depth: usize,
    /// Duration of the segment.
    time: Duration,
    /// Ascent rate (measured in m min^-1)
    ascent_rate: isize,
    /// Descent rate (measured in m min^-1)
    descent_rate: isize,
}

impl DiveSegment {
    /// Returns a new DiveSegment with the given parameters.
    /// # Arguments
    /// * `segment_type` - Type of this segment. See [`SegmentType`].
    /// * `start_depth`- Depth at the beginning of the segment.
    /// * `end_depth` - Depth at the end of the segment
    /// * `time` - Duration of the segment.
    /// * `ascent_rate` - Ascent rate of the segment (measured in m min^-1)
    /// * `descent_rate` - Descent rate of the segment (measured in m min^-1)
    /// # Errors
    /// This function will return a [`DiveSegmentError`] if any of the following are true:
    /// * `segment-type` is `AscDesc` but start and end depths match.
    /// * `segment-type` is *not* `AscDesc` but start and end depths *do not* match.
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

    /// Returns the type of the segment.
    pub fn segment_type(&self) -> SegmentType {
        self.segment_type
    }

    /// Returns the start depth of the segment.
    pub fn start_depth(&self) -> usize {
        self.start_depth
    }

    /// Returns the end depth of the segment.
    pub fn end_depth(&self) -> usize {
        self.end_depth
    }

    /// Returns the duration of the segment.
    pub fn time(&self) -> &Duration {
        &self.time
    }

    /// Returns the ascent rate of the segment.
    pub fn ascent_rate(&self) -> isize {
        self.ascent_rate
    }

    /// Returns the descent rate of the segment.
    pub fn descent_rate(&self) -> isize {
        self.descent_rate
    }

    /// Returns the quantity of gas a diver would consume in the segment.
    /// # Arguments
    /// * `sac_rate` - Surface Air Consumption (SAC) rate (measured in bar min^-1).
    /// * `metres_per_bar` - The depth of water required to induce 1 bar of pressure.
    pub fn gas_consumed(&self, sac_rate: usize, metres_per_bar: f64) -> usize {
        let pressure = match self.segment_type() {
            AscDesc => mtr_bar(((self.end_depth() + self.start_depth()) / 2) as f64, metres_per_bar),
            _ => mtr_bar(self.end_depth() as f64, metres_per_bar)
        };

        (pressure * (self.time().as_seconds_f64()/60.0) * sac_rate as f64) as usize
    }
}