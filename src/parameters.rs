/// Parameters for an open circuit dive plan.
#[derive(Copy, Clone, Debug)]
pub struct DiveParameters {
    /// Ascent rate to use for planner-generated segments
    pub ascent_rate: isize,
    /// Descent rate to use for planner-generated segments
    pub descent_rate: isize,
    /// Depth of water required to induce 1 bar of pressure.
    pub metres_per_bar: f64,
    /// Surface Air Consumption rate for bottom segments (measured in bar min^-1)
    pub sac_bottom: usize,
    /// Surface Air Consumption rate for bottom segments (measured in bar min^-1)
    pub sac_deco: usize,
}