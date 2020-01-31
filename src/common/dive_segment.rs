#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum SegmentType {
    NoDeco, DecoStop, DiveSegment
}

#[derive(Debug, Copy, Clone)]
pub struct DiveSegment {
    segment_type: SegmentType,
    depth: usize,
    time: usize,
    // Ascent rate and descent rate are for reaching the current DiveSegment from the previous.
    ascent_rate: isize,
    descent_rate: isize,
}

impl DiveSegment {
    pub fn new(segment_type: SegmentType, depth: usize, time: usize, ascent_rate: isize, descent_rate: isize) -> Self {
        Self {
            segment_type,
            depth,
            time,
            ascent_rate,
            descent_rate
        }
    }

    pub fn get_segment_type(&self) -> SegmentType {
        self.segment_type
    }

    pub fn get_depth(&self) -> usize {
        self.depth
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