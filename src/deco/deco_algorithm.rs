use crate::common::dive_segment;
use crate::common::dive_segment::DiveSegment;
use crate::common::gas::Gas;
use crate::deco::tissue::Tissue;
use crate::common::water_density::WaterDensity;

/// Trait for decompression models. This trait must be implemented for any custom decompression
/// algorithms if they are to be used in dive plans with the [`DivePlan`] trait.
pub trait DecoAlgorithm: Copy {
    /// Apply a segment to the deco model.
    /// # Arguments
    /// * `segment` - DiveSegment to apply.
    /// * `gas` - Gas that is being consumed in this segment.
    /// * `metres_per_bar` - Depth of water required to induce 1 bar of pressure.
    fn add_dive_segment(&mut self, segment: &DiveSegment, gas: &Gas, density: WaterDensity);

    /// Surface the deco model, returning the mandatory decompression stops / remaining no-decompression
    /// time along the way.
    /// # Arguments
    /// * `ascent_rate` - Ascent rate to use during stops
    /// * `descent_rate` - Ascent rate to use during stops
    fn surface(
        &mut self,
        ascent_rate: isize,
        descent_rate: isize,
        gas: &Gas,
        density: WaterDensity,
    ) -> Vec<dive_segment::DiveSegment>;

    /// Get the tissue loadings of the model
    fn get_tissue(&self) -> Tissue;

    /// Get the decompression stops required to surface the model. This is identical to `surface`
    /// but it does not modify the original model in any way.
    fn get_stops(
        &self,
        ascent_rate: isize,
        descent_rate: isize,
        gas: &Gas,
        metres_per_bar: f64,
    ) -> Vec<dive_segment::DiveSegment> {
        let mut virtual_deco = *self;
        virtual_deco.surface(ascent_rate, descent_rate, gas, metres_per_bar)
    }
}
