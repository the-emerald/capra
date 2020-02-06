#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum SegmentType {
    NoDeco, DecoStop, DiveSegment, AscDesc
}

#[derive(Debug, Copy, Clone)]
pub struct DiveSegment {
    segment_type: SegmentType,
    start_depth: usize,
    end_depth: usize,
    time: usize,
    // Ascent rate and descent rate are for reaching the current DiveSegment from the previous.
    ascent_rate: isize,
    descent_rate: isize,
}

impl DiveSegment {
    pub fn new(segment_type: SegmentType, start_depth: usize, end_depth: usize, time: usize, ascent_rate: isize, descent_rate: isize) -> Self {
        Self {
            segment_type,
            start_depth,
            end_depth,
            time,
            ascent_rate,
            descent_rate,
        }
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

    pub fn get_time(&self) -> usize {
        self.time
    }

    pub fn get_ascent_rate(&self) -> isize {
        self.ascent_rate
    }

    pub fn get_descent_rate(&self) -> isize {
        self.descent_rate
    }
}