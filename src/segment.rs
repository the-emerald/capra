use crate::environment::Environment;
use crate::segment::DiveSegmentError::InconsistentDepth;
use crate::units::consumption::GasConsumption;
use crate::units::consumption_rate::GasConsumptionRate;
use crate::units::depth::Depth;
use crate::units::rate::Rate;
use thiserror::Error;
use time::Duration;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq, Hash)]
pub enum DiveSegmentError {
    #[error("segment type inconsistent with start/end depth")]
    InconsistentDepth,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SegmentType {
    NoDeco,
    DecoStop,
    Bottom,
    AscDesc,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Segment {
    segment_type: SegmentType,
    start_depth: Depth,
    end_depth: Depth,
    time: Duration,
    ascent_rate: Rate,
    descent_rate: Rate,
}

impl Segment {
    pub fn new(
        segment_type: SegmentType,
        start_depth: Depth,
        end_depth: Depth,
        time: Duration,
        ascent_rate: Rate,
        descent_rate: Rate,
    ) -> Result<Self, DiveSegmentError> {
        match (segment_type, start_depth == end_depth) {
            (SegmentType::AscDesc, true) => return Err(InconsistentDepth),
            (SegmentType::AscDesc, false) => {}
            (_, false) => return Err(InconsistentDepth),
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

    pub fn start_depth(&self) -> Depth {
        self.start_depth
    }

    pub fn end_depth(&self) -> Depth {
        self.end_depth
    }

    pub fn time(&self) -> &Duration {
        &self.time
    }

    pub fn ascent_rate(&self) -> Rate {
        self.ascent_rate
    }

    pub fn descent_rate(&self) -> Rate {
        self.descent_rate
    }

    pub fn gas_consumed(
        &self,
        consumption_rate: GasConsumptionRate,
        environment: Environment,
    ) -> GasConsumption {
        let pressure = match self.segment_type {
            SegmentType::AscDesc => {
                ((self.start_depth + self.end_depth) / Depth(2)).pressure(environment)
            }
            _ => self.end_depth.pressure(environment),
        };
        GasConsumption(
            (pressure.0 * (self.time.as_seconds_f64() / 60.0) * consumption_rate.0 as f64) as u32,
        )
    }
}
